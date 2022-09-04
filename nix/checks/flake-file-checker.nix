{ root
, settings
, extraCheckers ? { }

, lib
, stdenv
, callPackage
, runCommand
, writeShellScript
, coreutils
}:

let
  checkers = {
    markdownlint = callPackage ./markdownlint.nix { };
    nixpkgs-fmt = callPackage ./nixpkgs-fmt.nix { };
    prettier = callPackage ./prettier.nix { };
    rustfmt = callPackage ./rustfmt.nix { };
  } // extraCheckers;

  packages = builtins.concatMap
    (name: checkers.${name}.packages or [ ])
    (builtins.attrNames settings);

  compiledCheckers = builtins.concatLists
    (builtins.attrValues
      (builtins.mapAttrs
        (name:
          { files ? [ ]
          , config ? { }
          }:
          let
            checker = checkers.${name};

            packages = checker.packages or [ ];

            configExport = lib.optionalString (checker ? configFormat) ''
              export config=${checker.configFormat.generate "config" config}
            '';

            fix = lib.optionalAttrs (checker ? fix) writeShellScript "${name}-fix" ''
              export PATH=${lib.makeBinPath packages}
              ${configExport}
              ${checker.fix}
            '';
          in
          (builtins.map
            (file: {
              inherit file fix;

              check = runCommand "${name}-${builtins.baseNameOf file}"
                {
                  nativeBuildInputs = packages;
                }
                ''
                  export path=${root + "/${file}"}
                  ${configExport}
                  ${checker.check}
                  if [ $? -eq 0 ]; then
                    touch "$out"
                  fi
                '';
            })
            files))
        settings));

  check = derivation {
    system = stdenv.buildPlatform.system;
    name = "flake-file-checker";
    nativeBuildInputs = builtins.map ({ check, ... }: check) compiledCheckers;
    builder = "${coreutils}/bin/touch";
    args = [ (placeholder "out") ];
  };

  raw-fix = writeShellScript "raw-fix" ''
    while [ ! -f flake.nix ]; do
      if [ $PWD == / ]; then
        echo "Couldn't find flake.nix"
        exit 1
      fi

      cd ..
    done

    ${builtins.concatStringsSep ""
      (builtins.concatMap
        ({ file, check, fix }:
          if fix != null
          then [
            ''
              if ! [ -e ${builtins.unsafeDiscardStringContext check} ]; then
                path=${file} ${fix}
              fi
            ''
          ]
          else []
        )
        compiledCheckers)}
  '';
in
{
  inherit packages check;

  fix = {
    type = "app";
    program = toString (writeShellScript "fix" ''
      ${raw-fix}
      nix-build ${builtins.unsafeDiscardStringContext check} &
    '');
  };

  fix-check = {
    type = "app";
    program = toString (writeShellScript "fix-check" ''
      ${raw-fix}
      nix flake check
    '');
  };
}
