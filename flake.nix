{
  description = "A type-centred purely functional programming language designed to type binary files";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/release-22.05";
  };

  outputs = { self, flake-utils, nixpkgs }:
    let systems = [ "x86_64-linux" ]; in
    flake-utils.lib.eachSystem systems (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      rec {
        checks = {
          readme = pkgs.callPackage ./nix/checks/readme.nix { };
        } // packages;

        packages.default = pkgs.callPackage ./nix/packages/ari.nix { };

        devShells.default = packages.default.overrideAttrs (attrs: {
          checkInputs = with pkgs; [
            nodePackages.prettier
            nodePackages.markdown-link-check
          ];
        });
      }
    );
}
