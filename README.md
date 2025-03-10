# proto-lab  
## **Lightweight & Deterministic Network Protocol Simulator**  

**proto-lab** is a simple yet powerful simulator designed for developing and testing **custom networking protocols** without requiring physical hardware. It provides a deterministic environment for automated testing, protocol experimentation, and multi-threaded simulations.  

This library simulates the **physical layer** of networking, including **modems** and **ethers** that connect them.  
📢 **Note:** proto-lab **only** handles raw **byte transmission** between modems—it does **not** implement IP, TCP, UDP, or any higher-layer protocols. Instead, it serves as a **foundation** for building and testing such protocols.  

### **Key Features**  
- 🛠 **Develop & test protocols** without needing real hardware.  
- 🧪 **Automate testing scenarios** with full deterministic control.  
- ⚡ **Simulate data collisions** when multiple modems transmit in the same ether at the same tick.  
- 📡 **Multi-ether support** – Modems can operate across multiple ethers at once.  
- 🔗 **Chained data transfer** – Simulate multi-hop data relay across devices.  
- 🔄 **Dynamic topology** – Simulate modems being **hot-plugged** or **removed** mid-transmission.  
- 🧵 **Thread-safe modem cloning** – Clone modems to different threads while sharing state.  
- ⏳ **Flexible tick-based updates** – Control simulation timing manually or run in **automatic background mode**.  

---

## **How It Works**  
proto-lab operates in **discrete ticks**, ensuring **deterministic simulation behavior**:  

1. **Start a Tick** → `start_tick()` initializes transmission & listening states for modems.  
2. **Simulate a Step** → `simulate()` processes modem transmissions and delivers bytes to listening modems.  
3. **End a Tick** → `stop_tick()` finalizes transmission, queues received bytes, and prepares for the next step.  

You can control ticks manually or let proto-lab handle updates via `start_simulation_thread()` and `stop_simulation_thread()`.  

### **Simulation Process in Detail**  

    start_tick() - Begins a new tick cycle:
        Iterates through all modems in ethers.
        Each sender modem prepares bytes for transmission.
        Each receiver modem enters listening mode.

    simulate() - Runs the simulation step:
        Identifies broadcasting modems.
        Transfers broadcasted bytes to all listening modems.

    stop_tick() - Ends the tick cycle:
        Iterates through all modems in ethers.
        Each sender modem stops broadcasting.
        Each receiver modem queues received bytes.


---

## **Full Example: Simulating a Mesh Network**
proto-lab seamlessly integrates into complex network simulations, including **mesh protocols**.  
Below is a simplified example demonstrating **multi-ether communication** and **node interaction**:

```rust
use proto_lab::{NetworkSimulator, WirelessModemFake};

fn main() {
    let mut simulator = NetworkSimulator::new(1);

    simulator.create_ether("1");
    simulator.create_ether("2");

    let mut driver_1 = WirelessModemFake::new("1");
    let mut driver_2 = WirelessModemFake::new("2");
    let mut driver_3 = WirelessModemFake::new("3");

    simulator.get_ether("1").unwrap().register_driver(driver_1.clone());
    simulator.get_ether("1").unwrap().register_driver(driver_2.clone());

    simulator.get_ether("2").unwrap().register_driver(driver_2.clone());
    simulator.get_ether("2").unwrap().register_driver(driver_3.clone());

    simulator.start_simulation_thread();

    // Example: Simulating communication between nodes...
    // For full example look into examples directory...

    simulator.stop_simulation_thread();

    println!("Simulation completed!");
}
```

Getting Started

    Add proto-lab to your Cargo.toml:

    [dependencies]
    proto-lab = "0.1"

    Explore the API and start building your protocol simulations.
    Run tests and fine-tune your protocol logic with deterministic control.

Contribute

proto-lab is a fresh project, and contributions are welcome! Feel free to submit pull requests, report issues, or suggest new features.

🛠 Note: Currently, virtual modems support only embedded-io traits. Contributions adding support for other traits are encouraged!
License

proto-lab is released under the GPL v3.0 license.
