# red-phosphorus
A redstone emulator aimed at speed and ease of use. Currently it is still in the developement phase, but plans to go into beta in a bit!

# How To Use
> Currently it is NOT AT A USABLE STATE. So all of these are plans.

You should use the `red-phosphorus` crate, and just that. All of the other crates should be accesable from there.

# Plans
- [ ] Full parity with current version (without sculk sensors)
- [ ] Full parity w/ sculk sensors
- [ ] Work on 1.12
- [ ] Make a "plugin" system for allowing additions of new redstone components without forking

## Structure plans
![](/docs/Structure.drawio.svg)

# Currently Implemented
- Some calcs for calculating the power output of containers.

# Contributing

## Requirements
We use [just](https://github.com/casey/just) to run commands, run `just fmt clippy` before pushing to format and lint your code.

## Testing:
To run the tests you can just run `just test`!

