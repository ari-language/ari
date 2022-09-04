{ rustfmt, formats }:

{
  packages = [ rustfmt ];

  settingsFormat = formats.toml { };

  check = ''
    rustfmt --config-path "$config" --check "$path"
  '';

  fix = ''
    rustfmt --config-path "$config" "$path"
  '';
}
