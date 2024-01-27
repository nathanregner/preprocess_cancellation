install:
    poetry install

pytest:
    maturin develop --release
    ./.venv/bin/pytest $@
