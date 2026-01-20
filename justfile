# just is a command runner, Justfile is very similar to Makefile, but simpler.

default:
  @just --list

# Run clippy to check for linting issues
lint:
  cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
  cargo fmt
