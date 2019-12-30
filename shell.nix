with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "peacock";

  buildInputs = [
    stdenv
    cmake
  ];
}
