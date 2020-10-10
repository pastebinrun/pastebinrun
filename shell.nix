{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [ cargo nodejs-12_x openssl pkg-config postgresql.lib ];
}
