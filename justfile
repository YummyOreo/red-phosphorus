default:
    just --list

test: fmt clippy
    cargo test

clippy:
    cargo clippy

fmt:
    cargo +nightly fmt --all
