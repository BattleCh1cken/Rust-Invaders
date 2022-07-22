{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nmattia/naersk/master";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell rec {
          nativeBuildInputs = [
            pkg-config
            llvmPackages.bintools # To use lld linker
            cargo
            rustc
            gcc
            rustfmt
            rust-analyzer
            clippy
          ];
          buildInputs = [
            udev
            alsaLib
            vulkan-loader
            xlibsWrapper
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi # To use x11 feature
            libxkbcommon
            wayland # To use wayland feature
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

        };
      });
}
