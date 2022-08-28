{ runCommand
, nodePackages
}:

runCommand "readme-check" {
  nativeBuildInputs = [ nodePackages.prettier ];
} ''
  prettier --check ${../../README.md} > "$out"
''
