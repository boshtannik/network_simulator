# Network simulator
## Simple stupid network simulator
This simulator is made as ground tool to develop / test of my protocols.
Also it is easy to use and can be used it to write determenistic tests for your protocols
or write your own protocols.

This library simulates physical layer of networking like modems and ethers containing that modems.
Currently wireless ethers and wireless modems are supported the most.

## Functionality that the simulator provides:
* Lets develop protocols with no need for actual hardware.
* Lets buld automated testing scenarious.
* Simulates data collision when more than one modem are broadcasting into same environment.
* Supports groups of modems working simultaneously in one or several ethers at the time.
* One modem can occupy more than one ether. It lets to simulate scenario of data being transferred by chain of simulated devices.
* Can simulate scenario of modem being fell of the environment during broadcasting or being hot plugged to existing environment.
* Provides interface to write multithread code with using several modems in the same ether by different threads.
  As the modem can be clonned by call it's clone method and then sent into another thread. Internally modem shares the state with it's clones.
* You can use simulators update methods in your loop to have more determenistic simulaton or run simulator's internal update loop thread
  to force simulator update ethers and their modems automatically in the background.

## The project is pretty fresh, and is welcomed to be extended by pull requests.
* Virtual modems now supports only embedded-io traits, other traits are welcomed to be implemented also.

## License GPL v3.0
