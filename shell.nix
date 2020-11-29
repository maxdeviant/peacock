with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "peacock";

  buildInputs = [
    stdenv
    SDL2
    SDL2_image
    SDL2_ttf
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];
}
