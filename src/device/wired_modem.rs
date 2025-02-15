use std::collections::VecDeque;

use super::IODriverSimulator;
///! Diagram of a full-duplex device, probably modem
///! Is made to picture the idea of internal quques connectivities.
///!                                                                   
///!                  o             o                                        
///!                  | TX          | RX                               
///!  +---------------|-------------|--------+                                         
///!  | Modem Device  |             |        |                                         
///!  |       +-------+             |        |                                         
///!  |       |                     |        |                                         
///!  | (T byte to netw)  (T byte from netw) |                                         
///!  |       |                     |        |                                         
///!  |       +--<-- To net buffer <----+    |                                         
///!  |                             |   |    |                                         
///!  |                             |   |    |                                         
///!  |                             |   |    |                                         
///!  |    +<---<- From net buffer <+   |    |                                         
///!  |    |                            |    |                                         
///!  |    |                            |    |                                         
///!  |    |                            |    |                                         
///!  |    |                            |    |
///!  |  TX pin                     RX pin   |                                   
///!  +--------------------------------------+                                         
///!       |                            |                                               
///!       |                            |                                               
///!       |                            |                                               
///!       o                            o                                               
///!                                                                   
pub struct WiredModuleDriver {
    from_network_buffer: VecDeque<u8>,
    to_network_buffer: VecDeque<u8>,
    tick_byte_to_network: Option<u8>,
    tick_byte_from_network: Option<u8>,
}

impl WiredModuleDriver {
    pub fn new() -> Self {
        Self {
            from_network_buffer: VecDeque::new(),
            to_network_buffer: VecDeque::new(),
            tick_byte_to_network: None,
            tick_byte_from_network: None,
        }
    }
}

impl IODriverSimulator for WiredModuleDriver {
    fn get_from_device_network_side(&mut self) -> Option<u8> {
        self.tick_byte_to_network.clone()
    }

    fn put_to_device_network_side(&mut self, byte: u8) {
        self.tick_byte_from_network.replace(byte);
    }

    fn get_from_tx_pin(&mut self) -> Option<u8> {
        self.from_network_buffer.pop_front()
    }

    fn put_to_rx_pin(&mut self, byte: u8) {
        self.to_network_buffer.push_back(byte);
    }

    fn start_tick(&mut self) {
        self.tick_byte_to_network = None;
        self.tick_byte_from_network = None;

        if let Some(byte) = self.to_network_buffer.pop_front() {
            self.tick_byte_to_network = Some(byte);
        }
    }

    fn end_tick(&mut self) {
        if let Some(byte) = self.tick_byte_from_network.take() {
            self.from_network_buffer.push_back(byte);
        }
        self.tick_byte_to_network = None;
    }

    fn read_ready(&self) -> bool {
        !self.from_network_buffer.is_empty()
    }

    fn write_ready(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod wired_modem_device_tests {
    use super::*;

    #[test]
    fn test_emtpy_rx_to_network() {
        let mut modem_device = WiredModuleDriver::new();

        modem_device.start_tick();
        assert_eq!(modem_device.get_from_device_network_side(), None);
        modem_device.end_tick();
    }

    #[test]
    fn test_emtpy_tx_from_network() {
        let mut modem_device = WiredModuleDriver::new();

        modem_device.start_tick();
        modem_device.end_tick();

        assert_eq!(modem_device.get_from_tx_pin(), None);
    }

    #[test]
    fn test_rx_pin_to_network() {
        let mut modem_device = WiredModuleDriver::new();
        modem_device.put_to_rx_pin(b'a');
        modem_device.put_to_rx_pin(b'b');

        modem_device.start_tick();
        assert_eq!(modem_device.get_from_device_network_side(), Some(b'a'));
        modem_device.end_tick();

        modem_device.start_tick();
        assert_eq!(modem_device.get_from_device_network_side(), Some(b'b'));
        modem_device.end_tick();

        modem_device.start_tick();
        assert_eq!(modem_device.get_from_device_network_side(), None);
        modem_device.end_tick();
    }

    #[test]
    fn test_tx_pin_from_network() {
        let mut modem_device = WiredModuleDriver::new();
        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.end_tick();

        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'b');
        modem_device.end_tick();

        assert_eq!(modem_device.get_from_tx_pin(), Some(b'a'));
        assert_eq!(modem_device.get_from_tx_pin(), Some(b'b'));
        assert_eq!(modem_device.get_from_tx_pin(), None);
    }

    #[test]
    fn test_full_duplex_send_per_tick() {
        let mut modem_device = WiredModuleDriver::new();
        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.put_to_rx_pin(b'b');
        modem_device.end_tick();

        let byte_on_tx_pin = modem_device.get_from_tx_pin();

        modem_device.start_tick();
        assert_eq!(modem_device.get_from_device_network_side(), Some(b'b'));
        modem_device.end_tick();
        assert_eq!(byte_on_tx_pin, Some(b'a'));
    }

    // Test data collision with overwriting data per same tick
    #[test]
    fn test_data_collision_per_tick() {
        let mut modem_device = WiredModuleDriver::new();
        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.put_to_device_network_side(b'b');
        modem_device.end_tick();
        assert_eq!(modem_device.get_from_tx_pin(), Some(b'b'));
    }

    // Test read ready
    #[test]
    fn test_read_ready() {
        let mut modem_device = WiredModuleDriver::new();
        assert_eq!(modem_device.read_ready(), false);

        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.end_tick();

        assert_eq!(modem_device.read_ready(), true);
    }

    // Test write ready
    #[test]
    fn test_write_ready() {
        let modem_device = WiredModuleDriver::new();
        assert_eq!(modem_device.write_ready(), true);
    }
}
