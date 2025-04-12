{
  lib,
  buildPythonPackage,
  libiconv,
  pytestCheckHook,
  python,
  pythonOlder,
  rustPlatform,
  stdenv,
}:
buildPythonPackage {
  pname = "preprocess_cancellation";
  version = "0.2.1.1";
  format = "pyproject";
  disabled = pythonOlder "3.8";

  src = ./.;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs =
    [
      (python.withPackages (p: [
        p.pytest
      ]))
    ]
    ++ (with rustPlatform; [
      cargoCheckHook
      cargoSetupHook
      maturinBuildHook
    ]);

  buildInputs = lib.optional stdenv.isDarwin libiconv;

  pythonImportsCheck = [ "preprocess_cancellation" ];

  nativeCheckInputs = [ pytestCheckHook ];
}
