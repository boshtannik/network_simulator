# Network simulator
## Simple stupid network simulator
This simulator is made as ground tool for develop / testing of my protocols.
This library simulates physical layer of networking both wired or wireless.
I use it to write automated tests for my already existing protocol, and probably
i can use it to build new ones.

## What it can do
* It lets develop protocols with no need for actual hardware.
* It lets buld automated testing scenarious.
* It hanldes data collision when more than one modem are broadcasting into same environment.
* It supports groups of modems working simultaneously in one or several environments at the time.
* It supports simulating modem being fell of the environment or being hot plugged to existing environment.

## The project is pretty fresh, and is welcomed to be extended by pull requests.
* Virtual modems now supports only embedded-io traits, other traits are welcomed to be implemented also.

## License GPL v3.0
