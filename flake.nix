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

    nixpkgs.url = "nixpkgs/release-23.11";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
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
            (final: prev: {
              craneLib = crane.lib.${system};
              craneLibLLvmTools = crane.lib.${system}.overrideToolchain
                (fenix.packages.${system}.complete.withComponents [
                  "cargo"
                  "llvm-tools"
                  "rustc"
                ]);
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
      {
        packages = {
          ari = callPackage ./nix/packages/ari.nix { };
          default = self.packages.${system}.ari;
        };

        checks = {
          inherit (self.packages.${system}.default)
            clippy
            coverage;

          flake-linter = linter.check;
        } // self.packages.${system};

        apps = {
          inherit (linter) fix;
        };

        devShells.default = self.packages.${system}.default.overrideAttrs (attrs: {
          doCheck = true;
          checkInputs = with pkgs; [
            self.packages.${system}.default.clippy.nativeBuildInputs
            self.packages.${system}.default.coverage.nativeBuildInputs
            linter.nativeBuildInputs
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
