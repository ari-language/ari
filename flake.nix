{
  description = "A type-centred purely functional programming language designed to type binary files";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/release-22.05";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, nixpkgs, crane }:
    let
      lib = nixpkgs.lib;
      systems = [ "x86_64-linux" ];
      files = import ./nix/files.nix { inherit lib; };
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system} // {
          craneLib = crane.lib.${system};
        };

        callPackage = pkgs.newScope pkgs;

        flake-checker = callPackage ./nix/checks/flake-checker.nix {
          root = ./.;
          settings = {
            markdownlint = {
              paths = files.markdown;
              extraSettings = {
                default = true;
                MD033 = {
                  allowed_elements = [ "img" "table" "tr" "th" "td" ];
                };
              };
            };
            nixpkgs-fmt.paths = files.nix;
            prettier.paths = files.markdown;
            rustfmt.paths = files.rust;
          };
        };
      in
      rec {
        packages.default = callPackage ./nix/packages/ari.nix { };

        checks = {
          clippy = packages.default.checks.clippy;
          coverage = packages.default.checks.coverage;
          flake-checker = flake-checker.check;
        } // packages;

        apps = {
          fix = flake-checker.fix;
          fix-check = flake-checker.fix-check;
        };

        devShells.default = packages.default.overrideAttrs (attrs: {
          doCheck = true;
          checkInputs = with pkgs; [
            flake-checker.packages
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
