{ nixpkgs-fmt }:

{
  packages = [ nixpkgs-fmt ];

  check = ''
    nixpkgs-fmt --check < "$path"
  '';

  fix = ''
    nixpkgs-fmt "$out"
  '';
}
