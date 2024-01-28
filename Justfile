install *FLAGS:
    poetry install --no-root {{FLAGS}}

pytest *FLAGS:
    maturin develop --release
    ./.venv/bin/pytest {{FLAGS}}

snapshot-test:
    cargo insta test --unreferenced delete
