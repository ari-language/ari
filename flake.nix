{
  description = "A type-centred purely functional programming language designed to type binary files";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    flake-linter = {
      url = "gitlab:kira-bruneau/flake-linter";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    nixpkgs.url = "nixpkgs/release-23.05";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, flake-utils, flake-linter, nixpkgs, crane, fenix }:
    let
      lib = nixpkgs.lib;
      systems = [ "x86_64-linux" ];
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs {
          overlays = [
            fenix.overlays.default
            (final: prev:
              let
                toolchain = final.fenix.complete.withComponents [
                  "cargo"
                  "clippy"
                  "llvm-tools"
                  "rust-analyzer"
                  "rustc"
                  "rustfmt"
                ];
              in
              {
                cargo = toolchain;
                clippy = toolchain;
                craneLib = crane.lib.${system}.overrideToolchain toolchain;
                rustc = toolchain;
                rustfmt = toolchain;
              })
          ];

          inherit system;
        };

        callPackage = pkgs.newScope pkgs;

        flake-linter-lib = flake-linter.lib.${system};

        paths = flake-linter-lib.partitionToAttrs
          flake-linter-lib.commonPaths
          (flake-linter-lib.walkFlake ./.);

        linter = flake-linter-lib.makeFlakeLinter {
          root = ./.;

          settings = {
            markdownlint = {
              paths = paths.markdown;
              settings = {
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
            checks.clippy.nativeBuildInputs
            checks.coverage.nativeBuildInputs
            linter.nativeBuildInputs
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
