{
  description = "Notmuch initial tagging";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, ... }@inputs:
    let
      package = pkgs: {
        pname = "spotnix";
        version = "v0.1.3";
        src = self;
        cargoSha256 = "sha256-tIZj+BdmJbRiSSWqY+JIDIShJ8wdLuIEWAb4c0m0MXw=";
        doCheck = false;
        nativeBuildInputs = [ pkgs.pkgconfig ];
        buildInputs = [ pkgs.openssl ];
        meta = {
          license = pkgs.stdenv.lib.licenses.mit;
          maintainers = [
            {
              email = "john@insane.se";
              github = "johnae";
              name = "John Axel Eriksson";
            }
          ];
        };
      };
    in
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          nixpkgs = import inputs.nixpkgs {
            localSystem = { inherit system; };
            config = {
              allowUnfree = true;
            };
            overlays = [
              inputs.fenix.overlay
            ];
          };
          rustPlatform = nixpkgs.makeRustPlatform {
            inherit (inputs.fenix.packages.${system}.minimal) cargo rustc;
          };
        in
        {
          defaultPackage = rustPlatform.buildRustPackage (package nixpkgs);
          devShell = import ./shell.nix { inherit nixpkgs; };

        }
      ) // {
      overlay = final: prev: {
        spotnix = prev.rustPlatform.buildRustPackage (package prev);
      };

    };
}
