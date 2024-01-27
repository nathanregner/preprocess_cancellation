{
  description = "GCode processor to add klipper cancel-object markers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust-bin = pkgs.rust-bin.nightly.latest.default;
      in rec {
        packages.default = pkgs.python311Packages.callPackage ./package.nix {
          rustPlatform = (let
            self = pkgs.makeRustPlatform {
              cargo = rust-bin;
              rustc = rust-bin;
            };
          in self // {
            # stupid hack to propagate nightly
            maturinBuildHook = self.maturinBuildHook.override (oldAttrs: {
              pkgsHostTarget = oldAttrs.pkgsHostTarget // {
                rustc = rust-bin;
                cargo = rust-bin;
              };
            });
          });
        };
        devShells.default = pkgs.mkShell {
          inherit (packages.default) nativeBuildInputs;
          venvDir = "./.venv";
          buildInputs = [ packages.default.buildInputs rust-bin ]
            ++ (with pkgs; [
              cargo-insta
              poetry
              python311Packages.venvShellHook
              rust-analyzer
            ]);
        };
      });
}
