mod traits;
mod wired_modem;
mod wireless_modem;

pub use {
    traits::IODriverSimulator, wired_modem::WiredModuleDriver, wireless_modem::WirelessModuleDriver,
};
