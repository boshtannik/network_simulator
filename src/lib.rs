mod device;
mod ether;

pub use device::{IODriverSimulator, WiredModemFake, WirelessModemFake};
pub use ether::Ether;
