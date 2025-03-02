pub trait IODriverSimulator {
    // Network interfaces
    fn get_from_device_network_side(&self) -> Option<u8>;
    fn put_to_device_network_side(&self, byte: u8);

    // Device pins interfaces
    fn get_from_tx_pin(&self) -> Option<u8>;
    fn put_to_rx_pin(&self, byte: u8);

    // Time quantification interfaces
    fn start_tick(&self);
    fn end_tick(&self);

    // Required methods for some interfaces
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;

    fn get_name(&self) -> &str;
}
