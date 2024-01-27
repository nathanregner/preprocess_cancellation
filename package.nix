{ lib, buildPythonPackage, pythonOlder, rustPlatform, stdenv, libiconv
, pytestCheckHook }:
buildPythonPackage rec {
  pname = "preprocess_cancellation";
  version = "0.2.0";
  format = "pyproject";
  disabled = pythonOlder "3.8";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-BGC3qtd3SQdLIh5YqXlNh41egOQlOEY+FdEkjGPT9JA=";
  };

  nativeBuildInputs = with rustPlatform; [ cargoSetupHook maturinBuildHook ];

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  pythonImportsCheck = [ "preprocess_cancellation" ];

  nativeCheckInputs = [ pytestCheckHook ];
}
