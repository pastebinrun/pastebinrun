{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs =
    [ cargo clippy diesel-cli nodejs-12_x openssl pkg-config postgresql.lib rustfmt ];
}
