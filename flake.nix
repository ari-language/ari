{
  description = "A type-centred purely functional programming language designed to type binary files";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/release-22.05";
  };

  outputs = { self, flake-utils, nixpkgs }:
    let
      lib = nixpkgs.lib;
      systems = [ "x86_64-linux" ];
      files = import ./nix/files.nix { inherit lib; };
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        flake-file-checker = pkgs.callPackage ./nix/checks/flake-file-checker.nix {
          root = ./.;
          checkers = {
            prettier = {
              checker = pkgs.callPackage ./nix/checks/prettier.nix { };
              files = files.markdown;
            };
          };
        };
      in
      rec {
        checks = {
          flake-file-checker = flake-file-checker.check;
        } // packages;

        packages = {
          default = pkgs.callPackage ./nix/packages/ari.nix { };
          fix = flake-file-checker.fix;
        };

        devShells.default = packages.default.overrideAttrs (attrs: {
          checkInputs = with pkgs; [
            nodePackages.markdown-link-check
            nodePackages.prettier
          ];
        });
      }
    );
}
