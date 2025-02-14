use std::collections::HashMap;

use crate::{device::IODriver, ether::Ether};

pub struct Simulator<IODrv: IODriver> {
    ethers: HashMap<String, Ether<IODrv>>,
}

impl<IODrv: IODriver> Simulator<IODrv> {
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
