use std::{
    iter::zip,
    ops::DerefMut,
    sync::{Arc, Mutex},
    time::Instant,
};

use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
use network_simulator::{Ether, WirelessModemFake};

// The following code below does 4 general things:
// 1. Sets up fake modem group which are working in fake ether, updated in by separate thread.
// 2. Instantiates protocol mesh nodes.
// 3. Does the simulation of expected test scenario.
// 4. Terminates the thread and the whole simulation.

fn main() {
    let program_start_time = Instant::now();

    let modem_1 = WirelessModemFake::new();
    let modem_2 = WirelessModemFake::new();
    let modem_3 = WirelessModemFake::new();

    let shared_modems = [
        Arc::new(Mutex::new(modem_1)),
        Arc::new(Mutex::new(modem_2)),
        Arc::new(Mutex::new(modem_3)),
    ];

    let clonned_shared_modems = [
        Arc::clone(&shared_modems[0]),
        Arc::clone(&shared_modems[1]),
        Arc::clone(&shared_modems[2]),
    ];

    let ether_thread_killer = Arc::new(Mutex::new(false));
    let ether_thread_killer_clone = Arc::clone(&ether_thread_killer);

    let ether_thread = std::thread::spawn(move || {
        let mut ether = Ether::new(&clonned_shared_modems);

        loop {
            if *ether_thread_killer_clone
                .lock()
                .expect("Thread failed to check thread_killer var")
            {
                break;
            }
            ether.start_tick();
            ether.simulate();
            ether.end_tick();
        }
    });

    let mut node_1 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(1).unwrap(),
        listen_period: 10 as ms,
    });

    let mut node_2 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(2).unwrap(),
        listen_period: 20 as ms,
    });

    let mut node_3 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(3).unwrap(),
        listen_period: 30 as ms,
    });

    let _ = node_1.send_to_exact(
        NodeString::from_iter("This is the message from node 1".chars()).into_bytes(),
        ExactAddressType::try_from(2).unwrap(),
        LifeTimeType::try_from(1).unwrap(),
        true,
    );

    let _ = node_3.send_to_exact(
        NodeString::from_iter("This is the message from node 3".chars()).into_bytes(),
        ExactAddressType::try_from(2).unwrap(),
        LifeTimeType::try_from(1).unwrap(),
        true,
    );

    let mut expected_messages_count = 2;

    loop {
        let current_time = Instant::now()
            .duration_since(program_start_time)
            .as_millis() as ms;

        for (shared_simulated_modem, node) in
            zip(&shared_modems, [&mut node_1, &mut node_2, &mut node_3])
        {
            let _ = node.update(
                shared_simulated_modem
                    .lock()
                    .expect("Fail to get lock to update node")
                    .deref_mut(),
                current_time,
            );
        }

        while let Some(message) = node_2.receive() {
            println!(
                "Node 2 got the message: {}",
                NodeString::from_iter(message.data.iter().map(|b| *b as char))
            );
            expected_messages_count -= 1;
        }

        if expected_messages_count.eq(&0) {
            break;
        }
    }

    *ether_thread_killer
        .lock()
        .expect("Fail to get lock on simulated device driver")
        .deref_mut() = true;

    ether_thread
        .join()
        .expect("Fail to join the thread that simulates the network");

    println!("Simulation done");
}
