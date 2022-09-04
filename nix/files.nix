{ lib }:

let
  walkDir = dir:
    (builtins.concatLists
      (builtins.attrValues
        (builtins.mapAttrs
          (path: type:
            if type == "directory"
            then walkDir "${dir}/${path}"
            else [ (lib.removePrefix "/" "${dir}/${path}") ])
          (builtins.readDir (../. + dir)))));
in
builtins.foldl'
  (files: path: {
    markdown = files.markdown
      ++ lib.optional (lib.hasSuffix ".md" path) path;

    nix = files.nix
      ++ lib.optional (lib.hasSuffix ".nix" path) path;

    rust = files.rust
      ++ lib.optional (lib.hasSuffix ".rs" path) path;
  })
{
  markdown = [ ];
  nix = [ ];
  rust = [ ];
}
  (walkDir "")
