set shell := ["bash", "-c"]

default:
    just --list

test: fmt clippy
    @echo -e "\033[1mTesting...\033[0m"
    @cargo test

# Do: `just clippy` for just linting OR `just clippy fmt` for both linting and formatting
clippy fmt="":
    @if [ "{{fmt}}" == "fmt" ]; then just fmt; fi
    @echo -e "\033[1mLinting...\033[0m"
    @cargo clippy

# Do: `just fmt` for just formatting OR `just fmt clippy` for both linting and formatting
fmt clippy="":
    @if [ "{{clippy}}" == "clippy" ]; then just clippy; fi
    @echo -e "\033[1mFormatting...\033[0m"
    @cargo +nightly fmt --all
