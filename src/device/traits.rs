pub trait IODriverSimulator {
    // Network interfaces
    fn get_from_device_network_side(&mut self) -> Option<u8>;
    fn put_to_device_network_side(&mut self, byte: u8);

    // Device pins interfaces
    fn get_from_tx_pin(&mut self) -> Option<u8>;
    fn put_to_rx_pin(&mut self, byte: u8);

    // Time quantification interfaces
    fn start_tick(&mut self);
    fn end_tick(&mut self);

    // Required methods for some interfaces
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;

    fn get_name(&self) -> &str;
}
