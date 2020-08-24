default: build

pre-clippy:
	@rustup component add clippy

clippy:
	cargo clippy

pre-format:
	@rustup component add rustfmt

format: pre-format
	@cargo fmt --all -- --check >/dev/null || \
	cargo fmt --all

test:
	echo "No tests no, please add some!"

dev: format clippy
	make test

build:
	cargo build

release:
	cargo build --release