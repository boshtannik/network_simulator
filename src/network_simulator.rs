use std::sync::{Arc, Mutex};

use crate::{EtherSimulator, WirelessModemFake};

pub struct NetworkSimulator {
    ethers: Option<Vec<EtherSimulator<WirelessModemFake>>>,
    ms_per_tick: u64,
    simulation_thread_handle:
        Option<std::thread::JoinHandle<Vec<EtherSimulator<WirelessModemFake>>>>,
    thread_killer: Arc<Mutex<bool>>,
}

impl NetworkSimulator {
    pub fn new(ms_per_tick: u64) -> Self {
        NetworkSimulator {
            ethers: Some(Vec::new()),
            ms_per_tick,
            simulation_thread_handle: None,
            thread_killer: Arc::new(Mutex::new(false)),
        }
    }

    pub fn create_ether(&mut self, name: &str) -> &mut EtherSimulator<WirelessModemFake> {
        match &mut self.ethers {
            Some(ethers) => {
                let new_ether = EtherSimulator::new(name);
                ethers.push(new_ether);
            }
            None => {
                panic!("Simulation thread is already started. Can not change configuration")
            }
        }
        self.get_ether(name).unwrap()
    }

    pub fn get_ether(&mut self, name: &str) -> Option<&mut EtherSimulator<WirelessModemFake>> {
        match self.ethers {
            None => panic!("Simulation thread is started. Can not get ether"),
            Some(ref mut ethers) => {
                for ether in ethers.iter_mut() {
                    if ether.get_name() == name {
                        return Some(ether);
                    }
                }
                None
            }
        }
    }

    pub fn start_simulation_thread(&mut self) {
        match self.simulation_thread_handle {
            Some(_) => panic!("Simulation thread is already started"),
            None => {
                let mut ethers = self.ethers.take().unwrap();
                let ms_per_tick = self.ms_per_tick;
                let thread_killer_clone = Arc::clone(&self.thread_killer);

                *self
                    .thread_killer
                    .lock()
                    .expect("Fail to get lock on thread killer") = false;

                self.simulation_thread_handle = Some(std::thread::spawn(move || {
                    loop {
                        if *thread_killer_clone
                            .lock()
                            .expect("Faild to get lock on clonned thread killer")
                        {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(ms_per_tick));
                        for ether in ethers.iter_mut() {
                            ether.start_tick();
                            ether.simulate();
                            ether.end_tick();
                        }
                    }
                    ethers
                }));
            }
        }
    }

    pub fn stop_simulation_thread(&mut self) {
        println!("Stopping");
        self.simulation_thread_handle = match self.simulation_thread_handle.take() {
            None => panic!("Simulation thread is not started"),
            Some(simulation_thread_handle) => {
                *self
                    .thread_killer
                    .lock()
                    .expect("Fail to get lock on thread killer") = true;
                self.ethers.replace(
                    simulation_thread_handle
                        .join()
                        .expect(" Fail to join simulation thread to get ethers back"),
                );
                None
            }
        };
        println!("Stopped");
    }
}
