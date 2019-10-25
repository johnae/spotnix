let
  mozilla = import (builtins.fetchGit {
      url = "https://github.com/mozilla/nixpkgs-mozilla.git";
      ref = "master";
      rev = "b52a8b7de89b1fac49302cbaffd4caed4551515f";
  });

  pkgs = (import <nixpkgs> { }).extend(mozilla);
  rustChannel = pkgs.rustChannelOf { date = "2019-10-21";
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
   ] ++ (if pkgs.rust-analyzer != null then [ rust-analyzer ] else []);
   RUST_SRC_PATH = "${rustChannel.rust-src}/lib/rustlib/src/rust/src";
  }
