{ lib, buildPythonPackage, fetchFromGitHub, rustPlatform, stdenv, libiconv
, brotli, hypothesis, lz4, memory-profiler, numpy, py, pytest-benchmark
, pytestCheckHook, python-snappy, zstd }:

buildPythonPackage rec {
  pname = "preprocess_cancellation";
  version = "0.1.0";
  format = "pyproject";

  src = ./.;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit src;
    hash = "sha256-QvDZPUOH4G3ZcECc1ejtO6HFC9IT6WvlAMMLJlNyRnI=";
  };

  nativeBuildInputs = with rustPlatform; [ cargoSetupHook maturinBuildHook ];

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  # nativeCheckInputs = [
  #   brotli
  #   hypothesis
  #   lz4
  #   memory-profiler
  #   numpy
  #   py
  #   pytest-benchmark
  #   pytestCheckHook
  #   python-snappy
  #   zstd
  # ];

  # pytestFlagsArray = [ "--benchmark-disable" ];
  #
  # disabledTestPaths = [ "benchmarks/test_bench.py" ];
  #
  # pythonImportsCheck = [ "cramjam" ];
}
