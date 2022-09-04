{ root
, checkers
, lib
, runCommand
, writeShellScript
, coreutils
, linkFarm
, writeShellScriptBin
}:

let
  resolvedCheckers = (builtins.concatLists
    (builtins.attrValues
      (builtins.mapAttrs
        (name:
          { checker
          , files ? [ ]
          , config ? { }
          }:
          (builtins.map
            (file:
              let
                configString = ''
                  ${lib.optionalString (checker ? configFormat) ''
                    export config=${checker.configFormat.generate "config" config}
                  ''}
                '';
              in
              {
                inherit name;
                inherit file;

                check = runCommand "${name}-${builtins.baseNameOf file}"
                  {
                    nativeBuildInputs = checker.packages or [ ];
                  }
                  ''
                    export path=${root + "/${file}"}
                    ${configString}
                    ${checker.check}
                    if [ $? -eq 0 ]; then
                      touch "$out"
                    fi
                  '';

                fix = lib.optionalAttrs (checker ? fix) writeShellScript "${name}-fix" ''
                  export PATH=${lib.makeBinPath ([ coreutils ] ++ checker.packages or [])}
                  ${configString}
                  ${checker.fix}
                '';
              })
            files))
        checkers)));
in
{
  check = linkFarm "flake-file-checker"
    (builtins.map
      ({ name, file, check, ... }: {
        name = "${name}/${file}";
        path = check;
      })
      resolvedCheckers);

  fix = writeShellScriptBin "fix" ''
    while [ ! -f flake.nix ]; do
      if [ $PWD == / ]; then
        echo "Couldn't find flake.nix"
        exit 1
      fi

      cd ..
    done

    ${builtins.concatStringsSep ""
      (builtins.concatMap
        ({ file, check, fix, ... }:
          if fix != null
          then [
            ''
              if ! [ -e ${builtins.unsafeDiscardStringContext check} ]; then
                path=${root + "/${file}"} out=${file} ${fix}
              fi
            ''
          ]
          else []
        )
        resolvedCheckers)}

    nix flake check &
  '';
}
