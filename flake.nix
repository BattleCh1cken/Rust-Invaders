{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nmattia/naersk/master";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, naersk, pre-commit-hooks }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };

        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell rec {
          inherit (self.checks.${system}.pre-commit-check) shellHook;
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
