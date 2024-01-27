install:
    poetry install

pytest:
    maturin develop --release
    ./.venv/bin/pytest $@

snapshot-test:
    cargo insta test --unreferenced delete
