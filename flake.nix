{
  description = "GCode processor to add klipper cancel-object markers";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: rec {
        packages.default = pkgs.python311Packages.callPackage ./package.nix { };
        devShells.default = pkgs.mkShell {
          inherit (packages.default) nativeBuildInputs;
          venvDir = "./.venv";
          buildInputs = [
            packages.default.buildInputs
            pkgs.cargo-insta
            pkgs.poetry
            pkgs.python311Packages.venvShellHook
            pkgs.rust-analyzer
          ];
        };
      };
    };
}
