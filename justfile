default:
    just --list

test pkg="all": fmt clippy
    @echo -e "\033[1mTesting {{pkg}}...\033[0m"
    @if [ "{{pkg}}" == "all" ]; then cargo test; else cargo test -p {{pkg}}; fi

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
