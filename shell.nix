let
  mozilla = import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      ref = "master";
      rev = "ac8e9d7bbda8fb5e45cae20c5b7e44c52da3ac0c";
  });

  pkgs = import <nixpkgs> (
    { overlays = [ mozilla ]; }
  );
  rustChannel = pkgs.rustChannelOf { date = "2019-09-01";
                                     channel = "nightly";
                                   };

in

  with pkgs; mkShell {
   buildInputs = [
               (rustChannel.rust.override { extensions = [ "clippy-preview" ]; })
               gcc
               openssl
               pkg-config
               skim
              ];
   RUST_SRC_PATH = "${rustChannel.rust-src}/lib/rustlib/src/rust/src";
  }
