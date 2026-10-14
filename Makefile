.PHONY: build test fmt clippy clean

build:
	stellar contract build

test:
	cargo test --workspace

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

clean:
	cargo clean
