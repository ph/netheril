{
  description = "paranormal - ain't afraid of no ghost";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    # TODO: this only need to support amd64 for now.
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustVersion = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in {
        devShell = pkgs.mkShell {
          buildInputs =
            [
              (rustVersion.override { extensions = [ "rust-src" "rustfmt" "clippy" ]; })
              pkgs.rust-analyzer
              pkgs.cmake
              pkgs.cloud-hypervisor
              pkgs.dosfstools
              pkgs.mtools
              pkgs.gh
              pkgs.act
            ];
        };
      });
}
