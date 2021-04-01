{ nixpkgs }:

nixpkgs.mkShell {
  buildInputs = [
    (nixpkgs.rust-nightly.latest.withComponents [
      "cargo"
      "clippy-preview"
      "rust-src"
      "rust-std"
      "rustc"
      "rustfmt-preview"
    ])
    nixpkgs.rust-analyzer-nightly
    nixpkgs.gcc
    nixpkgs.openssl
    nixpkgs.pkg-config
    nixpkgs.skim
  ];
}
