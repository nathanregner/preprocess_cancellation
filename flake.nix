{
  description = "GCode processor to add klipper cancel-object markers";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        packages = rec {
          default = pkgs.python3.pkgs.callPackage ./package.nix { };
          bench = pkgs.writeShellApplication {
            name = "bench";
            runtimeInputs = [ (pkgs.python3.withPackages (ps: [ default ])) ];
            text = "python ${./bench.py}";
          };
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ packages.default ];
          packages = (
            with pkgs;
            [
              cargo
              cargo-insta
              cargo-nextest
              clippy
              rust-analyzer
              rustfmt
            ]
          );
        };
      }
    );
}
