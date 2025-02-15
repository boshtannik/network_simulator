use std::collections::HashMap;

use crate::{device::IODriverSimulator, ether::Ether};

pub struct Simulator<IODrv: IODriverSimulator> {
    ethers: HashMap<String, Ether<IODrv>>,
}

impl<IODrv: IODriverSimulator> Simulator<IODrv> {
    pub fn new() -> Self {
        Self {
            ethers: HashMap::new(),
        }
    }

    pub fn add_ether(&mut self, ether: Ether<IODrv>, name: String) {
        self.ethers.insert(name, ether);
    }

    pub fn add_device_to_ether(&mut self, ether_name: String, device: IODrv) {
        self.ethers
            .get_mut(&ether_name)
            .expect(format!("Ether with name: {} not found", ether_name).as_str())
            .add_device(device);
    }

    pub fn update(&mut self) {
        for ether in self.ethers.values_mut() {
            ether.update();
        }
    }
}

/*
Playground to make some drawings on the way i want to se usage of such simulator.

         Ether 1         Ether 2
[device 1       device 2]
                [device 2       device 3]

!!! Implementation embedded_io for RadioDriver needed!
!!! Probably `read_ready` and `write_ready` methods
!!! are still needed for Modem trait.

let mut radio_modem_1 = SimulatedRadioModem::new();
let mut radio_modem_2 = SimulatedRadioModem::new();
let mut radio_modem_3 = SimulatedRadioModem::new();

let mut mesh_node_1 = Node::new(NodeConfig{...});
let mut mesh_node_2 = Node::new(NodeConfig{...});
let mut mesh_node_3 = Node::new(NodeConfig{...});

let mut simulator = Simulator::new();
simulator.create_ether(String::from("Ether 1"))
simulator.create_ether(String::from("Ether 2"))

loop {
    let current_time = Instant::now()
        .duration_since(program_start_time)
        .as_millis() as ms;

    mesh_node_1.update(&mut radio_modem_1, current_time)
    mesh_node_2.update(&mut radio_modem_1, current_time)
    mesh_node_3.update(&mut radio_modem_1, current_time)

    simulator.get_ether_ref("Ether 1").simulate_for([&radio_modem_1, &radio_modem_2]);
    simulator.get_ether_ref("Ether 2").simulate_for([&radio_modem_2, &radio_modem_3]);
}
*/
