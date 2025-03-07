# Network simulator
## Simple stupid network simulator
This simulator is made as ground tool to develop / test of my protocols.
Also it is easy to use and can be used it to write determenistic tests for your protocols
or write your own protocols.

This library simulates physical layer of networking like modems and ethers containing that modems.
Note! That this simulator simulates only bytes transfering between the modems, and have no thing
in common with such protocols as IP, TCP, UDP, etc. It is made to build or test ones.
Currently wireless ethers and wireless modems are supported the most.

## Functionality that the simulator provides:
* Lets develop protocols with no need for actual hardware.
* Lets buld automated testing scenarious.
* Simulates data collision when more than one modem are broadcasting into same environment at the same tick.
* Supports groups of modems working simultaneously in one or several ethers at the time.
* One modem can occupy more than one ether. It lets to simulate scenario of data being transferred by chain of simulated devices.
* Can simulate scenario of modem being fell of the environment during broadcasting or being hot plugged to existing environment.
* Provides interface to write multithread code with using several modems in the same ether by different threads.
  As the modem can be clonned by call it's clone method and then sent into another thread. Internally modem shares the state with it's clones.
* You can use simulators update methods in your loop to have more determenistic simulaton or run simulator's internal update loop thread
  to force simulator update ethers and their modems automatically in the background.

## Usage
To tell one part of simulated time from other - tick is invented and it is essential.
The use of simulator is next:

Ether simulator transfers data between modems only in tick event.
```
* start tick - call simulators start_tick() method. It does the following:
                  * iterates trough all modems in ethers.
                  * For each sender modem says that modem can grab stacked bytes that are ready to be sent.
                  * For each receiver modem says that modem goes into listening state.
* simulate    - then you call simulate() method of simulator. this does the follwing:
                  * Looks for modems are currently broadcasting byte, grabs that byte and transfers it to other modems that are in listening state.
* end tick    - then you can call simulators stop_tick() method to stop tick. It does the following:
                  * iterates trough all modems in ethers.
                  * For each sender modem says that modem to stop broadcasting byte of that tick.
                  * For each receiver modem says that modem can grab received byte and store it into received bytes queue.
```
You can call those methods by yourself or call `start_simulation_thread()` and `stop_simulation_thread()` respectively to make simulator do all job described above automatically.

Simulator contains ethers.
Ethers contains modems.

To use it.
1. You shall create simulator.
2. You shall create ether.
3. You shall create modem.
4. You shall register your created modem in the ethers you want it to share.
5. You shall update the simulator ticks by one of provided strategies above.

## The project is pretty fresh, and is welcomed to be extended by pull requests.
* Virtual modems now supports only embedded-io traits, other traits are welcomed to be implemented also.

## License GPL v3.0
