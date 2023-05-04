# Contributing
## Commits
We use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) as a format for commit messages. You *dont* have to use this format in your pr as we will fix it when we squash and commit it into main! (this also means mege commits are fine)
> **Note**
> 
> For `skip ci`: Put `skip ci` in the description when you make changes to non-code things such as docs!

## Requirements
We use [just](https://github.com/casey/just) to run commands, run `just fmt clippy` before pushing to format and lint your code.

## Testing:
To run the tests you can just run `just test`!
