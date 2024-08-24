{
  description = "Bevy devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{ self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        bevy-deps = [
          pkgs.udev
          pkgs.alsa-lib
          pkgs.vulkan-loader
          pkgs.xorg.libX11
          pkgs.xorg.libXcursor
          pkgs.xorg.libXi
          pkgs.xorg.libXrandr # To use the x11 feature
          pkgs.libxkbcommon
          pkgs.wayland # To use the wayland feature
          pkgs.libz
          pkgs.openssl
        ];
        tools = [
          pkgs.renderdoc
          pkgs.pkg-config
          pkgs.cargo-nextest
          pkgs.cargo-edit
          pkgs.just
        ];

      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            # rust deps
            mold
            llvmPackages_latest.clang
            stdenv
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
              targets = [ ];
            })
          ] ++ bevy-deps ++ tools;
          LD_LIBRARY_PATH = lib.makeLibraryPath (bevy-deps);
        };
      }
    );
}

