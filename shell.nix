{ pkgs ? import <nixpkgs> { } }:
let
  RUST_SRC_PATH = pkgs.stdenv.mkDerivation {
    inherit (pkgs.rustc) src;
    inherit (pkgs.rustc.src) name;
    phases = [ "unpackPhase" "installPhase" ];
    installPhase = "cp -r src $out";
  };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo

    pkgs.gcc
    pkgs.openssl
    pkgs.pkg-config

    pkgs.skim
  ];
  inherit RUST_SRC_PATH;
}
