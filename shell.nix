{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs =
    [ cargo clippy nodejs-12_x openssl pkg-config postgresql.lib rustfmt ];
}
