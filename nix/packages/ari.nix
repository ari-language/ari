{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage {
  pname = "ari";
  version = "0.1";
  src = ../../ari;
  cargoSha256 = "sha256-Ec+nKZKMKUhh5JvBF0rn/8H0ijlOKAElH9MikaD4zfg=";

  meta = with lib; {
    description = "A type-centred purely functional programming language designed to type binary files";
    homepage = "https://gitlab.com/ari-lang/ari";
    license = licenses.gpl3Plus;
    maintainers = with maintainers; [ kira-bruneau ];
  };
}
