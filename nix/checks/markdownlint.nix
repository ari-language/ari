{ nodePackages, formats }:

{
  packages = [ nodePackages.markdownlint-cli ];

  settingsFormat = formats.json { };

  check = ''
    markdownlint --config "$config" "$path";
  '';

  fix = ''
    markdownlint --config "$config" --fix "$path"
  '';
}
