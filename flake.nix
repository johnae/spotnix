{
  description = "Spotnix - spotify as named pipes";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, ... }@inputs:
    let
      package = pkgs: {
        pname = "spotnix";
        version = "v0.1.3";
        src = self;
        cargoSha256 = "sha256-KyLcyn0kC/aGe7gOMwBF/RXPAXxpY0rhqMxMxoQ4M94=";
        doCheck = false;
        nativeBuildInputs = [ pkgs.pkgconfig ];
        buildInputs = [ pkgs.openssl ];
        meta = {
          license = pkgs.lib.licenses.mit;
          maintainers = [
            {
              email = "john@insane.se";
              github = "johnae";
              name = "John Axel Eriksson";
            }
          ];
        };
      };
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
      let
        pkgs = forAllSystems (system: import inputs.nixpkgs {
          localSystem = { inherit system; };
          overlays = [ fenix.overlay ];
        });
        rustPlatform = forAllSystems (system: pkgs.${system}.makeRustPlatform {
          inherit (fenix.packages.${system}.minimal) cargo rustc;
        });
      in
        {
          overlay = final: prev: {
            spotnix = prev.rustPlatform.buildRustPackage (package prev);
          };
          defaultPackage = forAllSystems (system: rustPlatform.${system}.buildRustPackage (package pkgs.${system}));
          devShell = forAllSystems (system: import ./devshell.nix { pkgs = pkgs.${system}; });
        };
}
