mod traits;
// mod wired_modem;
mod wireless_modem;

pub use {
    traits::IODriverSimulator,
    wireless_modem::WirelessModemFake,
    /*wired_modem::WiredModemFake*/
};
