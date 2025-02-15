use crate::device::IODriverSimulator;

pub struct Ether<IODrv: IODriverSimulator> {
    devices: Vec<IODrv>,
}

impl<IODrv: IODriverSimulator> Ether<IODrv> {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: IODrv) {
        self.devices.push(device);
    }

    /// Gets the broadcasted byte from any device.
    /// That is the place where the collisions are simulated.
    fn get_current_byte(&mut self) -> Option<u8> {
        let mut result: Option<u8> = None;

        for device in self.devices.iter_mut() {
            if let Some(byte) = device.get_from_device_network_side() {
                result = Some(byte);
            }
        }

        result
    }

    fn start_tick(&mut self) {
        for device in self.devices.iter_mut() {
            device.start_tick();
        }
    }

    fn end_tick(&mut self) {
        for device in self.devices.iter_mut() {
            device.end_tick();
        }
    }

    pub fn update(&mut self) {
        self.start_tick();
        if let Some(current_byte) = self.get_current_byte() {
            for device in self.devices.iter_mut() {
                device.put_to_device_network_side(current_byte);
            }
        }
        self.end_tick();
    }
}
