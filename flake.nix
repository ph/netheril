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

        cloud-image =
          let
            inherit (pkgs) stdenv lib;
          in
            stdenv.mkDerivation  rec {
              version = "0250218.1";
              name = "ubuntu-noble-server-cloudimg";

              src = builtins.fetchurl {
                url = "https://cloud-images.ubuntu.com/focal/0250218.1/focal-server-cloudimg-amd64.img";
                sha256 = "sha256:00vji71fsgg17qsc1hxknrfh5dm7p3rspskbbgsw5wf26lj37gdm";
              };

              sourceRoot = ".";
              buildInputs = [
                pkgs.qemu
              ];
              phases = ["unpackPhase" "buildPhase" "installPhase" "configurePhase"];
              configurePhase = ''
                export TMP=$(mktemp -d)
                '';
              unpackPhase = ''
                mkdir -p $out
                '';
              buildPhase = ''
                qemu-img convert -p -f qcow2 -O raw $src $TMP/${name}.raw
'';
              installPhase = ''
                cp $TMP/${name}.raw $out
                '';
              meta = with nixpkgs.lib; {
                homepage = "https://cloud-images.ubuntu.com/noble";
                description = "Ubuntu Noble Server Cloud Image";
                platforms = platforms.linux;
              };
            };

        cloud-hypervisor-firmware =
          let
            inherit (pkgs) stdenv lib;
          in
            stdenv.mkDerivation  rec {
              version = "0.5.0";
              name = "cloud-hypervisor-firmware";
              src = builtins.fetchurl {
                url = "https://github.com/cloud-hypervisor/rust-hypervisor-firmware/releases/download/${version}/hypervisor-fw";
                sha256 = "sha256:0n6rdb0kwdwmwqlpxq2sbszm18zrxnyid8lq45fv3xk8ffbiw2ja";
              };

              sourceRoot = ".";
              phases = ["unpackPhase" "installPhase"];
              unpackPhase = ''
                mkdir -p $out
                '';
              installPhase = ''
                cp $src $out/${name}
                '';
              meta = with nixpkgs.lib; {
                homepage = "https://github.com/cloud-hypervisor/rust-hypervisor-firmware";
                description = "Hypervisor firmware for cloud-hypervisor";
                platforms = platforms.linux;
              };
            };

        rustVersion = pkgs.rust-bin.stable.latest.default;
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
              cloud-image
              cloud-hypervisor-firmware
            ];

          CLOUD_IMAGE="${cloud-image}/${cloud-image.name}.raw";
          HYPERVISOR_FIRMWARE="${cloud-hypervisor-firmware}/${cloud-hypervisor-firmware.name}";
        };
      });
}
