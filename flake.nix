{
  description = "Benstoy development flake";

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShell = pkgs.mkShell {
        buildInputs = [
          (fenix.packages.${system}.complete.withComponents ["rust-src"])

          # c dependencies for cargo library
          pkgs.pkg-config
          pkgs.gnumake
          pkgs.openssl
        ];
      };
    });
}
