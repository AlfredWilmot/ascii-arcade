.PHONY: docs lint format test build release run all

all: lint test build run

run: build
	cargo run

release: lint test
	cargo build --release

build:
	cargo build

test:
	cargo test --all

format:
	cargo fmt --all

lint:
	cargo fmt --all --check
	cargo clippy --all-targets --all-features --workspace -- -D warnings

docs:
	cargo doc --no-deps --document-private-items --all-features --workspace --examples

clean:
	rm -rf target


# REFERENCES:
# - https://github.com/rust-github/template/blob/main/template/.github/workflows/ci.yml
