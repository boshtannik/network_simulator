use std::collections::VecDeque;

use super::IODriverSimulator;
// Diagram of a full-duplex device, probably modem
// Is made to picture the idea of internal quques connectivities.
//
//```
//                  o             o
//                  | TX          | RX
//  +---------------|-------------|--------+
//  | Modem Device  |             |        |
//  |       +-------+             |        |
//  |       |                     |        |
//  | (T byte to netw)  (T byte from netw) |
//  |       |                     |        |
//  |       +--<-- To net buffer <----+    |
//  |                             |   |    |
//  |                             |   |    |
//  |                             |   |    |
//  |    +<---<- From net buffer <+   |    |
//  |    |                            |    |
//  |    |                            |    |
//  |    |                            |    |
//  |    |                            |    |
//  |  TX pin                     RX pin   |
//  +--------------------------------------+
//       |                            |
//       |                            |
//       |                            |
//       o                            o
//```

enum TickState {
    InTick,
    OffTick,
}

pub struct WiredModemFake {
    from_network_buffer: VecDeque<u8>,
    to_network_buffer: VecDeque<u8>,
    tick_byte_to_network: Option<u8>,
    tick_byte_from_network: Option<u8>,
    tick_state: TickState,
}

impl WiredModemFake {
    pub fn new() -> Self {
        Self {
            from_network_buffer: VecDeque::new(),
            to_network_buffer: VecDeque::new(),
            tick_byte_to_network: None,
            tick_byte_from_network: None,
            tick_state: TickState::OffTick,
        }
    }
}

impl IODriverSimulator for WiredModemFake {
    /// Testing to be sent to network
    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// radio_driver.start_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), None);
    /// radio_driver.end_tick();
    ///
    /// radio_driver.put_to_rx_pin(b'a');
    ///
    /// radio_driver.start_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), Some(b'a'));
    /// radio_driver.end_tick();
    /// ```
    fn get_from_device_network_side(&mut self) -> Option<u8> {
        match self.tick_state {
            TickState::InTick => self.tick_byte_to_network.clone(),
            TickState::OffTick => None,
        }
    }

    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.get_from_tx_pin(), None);
    ///
    /// radio_driver.start_tick();
    /// radio_driver.put_to_device_network_side(b'a');
    /// radio_driver.end_tick();
    ///
    /// assert_eq!(radio_driver.get_from_tx_pin(), Some(b'a'));
    ///
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.get_from_tx_pin(), None);
    /// ```
    /// Testing some data put to queues to be sent to network
    fn put_to_device_network_side(&mut self, byte: u8) {
        match self.tick_state {
            TickState::InTick => {
                self.tick_byte_from_network.replace(byte);
            }
            TickState::OffTick => (),
        };
    }

    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.get_from_tx_pin(), None);
    ///
    /// radio_driver.start_tick();
    /// radio_driver.put_to_device_network_side(b'a');
    /// radio_driver.end_tick();
    ///
    /// assert_eq!(radio_driver.get_from_tx_pin(), Some(b'a'));
    ///
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.get_from_tx_pin(), None);
    /// ```
    /// Testing some data put to queues to be sent to network
    fn get_from_tx_pin(&mut self) -> Option<u8> {
        self.from_network_buffer.pop_front()
    }

    /// Testing to be sent to network
    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// radio_driver.start_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), None);
    /// radio_driver.end_tick();
    ///
    /// radio_driver.put_to_rx_pin(b'a');
    ///
    /// radio_driver.start_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), Some(b'a'));
    /// radio_driver.end_tick();
    /// ```
    /// Testing some data put to queues to be sent to network
    fn put_to_rx_pin(&mut self, byte: u8) {
        self.to_network_buffer.push_back(byte);
    }

    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    ///
    /// assert_eq!(radio_driver.get_from_device_network_side(), None);
    ///
    /// radio_driver.put_to_rx_pin(b'a');
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), None);
    ///
    /// radio_driver.put_to_rx_pin(b'c');
    /// radio_driver.start_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), Some(b'c'));
    /// radio_driver.end_tick();
    /// ```
    fn start_tick(&mut self) {
        match self.tick_state {
            TickState::InTick => (),
            TickState::OffTick => {
                self.tick_byte_to_network = None;
                self.tick_byte_from_network = None;

                if let Some(byte) = self.to_network_buffer.pop_front() {
                    self.tick_byte_to_network = Some(byte);
                }

                self.tick_state = TickState::InTick;
            }
        }
    }

    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// assert_eq!(radio_driver.get_from_device_network_side(), None);
    ///
    /// radio_driver.put_to_rx_pin(b'a');
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), None);
    ///
    /// radio_driver.put_to_rx_pin(b'c');
    /// radio_driver.start_tick();
    /// assert_eq!(radio_driver.get_from_device_network_side(), Some(b'c'));
    /// radio_driver.end_tick();
    /// ```
    fn end_tick(&mut self) {
        match self.tick_state {
            TickState::OffTick => (),
            TickState::InTick => {
                if let Some(byte) = self.tick_byte_from_network.take() {
                    self.from_network_buffer.push_back(byte);
                }
                self.tick_byte_to_network = None;

                self.tick_state = TickState::OffTick;
            }
        }
    }

    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// radio_driver.start_tick();
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.readable(), false);
    /// radio_driver.start_tick();
    /// radio_driver.put_to_device_network_side(b'a');
    /// radio_driver.end_tick();
    /// assert_eq!(radio_driver.readable(), true);
    /// ```
    fn readable(&self) -> bool {
        !self.from_network_buffer.is_empty()
    }

    /// ```
    /// use network_simulator::{IODriverSimulator, WiredModemFake};
    /// let mut radio_driver = WiredModemFake::new();
    /// assert_eq!(radio_driver.writable(), true);
    /// ```
    fn writable(&self) -> bool {
        true
    }
}

impl embedded_io::ErrorType for WiredModemFake {
    type Error = core::convert::Infallible;
}

impl embedded_io::ReadReady for WiredModemFake {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.readable())
    }
}

impl embedded_io::Read for WiredModemFake {
    // Read from WirelessModuleDriver into buf
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut count_red: usize = 0;
        for buf_vancant_place in buf.iter_mut() {
            if let Some(byte) = self.get_from_tx_pin() {
                *buf_vancant_place = byte;
                count_red += 1;
            }
        }
        Ok(count_red)
    }
}

impl embedded_io::Write for WiredModemFake {
    // Write from buf into WirelessModuleDriver
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut count_written: usize = 0;
        for b in buf {
            self.put_to_rx_pin(*b);
            count_written += 1;
        }
        Ok(count_written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod wired_modem_device_tests {
    use super::*;

    #[test]
    fn test_full_duplex_send_per_tick() {
        let mut modem_device = WiredModemFake::new();
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
        let mut modem_device = WiredModemFake::new();
        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.put_to_device_network_side(b'b');
        modem_device.end_tick();
        assert_eq!(modem_device.get_from_tx_pin(), Some(b'b'));
    }
}
