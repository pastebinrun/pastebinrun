{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs =
    [ cargo clippy diesel-cli nodejs-16_x openssl pkg-config postgresql.lib rustfmt ];
}
