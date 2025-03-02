use std::sync::{Arc, Mutex};

use crate::{device::IODriverSimulator, WirelessModemFake};

pub struct EtherSimulator {
    name: String,
    devices: Arc<Mutex<Vec<WirelessModemFake>>>,
}

impl EtherSimulator {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            devices: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn create_driver(name: &str) -> WirelessModemFake {
        WirelessModemFake::new(name)
    }

    pub fn register_driver(&mut self, driver: WirelessModemFake) {
        let mut devices = self.devices.lock().expect("Fail to get lock on devices");
        devices.push(WirelessModemFake::clone(&driver));
    }

    pub fn unregister_driver(&mut self, name: &str) {
        let mut devices = self.devices.lock().expect("Fail to get lock on devices");

        loop {
            let mut index_to_remove: Option<_> = None;

            for (i, device) in devices.iter_mut().enumerate() {
                if device.get_name() == name {
                    index_to_remove.replace(i);
                }
            }

            match index_to_remove {
                Some(i) => devices.remove(i),
                None => break,
            };
        }
    }

    pub fn get_driver(&self, name: &str) -> Option<WirelessModemFake> {
        let devices = self.devices.lock().expect("Fail to get lock on devices");

        for device in devices.iter() {
            if device.get_name() == name {
                return Some(WirelessModemFake::clone(&device));
            }
        }
        None
    }

    /// Gets the broadcasted byte from latest broadasting device.
    /// That is the place where the data collision is possible.
    fn get_current_byte(&self) -> Option<u8> {
        let mut result: Option<u8> = None;
        let devices = self.devices.lock().expect("Fail to get lock on devices");

        for device in devices.iter() {
            if let Some(byte) = device.get_from_device_network_side() {
                result = Some(byte);
            }
        }

        result
    }

    // For better cross ether interference (producing data collisions) it is better to
    // start tick for all ethers at once, then simulate each ether then do end tick
    // for all ethers at once.
    pub fn start_tick(&self) {
        let devices = self.devices.lock().expect("Fail to get lock on devices");
        for device in devices.iter() {
            device.start_tick();
        }
    }

    // For better cross ether interference (producing data collisions) it is better to
    // start tick for all ethers at once, then simulate each ether then do end tick
    // for all ethers at once.
    pub fn end_tick(&self) {
        let devices = self.devices.lock().expect("Fail to get lock on devices");
        for device in devices.iter() {
            device.end_tick();
        }
    }

    pub fn simulate(&self) {
        let current_byte = self.get_current_byte();

        let devices = self.devices.lock().expect("Fail to get lock on devices");

        if let Some(current_byte) = current_byte {
            for device in devices.iter() {
                device.put_to_device_network_side(current_byte);
            }
        }
    }

    pub fn clone(&self) -> EtherSimulator {
        EtherSimulator {
            name: String::from(&self.name),
            devices: Arc::clone(&self.devices),
        }
    }
}
