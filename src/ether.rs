use std::sync::{Arc, Mutex};

use crate::device::IODriverSimulator;

pub struct Ether<'a, IODrv: IODriverSimulator> {
    devices: &'a [Arc<Mutex<IODrv>>],
}

impl<'a, IODrv: IODriverSimulator> Ether<'a, IODrv> {
    pub fn new(devices: &'a [Arc<Mutex<IODrv>>]) -> Self {
        Self { devices }
    }

    /// Gets the broadcasted byte from latest broadasting device.
    /// That is the place where the data collision is possible.
    fn get_current_byte(&mut self) -> Option<u8> {
        let mut result: Option<u8> = None;

        for device in self.devices.iter() {
            if let Some(byte) = device
                .lock()
                .expect("Fail to get lock on simulated device driver")
                .get_from_device_network_side()
            {
                result = Some(byte);
            }
        }

        result
    }

    // For better cross ether interference (producing data collisions) it is better to
    // start tick for all ethers at once, then simulate each ether then do end tick
    // for all ethers at once.
    pub fn start_tick(&mut self) {
        for device in self.devices.iter() {
            device
                .lock()
                .expect("Fail to get lock to start modem tick")
                .start_tick();
        }
    }

    // For better cross ether interference (producing data collisions) it is better to
    // start tick for all ethers at once, then simulate each ether then do end tick
    // for all ethers at once.
    pub fn end_tick(&mut self) {
        for device in self.devices.iter() {
            device
                .lock()
                .expect("Fail to get lock to end modem tick")
                .end_tick();
        }
    }

    pub fn simulate(&mut self) {
        if let Some(current_byte) = self.get_current_byte() {
            for device in self.devices.iter() {
                device
                    .lock()
                    .expect("Fail to get lock on simulated device driver")
                    .put_to_device_network_side(current_byte);
            }
        }
    }
}
