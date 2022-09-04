{ nodePackages, formats }:

{
  packages = [ nodePackages.markdownlint-cli ];

  configFormat = formats.json { };

  check = ''
    markdownlint --config "$config" "$path";
  '';

  fix = ''
    markdownlint --config "$config" --fix "$path"
  '';
}
