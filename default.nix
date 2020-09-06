{ stdenv, rustPlatform, pkgconfig, openssl, self }:

rustPlatform.buildRustPackage {
  pname = "spotnix";
  version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
  src = self;
  cargoSha256 = "sha256-FXtaODjmMHoxX84aPynCV19wXhc7l+bl5FC88Av/2MQ=";
  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [ openssl ];
  doCheck = false;
  meta = {
    license = stdenv.lib.licenses.mit;
    maintainers = [
      {
        email = "john@insane.se";
        github = "johnae";
        name = "John Axel Eriksson";
      }
    ];
  };
}
