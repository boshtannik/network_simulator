use crate::device::IODriverSimulator;

pub struct Ether<'a, IODrv: IODriverSimulator> {
    devices: &'a mut [&'a mut IODrv],
}

impl<'a, IODrv: IODriverSimulator> Ether<'a, IODrv> {
    pub fn new(devices: &'a mut [&'a mut IODrv]) -> Self {
        Self { devices }
    }

    /// Gets the broadcasted byte from latest broadasting device.
    /// That is the place where the data collision is possible.
    fn get_current_byte(&mut self) -> Option<u8> {
        let mut result: Option<u8> = None;

        for device in self.devices.iter_mut() {
            if let Some(byte) = device.get_from_device_network_side() {
                result = Some(byte);
            }
        }

        result
    }

    pub fn simulate(&mut self) {
        if let Some(current_byte) = self.get_current_byte() {
            for device in self.devices.iter_mut() {
                device.put_to_device_network_side(current_byte);
            }
        }
    }
}
