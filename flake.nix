{
  description = "Spotnix - spotify as named pipes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    dream2nix = {
      url = "github:nix-community/dream2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    dream2nix,
    flake-utils,
    devshell,
    fenix,
    nixpkgs,
  }: let
    l = nixpkgs.lib // builtins;
    pkgsFor = system:
      import nixpkgs {
        inherit system;
        overlays = [
          devshell.overlay
          fenix.overlay
          (
            final: prev: let
              fenixRust = final.fenix.complete;
            in {
              inherit fenixRust;
              rustToolchain = fenixRust.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ];
            }
          )
        ];
      };

    initD2N = pkgs:
      dream2nix.lib.init {
        inherit pkgs;
        config.projectRoot = ./.;
        config.disableIfdWarning = true;
      };

    makeOutputs = pkgs: let
      outputs = (initD2N pkgs).makeOutputs {
        source = ./.;
        settings = [
          {
            builder = "crane";
            translator = "cargo-lock";
          }
        ];
        packageOverrides = {
          "^.*".add-deps = {
            nativeBuildInputs = old: old ++ [pkgs.pkgconfig];
            buildInputs = old: old ++ [pkgs.openssl];
            overrideRustToolchain = old: {
              cargo = pkgs.fenixRust.toolchain;
            };
          };
        };
      };
    in {
      packages.${pkgs.system} = outputs.packages;
      checks.${pkgs.system} = {
        inherit (outputs.packages) spotnix;
      };
    };
    allOutputs = l.map makeOutputs (map pkgsFor flake-utils.lib.defaultSystems);
    outputs = l.foldl' l.recursiveUpdate {} allOutputs;
  in
    outputs
    // {
      overlays.default = final: prev: {
        spotnix = self.packages.${prev.system}.spotnix;
      };
    }
    // (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = pkgsFor system;
    in {
      devShells.default = pkgs.devshell.mkShell {
        imports = [
          (import ./devshell.nix {pkgs = pkgs;})
        ];
      };
    }));
}
