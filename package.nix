{ lib, python311Packages, rustPlatform, stdenv, libiconv, }:
python311Packages.buildPythonPackage rec {
  pname = "preprocess_cancellation";
  version = "0.1.0";
  format = "pyproject";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-sOdvMM9a0XVe8MqQybcgTThm3ExIM9tMDMItswhdI+k=";
  };

  nativeBuildInputs = with rustPlatform; [ cargoSetupHook maturinBuildHook ];

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  nativeCheckInputs = [ python311Packages.pytestCheckHook ];
}
