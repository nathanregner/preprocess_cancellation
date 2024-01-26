{ lib, buildPythonPackage, rustPlatform, stdenv, libiconv, pytestCheckHook, }:
buildPythonPackage rec {
  pname = "preprocess_cancellation";
  version = "0.1.0";
  format = "pyproject";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-d+k2PoWlV+tnhoVzS29NZsGxVCuv4y6+Pyz534gtRBM=";
  };

  nativeBuildInputs = with rustPlatform; [ cargoSetupHook maturinBuildHook ];

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  nativeCheckInputs = [ pytestCheckHook ];
}
