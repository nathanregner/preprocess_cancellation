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
        lib = pkgs.lib;
        rust = pkgs.rust-bin.nightly.latest.default;
        rustOverrides = {
          cargo = rust;
          rustc = rust;
        };
      in rec {
        packages = rec {
          default = pkgs.python3.pkgs.callPackage ./package.nix {
            rustPlatform = (let self = pkgs.makeRustPlatform rustOverrides;
            in self // {
              # stupid hack to propagate nightly
              maturinBuildHook = self.maturinBuildHook.overrideAttrs
                (oldAttrs: {
                  propagatedBuildInputs = [ pkgs.pkgsHostTarget.maturin ]
                    ++ (lib.attrValues rustOverrides);
                });
            });
          };
          bench = pkgs.writeShellApplication {
            name = "bench";
            runtimeInputs = [ (pkgs.python3.withPackages (ps: [ default ])) ];
            text = "python ${./bench.py}";
          };
        };
        devShells.default = pkgs.mkShell {
          inherit (packages.default) nativeBuildInputs;
          venvDir = "./.venv";
          buildInputs = [ packages.default.buildInputs rust ] ++ (with pkgs; [
            cargo-insta
            cargo-nextest
            just
            poetry
            python311Packages.venvShellHook
            rust-analyzer
          ]);
        };
      });
}
