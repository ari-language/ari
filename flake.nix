{
  description = "A type-centred purely functional programming language designed to type binary files";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    flake-linter = {
      url = "gitlab:kira-bruneau/flake-linter";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "nixpkgs/release-22.05";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, flake-linter, nixpkgs, crane }:
    let
      lib = nixpkgs.lib;
      systems = [ "x86_64-linux" ];
      paths = flake-linter.lib.partitionToAttrs
        flake-linter.lib.commonFlakePaths
        (flake-linter.lib.walkFlake ./.);
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system} // {
          craneLib = crane.lib.${system};
        };

        callPackage = pkgs.newScope pkgs;

        linter = flake-linter.lib.makeFlakeLinter {
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

          flake-linter = linter.check;
        } // packages;

        apps = {
          inherit (linter) fix;
        };

        devShells.default = packages.default.overrideAttrs (attrs: {
          doCheck = true;
          checkInputs = with pkgs; [
            cargo-tarpaulin
            clippy
            linter.nativeBuildInputs
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
