{ lib
, craneLib
}:

let
  root = ../..;

  src = builtins.path {
    path = root;
    name = "source";
    filter = absolutePath: type:
      let
        path = lib.removePrefix "${builtins.toString root}/" absolutePath;
      in
      lib.hasPrefix "src" path || lib.hasPrefix "tests" path ||
      type != "directory" && (path == "Cargo.toml" || path == "Cargo.lock");
  };

  buildSrc = builtins.path {
    path = src;
    name = "build-source";
    filter = path: type:
      !(lib.hasPrefix "${builtins.toString src}/tests" path);
  };

  cargoArtifacts = craneLib.buildDepsOnly {
    src = buildSrc;
  };
in
craneLib.buildPackage {
  pname = "ari";
  version = "0.1";

  src = buildSrc;
  inherit cargoArtifacts;

  doCheck = false;

  passthru.checks = {
    clippy = craneLib.cargoClippy {
      inherit src cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    };

    coverage = craneLib.cargoTarpaulin {
      inherit src cargoArtifacts;
    };
  };

  meta = with lib; {
    description = "A type-centred purely functional programming language designed to type binary files";
    homepage = "https://gitlab.com/ari-lang/ari";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [ kira-bruneau ];
  };
}
