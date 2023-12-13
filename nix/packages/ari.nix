{ lib
, craneLib
, craneLibLLvmTools
, cargo-nextest
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

    coverage = craneLibLLvmTools.cargoLlvmCov {
      inherit src cargoArtifacts;
      nativeBuildInputs = [ cargo-nextest ];
      cargoLLvmCovCommand = "nextest";
      cargoLlvmCovExtraArgs = "--ignore-filename-regex /nix/store --show-missing-lines --lcov --output-path $out";
    };
  };

  meta = with lib; {
    description = "A type-centred purely functional programming language designed to type binary files";
    homepage = "https://gitlab.com/ari-lang/ari";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [ kira-bruneau ];
  };
}
