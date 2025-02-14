use std::collections::VecDeque;

pub trait IODriver {
    // Antenna interfaces
    fn get_from_device_network_side(&mut self) -> Option<u8>;
    fn put_to_device_network_side(&mut self, byte: u8);

    // Transceiver interfaces
    fn get_from_tx_pin(&mut self) -> Option<u8>;
    fn put_to_rx_pin(&mut self, byte: u8);

    // Time quantification interfaces
    fn start_tick(&mut self);
    fn end_tick(&mut self);
}

pub enum DeviceState {
    TX(u8),
    RX(u8),
    Idle,
}

///! Diagram of a half-duplex device, probably radio driver, or single wired transceiver
///! Is made to picture the idea of internal quques connectivities.
///!                                                                   
///!                      \|/                                                    
///!                       |  - Antenna                                                
///!                       |                                                     
///!  +--------------------|-----------------+                                         
///!  | Radio Device      / \                |                                         
///!  |        +------>---+ +--->--+         |                                         
///!  |        |                   |         |                                         
///!  |        +-<-TX_to_ether --<---+       |                                         
///!  |                            | |       |                                         
///!  |                            | |       |                                         
///!  |     ---<---RX_from_ether <-+ |       |                                         
///!  |     |                        |       |                                         
///!  |     |                        |       |                                         
///!  |     |                        |       |                                         
///!  |   TX pin                 RX pin      |                                   
///!  +--------------------------------------+                                         
///!       |                     |                                               
///!       |                     |                                               
///!       |                     |                                               
///!       o                     o                                               
///!                                                                   
pub struct RadioModuleDriver {
    device_state: DeviceState,
    rx_from_ether_buffer: VecDeque<u8>,
    tx_to_ether_buffer: VecDeque<u8>,
}

impl IODriver for RadioModuleDriver {
    fn get_from_device_network_side(&mut self) -> Option<u8> {
        match self.device_state {
            DeviceState::TX(byte) => Some(byte),
            _ => None,
        }
    }

    fn put_to_device_network_side(&mut self, byte: u8) {
        match self.device_state {
            DeviceState::TX(_) => (),
            DeviceState::Idle | DeviceState::RX(_) => self.device_state = DeviceState::RX(byte),
        }
    }

    /// gets byte from tx pin of the device
    fn get_from_tx_pin(&mut self) -> Option<u8> {
        self.rx_from_ether_buffer.pop_front()
    }

    /// puts byte to rx pin of the device
    fn put_to_rx_pin(&mut self, byte: u8) {
        self.tx_to_ether_buffer.push_back(byte);
    }

    // On start tick the device should check if it has some byte to
    // send to ether from buffer into the ether If has - then put
    // device in state of sending of that byte
    // Else - put device in Idle state to be ready to receive byte.
    fn start_tick(&mut self) {
        if let Some(byte) = self.tx_to_ether_buffer.pop_front() {
            self.device_state = DeviceState::TX(byte);
        } else {
            self.device_state = DeviceState::Idle;
        }
    }

    // On end tick device should check if it has some byte received from ether
    // If has - then put that received byte into rx from ether buffer.
    // Put device in state of idle
    fn end_tick(&mut self) {
        match self.device_state {
            DeviceState::RX(byte) => {
                self.rx_from_ether_buffer.push_back(byte);
            }
            _ => (),
        }
        self.device_state = DeviceState::Idle;
    }
}

///! Diagram of a full-duplex device, probably modem
///! Is made to picture the idea of internal quques connectivities.
///!                                                                   
///!                  o             o                                        
///!                  | TX          | RX                               
///!  +---------------|-------------|--------+                                         
///!  | Modem Device  |             |        |                                         
///!  |       +-------+             |        |                                         
///!  |       |                     |        |                                         
///!  |       |                     |        |                                         
///!  |       |                     |        |                                         
///!  |       +--<---TX_to_network <----+    |                                         
///!  |                             |   |    |                                         
///!  |                             |   |    |                                         
///!  |                             |   |    |                                         
///!  |    +<---<- RX_from_network <+   |    |                                         
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
    rx_from_network_buffer: VecDeque<u8>,
    tx_to_network_buffer: VecDeque<u8>,
    current_byte_to_network: Option<u8>,
    current_byte_from_network: Option<u8>,
}

impl WiredModuleDriver {
    pub fn new() -> Self {
        Self {
            rx_from_network_buffer: VecDeque::new(),
            tx_to_network_buffer: VecDeque::new(),
            current_byte_to_network: None,
            current_byte_from_network: None,
        }
    }
}

impl IODriver for WiredModuleDriver {
    fn get_from_device_network_side(&mut self) -> Option<u8> {
        self.current_byte_to_network.clone()
    }

    fn put_to_device_network_side(&mut self, byte: u8) {
        self.current_byte_from_network.replace(byte);
    }

    fn get_from_tx_pin(&mut self) -> Option<u8> {
        self.rx_from_network_buffer.pop_front()
    }

    fn put_to_rx_pin(&mut self, byte: u8) {
        self.tx_to_network_buffer.push_back(byte);
    }

    fn start_tick(&mut self) {
        self.current_byte_to_network = None;
        self.current_byte_from_network = None;

        if let Some(byte) = self.tx_to_network_buffer.pop_front() {
            self.current_byte_to_network = Some(byte);
        }
    }

    fn end_tick(&mut self) {
        if let Some(byte) = self.current_byte_from_network.take() {
            self.rx_from_network_buffer.push_back(byte);
        }
        self.current_byte_to_network = None;
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
}
