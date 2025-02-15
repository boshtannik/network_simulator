use std::collections::VecDeque;

use super::IODriverSimulator;

enum AntennaState {
    Transmit(u8),
    Receive(u8),
    Idle,
}

///! Diagram of a half-duplex device, probably radio driver, or single wired transceiver
///! Is made to picture the idea of internal quques connectivities.
///!                                                                   
///!               (Network side)                                      
///!                      \|/                                                    
///!                       |  - Antenna                                                
///!                       |                                                     
///!  +--------------------|-----------------+                                         
///!  | Radio Device     +-+-+               |                                         
///!  |                  \   \               |                                         
///!  |        +------>--+   +-->--+         |                                         
///!  |        |                   |         |                                         
///!  |        +-<-   to_ether  -<---+       |                                         
///!  |                            | |       |                                         
///!  |                            | |       |                                         
///!  |     ---<---   from_ether <-+ |       |                                         
///!  |     |                        |       |                                         
///!  |     |                        |       |                                         
///!  |     |                        |       |                                         
///!  |   TX pin                 RX pin      |                                   
///!  +--------------------------------------+                                         
///!       |                     |                                               
///!       |      (Pins side)    |                                               
///!       |                     |                                               
///!       o                     o                                               
///!                                                                   
pub struct RadioModuleDriver {
    antennta_state: AntennaState,
    from_antenna_buffer: VecDeque<u8>,
    to_antenna_buffer: VecDeque<u8>,
}

impl RadioModuleDriver {
    fn new() -> Self {
        RadioModuleDriver {
            antennta_state: AntennaState::Idle,
            from_antenna_buffer: VecDeque::new(),
            to_antenna_buffer: VecDeque::new(),
        }
    }
}

impl IODriverSimulator for RadioModuleDriver {
    fn get_from_device_network_side(&mut self) -> Option<u8> {
        match self.antennta_state {
            AntennaState::Transmit(byte) => Some(byte),
            _ => None,
        }
    }

    fn put_to_device_network_side(&mut self, byte: u8) {
        match self.antennta_state {
            AntennaState::Transmit(_) => (),
            AntennaState::Idle | AntennaState::Receive(_) => {
                self.antennta_state = AntennaState::Receive(byte)
            }
        }
    }

    /// gets byte from tx pin of the device pins side
    fn get_from_tx_pin(&mut self) -> Option<u8> {
        self.from_antenna_buffer.pop_front()
    }

    /// puts byte to rx pin of the device pins side
    fn put_to_rx_pin(&mut self, byte: u8) {
        self.to_antenna_buffer.push_back(byte);
    }

    // On start tick the device checks if it has some byte to
    // send to ether from buffer. If has - then put
    // device in sending state of that byte
    // Else - put device in Idle state to be ready to receive byte.
    fn start_tick(&mut self) {
        self.antennta_state = match self.to_antenna_buffer.pop_front() {
            Some(byte) => AntennaState::Transmit(byte),
            _ => AntennaState::Idle,
        };
    }

    // On end tick device checks if it has some byte received from ether
    // If has - then put that received byte into from antenna buffer.
    // Put device in state of idle
    fn end_tick(&mut self) {
        match self.antennta_state {
            AntennaState::Receive(byte) => {
                self.from_antenna_buffer.push_back(byte);
            }
            _ => (),
        }
        self.antennta_state = AntennaState::Idle;
    }

    fn read_ready(&self) -> bool {
        !self.from_antenna_buffer.is_empty()
    }

    fn write_ready(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod radio_modem_device_tests {
    use super::*;

    #[test]
    fn test_emtpy_to_network() {
        let mut modem_device = RadioModuleDriver::new();

        modem_device.start_tick();
        assert_eq!(modem_device.get_from_device_network_side(), None);
        modem_device.end_tick();
    }

    #[test]
    fn test_emtpy_from_network() {
        let mut modem_device = RadioModuleDriver::new();

        modem_device.start_tick();
        modem_device.end_tick();

        assert_eq!(modem_device.get_from_tx_pin(), None);
    }

    #[test]
    fn test_get_from_network() {
        let mut modem_device = RadioModuleDriver::new();
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
    fn test_put_to_network() {
        let mut modem_device = RadioModuleDriver::new();
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
    fn test_half_duplex_send_per_tick() {
        let mut modem_device = RadioModuleDriver::new();
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
        let mut modem_device = RadioModuleDriver::new();
        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.put_to_device_network_side(b'b');
        modem_device.put_to_device_network_side(b'c');
        modem_device.end_tick();
        assert_eq!(modem_device.get_from_tx_pin(), Some(b'c'));
    }

    // Test read ready
    #[test]
    fn test_read_ready() {
        let mut modem_device = RadioModuleDriver::new();
        assert_eq!(modem_device.read_ready(), false);

        modem_device.start_tick();
        modem_device.put_to_device_network_side(b'a');
        modem_device.end_tick();

        assert_eq!(modem_device.read_ready(), true);
    }

    // Test write ready
    #[test]
    fn test_write_ready() {
        let modem_device = RadioModuleDriver::new();
        assert_eq!(modem_device.write_ready(), true);
    }
}
