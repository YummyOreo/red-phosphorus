# red-phosphorus
A redstone emulator aimed at speed and ease of use. Currently it is still in the developement phase, but plans to go into beta in a bit!

# How To Use
> Currently it is NOT AT A USABLE STATE. So all of these are plans.

You should use the `red-phosphorus` crate, and just that. All of the other crates should be accesable from there.

# Plans
- [ ] Full parity with current version (without sculk sensors)
- [ ] Full parity w/ sculk sensors
- [ ] Work on 1.12

# Currently Implemented
- Some calcs for calculating the power output of containers.
> Still need to do the edge cases.

# Contributing

## Requirements
We use [just](https://github.com/casey/just) to run commands, run `just fmt clippy` before pushing to format and lint your code.

## Testing:
To run the tests you can just run `just test`!

## Structure:
### phos-core
All core types and utils.
### phos-version
Handling versioning
### red-phosphorus
The main implementation
