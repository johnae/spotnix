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
      inherit (nixpkgs.lib) genAttrs;
      package = pkgs: {
        pname = "spotnix";
        version = "v0.1.4";
        src = self;
        cargoSha256 = "sha256-cku0dbsGI0pAkcoBl+YxmwAIw2Gq+lruYyXet6OMyiA=";
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
      forAllSystems = genAttrs supportedSystems;
    in
      let
        pkgs = forAllSystems (system: import nixpkgs {
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
