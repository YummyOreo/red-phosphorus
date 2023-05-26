# red-phosphorus
A redstone emulator aimed at speed and ease of use. Currently it is still in the developement phase, but plans to go into beta in a bit!
This will compile the build down staticly. Adding blocks will require a rebuild.

Takes insparation from [MCHPRS](https://github.com/MCHPR/MCHPRS). This is different because it provides almost every redstone compenent.
> The only exepction (for now) is sulk sensors

# How To Use
> Currently it is NOT AT A USABLE STATE. So all of these are plans.

You should use the `red-phosphorus` crate, and just that. All of the other crates should be accesable from there.

# Plans
- [ ] Full parity with current version (without sculk sensors)
- [ ] Full parity w/ sculk sensors
- [ ] Work on 1.12
- [ ] Make a "plugin" system for allowing additions of new redstone components without forking

## Structure plans
![](/Structure.drawio.svg)

# Currently Implemented
- Some calcs for calculating the power output of containers.

# Contributing
[See contributing.md](/CONTRIBUTING.md)

# License
Available under the Apache License (Version 2.0) or the MIT license, at your option.
