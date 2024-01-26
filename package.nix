{ lib, buildPythonPackage, buildPythonApplication, fetchFromGitHub, rustPlatform
, stdenv, libiconv, brotli, hypothesis, lz4, memory-profiler, numpy, py
, pytest-benchmark, pytestCheckHook, python-snappy, zstd }:

# buildPythonPackage rec {
buildPythonApplication rec {
  pname = "preprocess_cancellation";
  version = "0.1.0";
  format = "pyproject";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-+rl2558VrOqMBA5QcaSAvvPev96jOWhbL+i6Mmzv0Xk=";
  };

  nativeBuildInputs = with rustPlatform; [ cargoSetupHook maturinBuildHook ];

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  nativeCheckInputs = [ pytestCheckHook ];
}
