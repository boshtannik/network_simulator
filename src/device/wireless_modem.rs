use std::collections::VecDeque;

use super::IODriverSimulator;

enum AntennaState {
    Transmit(u8),
    Receive(u8),
    Idle,
}

//  Diagram of a half-duplex device, probably radio driver, or single wired transceiver
//  Is made to picture the idea of internal quques connectivities.
//
//```
//                (Network side)
//                       \|/
//                        |  - Antenna
//                        |
//   +--------------------|-----------------+
//   | Wireless Device     +-+-+               |
//   |                  \   \               |
//   |        +------>--+   +-->--+         |
//   |        |                   |         |
//   |        +-<-   to_ether  -<---+       |
//   |                            | |       |
//   |                            | |       |
//   |     ---<---   from_ether <-+ |       |
//   |     |                        |       |
//   |     |                        |       |
//   |     |                        |       |
//   |   TX pin                 RX pin      |
//   +--------------------------------------+
//        |                     |
//        |      (Pins side)    |
//        |                     |
//        o                     o
//```
//
enum TickState {
    InTick,
    OffTick,
}

pub struct WirelessModemFake {
    antennta_state: AntennaState,
    from_antenna_buffer: VecDeque<u8>,
    to_antenna_buffer: VecDeque<u8>,
    tick_state: TickState,
    name: String,
}

impl WirelessModemFake {
    pub fn new(name: &str) -> Self {
        WirelessModemFake {
            antennta_state: AntennaState::Idle,
            from_antenna_buffer: VecDeque::new(),
            to_antenna_buffer: VecDeque::new(),
            tick_state: TickState::OffTick,
            name: String::from(name),
        }
    }
}

impl IODriverSimulator for WirelessModemFake {
    fn get_from_device_network_side(&mut self) -> Option<u8> {
        match self.tick_state {
            TickState::OffTick => None,
            TickState::InTick => match self.antennta_state {
                AntennaState::Transmit(byte) => Some(byte),
                _ => None,
            },
        }
    }

    fn put_to_device_network_side(&mut self, byte: u8) {
        match self.tick_state {
            TickState::OffTick => (),
            TickState::InTick => match self.antennta_state {
                AntennaState::Transmit(_) => (),
                AntennaState::Idle | AntennaState::Receive(_) => {
                    self.antennta_state = AntennaState::Receive(byte)
                }
            },
        }
    }

    fn get_from_tx_pin(&mut self) -> Option<u8> {
        self.from_antenna_buffer.pop_front()
    }

    fn put_to_rx_pin(&mut self, byte: u8) {
        self.to_antenna_buffer.push_back(byte);
    }

    fn start_tick(&mut self) {
        match self.tick_state {
            TickState::InTick => (),
            TickState::OffTick => {
                self.antennta_state = match self.to_antenna_buffer.pop_front() {
                    Some(byte) => AntennaState::Transmit(byte),
                    _ => AntennaState::Idle,
                };

                self.tick_state = TickState::InTick;
            }
        }
    }

    fn end_tick(&mut self) {
        match self.tick_state {
            TickState::OffTick => (),
            TickState::InTick => {
                match self.antennta_state {
                    AntennaState::Receive(byte) => {
                        self.from_antenna_buffer.push_back(byte);
                    }
                    _ => (),
                }
                self.antennta_state = AntennaState::Idle;

                self.tick_state = TickState::OffTick;
            }
        }
    }

    fn readable(&self) -> bool {
        !self.from_antenna_buffer.is_empty()
    }

    fn writable(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl embedded_io::ErrorType for WirelessModemFake {
    type Error = core::convert::Infallible;
}

impl embedded_io::ReadReady for WirelessModemFake {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(self.readable())
    }
}

impl embedded_io::Read for WirelessModemFake {
    // Read from WirelessModemFake into buf
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

impl embedded_io::Write for WirelessModemFake {
    // Write from buf into WirelessModemFake
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
mod radio_modem_device_tests {
    use super::*;

    #[test]
    fn test_half_duplex_send_per_tick() {
        let mut modem_device = WirelessModemFake::new("");
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
        let mut modem_device = WirelessModemFake::new("");
        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.put_to_device_network_side(b'b');
        modem_device.put_to_device_network_side(b'c');
        modem_device.end_tick();
        assert_eq!(modem_device.get_from_tx_pin(), Some(b'c'));
    }
}
