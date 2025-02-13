{
  description = "paranormal - ain't afraid of no ghost";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

	rustVersion = pkgs.rust-bin.nightly.latest.default;
      in {
        devShell = pkgs.mkShell {
          buildInputs =
            [
              (rustVersion.override { extensions = [ "rust-src" "rustfmt" "clippy" ]; })
              pkgs.rust-analyzer
              pkgs.openssl
              pkgs.glibc
              pkgs.pkg-config
              pkgs.cmake
              pkgs.act
            ];
        };
      });
}
