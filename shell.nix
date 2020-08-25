{ nixpkgs }:

nixpkgs.mkShell {
  buildInputs = [
    nixpkgs.rustc
    nixpkgs.cargo

    nixpkgs.gcc
    nixpkgs.openssl
    nixpkgs.pkg-config

    nixpkgs.skim
  ];
}
