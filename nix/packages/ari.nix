{ lib
, craneLib
}:

let
  src = craneLib.cleanCargoSource (craneLib.path ../..);
  cargoArtifacts = craneLib.buildDepsOnly {
    inherit src;
  };
in
craneLib.buildPackage {
  pname = "ari";
  version = "0.1";
  inherit src cargoArtifacts;

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
