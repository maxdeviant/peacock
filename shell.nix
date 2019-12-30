with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "peacock";

  buildInputs = [
    stdenv
    SDL2
  ];
}
