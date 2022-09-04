{
  description = "A type-centred purely functional programming language designed to type binary files";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    flake-checker = {
      url = "gitlab:kira-bruneau/flake-checker";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "nixpkgs/release-22.05";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, flake-checker, nixpkgs, crane }:
    let
      lib = nixpkgs.lib;
      systems = [ "x86_64-linux" ];
      paths = flake-checker.lib.partitionToAttrs
        flake-checker.lib.commonFlakePaths
        (flake-checker.lib.walkFlake ./.);
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system} // {
          craneLib = crane.lib.${system};
        };

        callPackage = pkgs.newScope pkgs;

        checker = flake-checker.lib.makeFlakeChecker {
          root = ./.;
          settings = {
            markdownlint = {
              paths = paths.markdown;
              extraSettings = {
                default = true;
                MD033 = {
                  allowed_elements = [ "img" "table" "tr" "th" "td" ];
                };
              };
            };
            nixpkgs-fmt.paths = paths.nix;
            prettier.paths = paths.markdown;
            rustfmt.paths = paths.rust;
          };

          inherit pkgs;
        };
      in
      rec {
        packages.default = callPackage ./nix/packages/ari.nix { };

        checks = {
          inherit (packages.default.checks)
            clippy
            coverage;

          inherit (checker) check;
        } // packages;

        apps = {
          inherit (checker) fix;
        };

        devShells.default = packages.default.overrideAttrs (attrs: {
          doCheck = true;
          checkInputs = with pkgs; [
            checker.packages
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
