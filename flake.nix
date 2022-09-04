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

        flake-file-checker = callPackage ./nix/checks/flake-file-checker.nix {
          root = ./.;
          checkers = {
            prettier = {
              checker = callPackage ./nix/checks/prettier.nix { };
              files = files.markdown;
            };
            markdownlint = {
              checker = callPackage ./nix/checks/markdownlint.nix { };
              files = files.markdown;
              config = {
                default = true;
                MD033 = {
                  allowed_elements = [ "img" "table" "tr" "th" "td" ];
                };
              };
            };
            nixpkgs-fmt = {
              checker = callPackage ./nix/checks/nixpkgs-fmt.nix { };
              files = files.nix;
            };
            rustfmt = {
              checker = callPackage ./nix/checks/rustfmt.nix { };
              files = files.rust;
            };
          };
        };
      in
      rec {
        packages.default = callPackage ./nix/packages/ari.nix { };

        checks = {
          clippy = packages.default.checks.clippy;
          coverage = packages.default.checks.coverage;
          flake-file-checker = flake-file-checker.check;
        } // packages;

        apps = {
          fix = flake-file-checker.fix;
          fix-check = flake-file-checker.fix-check;
        };

        devShells.default = packages.default.overrideAttrs (attrs: {
          doCheck = true;
          checkInputs = with pkgs; [
            flake-file-checker.packages
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
