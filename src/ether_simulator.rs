use std::sync::{Arc, Mutex};

use crate::{device::IODriverSimulator, WirelessModemFake};

pub struct EtherSimulator<IODrv: IODriverSimulator> {
    name: String,
    devices: Vec<Arc<Mutex<IODrv>>>,
}

impl<'a, IODrv: IODriverSimulator> EtherSimulator<IODrv> {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            devices: vec![],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn create_driver(&mut self, name: &str) -> Arc<Mutex<WirelessModemFake>>
    where
        IODrv: IODriverSimulator,
    {
        Arc::new(Mutex::new(WirelessModemFake::new(name)))
    }

    pub fn register_driver(&mut self, driver: Arc<Mutex<IODrv>>) {
        self.devices.push(driver);
    }

    pub fn unregister_driver(&mut self, name: &str) {
        loop {
            let mut index_to_remove: Option<_> = None;

            for (i, device) in self.devices.iter_mut().enumerate() {
                if device.lock().unwrap().get_name() == name {
                    index_to_remove.replace(i);
                }
            }

            match index_to_remove {
                Some(i) => self.devices.remove(i),
                None => break,
            };
        }
    }

    pub fn get_driver(&mut self, name: &str) -> Option<Arc<Mutex<IODrv>>> {
        for device in self.devices.iter() {
            if device.lock().unwrap().get_name() == name {
                return Some(Arc::clone(device));
            }
        }
        None
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
