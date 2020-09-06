{
  description = "Spotify for UNIX";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils, ... }:
    {
      overlay = final: prev: {
        spotnix = final.callPackage ./. { inherit self; };
      };
    } // (
      flake-utils.lib.eachDefaultSystem
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in
          {
            defaultPackage =
              (import nixpkgs {
                inherit system;
                overlays = [ self.overlay ];
              }).spotnix;
            packages = flake-utils.lib.flattenTree {
              spotnix = self.defaultPackage;
            };
            apps.spotnix = flake-utils.lib.mkApp {
              drv = self.spotnix;
              exePath = "/bin/spotnix";
            };
            defaultApp = self.apps.spotnix;
            devShell = import ./shell.nix { inherit pkgs; };
          }
        )
    );
}
