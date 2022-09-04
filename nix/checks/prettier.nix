{ nodePackages, formats }:

{
  packages = [ nodePackages.prettier ];

  settingsFormat = formats.json { };

  check = ''
    prettier --config "$config" --check "$path"
  '';

  fix = ''
    prettier --config "$config" --write "$path"
  '';
}
