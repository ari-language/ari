{ rustfmt, formats }:

{
  packages = [ rustfmt ];

  configFormat = formats.toml { };

  check = ''
    rustfmt --config-path "$config" --check "$path"
  '';

  fix = ''
    rustfmt --config-path "$config" "$out"
  '';
}
