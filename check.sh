#!/usr/bin/env sh

# exit immediately if any stage fails
set -e

# standardize formatting across all *.rs files
cargo fmt --all

# run all unit-tests
cargo test --all-features --workspace

# run linter
cargo clippy --all-targets --all-features --workspace -- -D warnings


# REFERENCES:
# - https://github.com/rust-github/template/blob/main/template/.github/workflows/ci.yml
