{ lib
, craneLib
}:

let
  commonArgs = {
    src = ../../ari;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (commonArgs // {
  pname = "ari";
  version = "0.1";

  inherit cargoArtifacts;

  doCheck = false;

  passthru.checks = {
    coverage = craneLib.cargoTarpaulin (commonArgs // {
      inherit cargoArtifacts;
    });
  };

  meta = with lib; {
    description = "A type-centred purely functional programming language designed to type binary files";
    homepage = "https://gitlab.com/ari-lang/ari";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [ kira-bruneau ];
  };
})
