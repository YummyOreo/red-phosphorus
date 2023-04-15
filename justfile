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

# Possible commands: list build start update | Will auto install packages
tech-docs command="list":
    @if [ "{{command}}" == "list" ]; then echo -e "\033[1m'list': view this\n'build': build the docs\n'start': view docs localy\n'update': update required packages\033[0m"; fi
    @if [ "{{command}}" == "update" ]; then echo -e "\033[1mUpdating packages...\033[0m"; cd ./tech-mc-docs/; yarn; fi
    @if [ "{{command}}" != "update" -a "{{command}}" != "list" -a ! -d "./tech-mc-docs/node_modules/" ]; then just tech-docs update; fi
    @if [ "{{command}}" == "build" ]; then echo -e "\033[1mBuilding...\033[0m"; cd ./tech-mc-docs/; yarn build; fi
    @if [ "{{command}}" == "start" ]; then echo -e "\033[1mStarting...\033[0m"; cd ./tech-mc-docs/; yarn start; fi
