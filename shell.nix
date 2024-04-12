{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
  ];
  buildInputs = with pkgs; [
    rustc
    rust-analyzer
    rustfmt
    SDL2
  ] ++ lib.optional stdenv.isDarwin libiconv;
}
