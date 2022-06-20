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
            ## let's build with rustc/cargo from nixpkgs
            # overrideRustToolchain = old: {
            #   cargo = pkgs.fenix.complete.toolchain;
            # };
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
    // (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = pkgsFor system;
    in {
      devShells.default = pkgs.devshell.mkShell {
        imports = [
          (pkgs.devshell.importTOML ./devshell.toml)
        ];
      };
    }));
}
