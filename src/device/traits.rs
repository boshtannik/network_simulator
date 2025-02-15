pub trait IODriverSimulator {
    // Antenna interfaces
    fn get_from_device_network_side(&mut self) -> Option<u8>;
    fn put_to_device_network_side(&mut self, byte: u8);

    // Transceiver interfaces
    fn get_from_tx_pin(&mut self) -> Option<u8>;
    fn put_to_rx_pin(&mut self, byte: u8);

    // Time quantification interfaces
    fn start_tick(&mut self);
    fn end_tick(&mut self);

    // Required methods for some interfaces
    fn read_ready(&self) -> bool;
    fn write_ready(&self) -> bool;
}
