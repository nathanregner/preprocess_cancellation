.PHONY: test

test:
	maturin develop --release
	./.venv/bin/pytest $@
