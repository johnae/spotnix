{pkgs}: {
  imports = ["${pkgs.devshell.extraModulesDir}/language/rust.nix"];
  packages = [pkgs.pkgconfig pkgs.openssl pkgs.gcc];
  language.rust = {
    packageSet = pkgs.fenixRust;
    tools = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
    ];
  };
  env = [
    {
      name = "PKG_CONFIG_PATH";
      value = "${pkgs.openssl.dev}/lib/pkgconfig";
    }
  ];
  commands = [
    {
      package = pkgs.fenixRust.cargo;
      name = "cargo";
      help = "manage dependencies and the full build cycle of rust programs";
      category = "devtools";
    }
    {
      package = pkgs.fenixRust.rustc;
      name = "rustc";
      help = "the rust compiler";
      category = "devtools";
    }
  ];
}
