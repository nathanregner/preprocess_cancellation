{ lib, python311Packages, rustPlatform, stdenv, libiconv, }:
python311Packages.buildPythonPackage rec {
  pname = "preprocess_cancellation";
  version = "0.1.0";
  format = "pyproject";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-hXdGM9cUNSEBz244aIWYBW5EI+mMYOo1BxjfG+fiP4k=";
  };

  nativeBuildInputs = with rustPlatform; [ cargoSetupHook maturinBuildHook ];

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  nativeCheckInputs = [ python311Packages.pytestCheckHook ];
}
