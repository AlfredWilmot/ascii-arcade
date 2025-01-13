#!/usr/bin/env sh

# exit immediately if any stage fails
set -e

_header() {
  echo "# ---------------------- #"
  echo "# $*"
  echo "# ---------------------- #"
}

# standardize formatting across all *.rs files
_header "Formatting *.rs files"
cargo fmt --all

# run linter
_header "Running linters"
cargo clippy --all-targets --all-features --workspace -- -D warnings

# run all unit-tests
_header "Running unit-tests"
cargo test --all-features --workspace
cargo test --all-features --doc

# REFERENCES:
# - https://github.com/rust-github/template/blob/main/template/.github/workflows/ci.yml
