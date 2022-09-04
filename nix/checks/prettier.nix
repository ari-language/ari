{ nodePackages, formats }:

{
  packages = [ nodePackages.prettier ];

  configFormat = formats.json { };

  check = ''
    prettier --config "$config" --check "$path"
  '';

  fix = ''
    prettier --config "$config" --write "$path"
  '';
}
