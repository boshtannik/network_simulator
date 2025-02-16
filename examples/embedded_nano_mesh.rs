use std::time::Instant;

use embedded_nano_mesh::*;
use network_simulator::{Ether, IODriverSimulator, WirelessModuleDriver};

fn main() {
    let program_start_time = Instant::now();

    let mut modem_1 = WirelessModuleDriver::new();
    let mut modem_2 = WirelessModuleDriver::new();
    let mut modem_3 = WirelessModuleDriver::new();

    let mut node_1 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(1).unwrap(),
        listen_period: 10 as ms,
    });

    let _ = node_1.send_to_exact(
        NodeString::from_iter("This is the message from node 2".chars()).into_bytes(),
        ExactAddressType::try_from(2).unwrap(),
        LifeTimeType::try_from(1).unwrap(),
        true,
    );

    let mut node_2 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(2).unwrap(),
        listen_period: 20 as ms,
    });

    let mut node_3 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(3).unwrap(),
        listen_period: 30 as ms,
    });

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

        let _ = node_1.update(&mut modem_1, current_time);
        let _ = node_2.update(&mut modem_2, current_time);
        let _ = node_3.update(&mut modem_3, current_time);

        if let Some(message) = node_2.receive() {
            println!(
                "Node 2 has the message: {}",
                NodeString::from_iter(message.data.iter().map(|b| *b as char))
            );
            expected_messages_count -= 1;
        }

        let mut ref_arr = [&mut modem_1, &mut modem_2, &mut modem_3];

        for modem in ref_arr.iter_mut() {
            modem.start_tick();
        }

        {
            let mut ether = Ether::new(&mut ref_arr);
            ether.simulate();
        }

        let mut ref_arr = [&mut modem_1, &mut modem_2, &mut modem_3];

        for modem in ref_arr.iter_mut() {
            modem.end_tick();
        }

        if expected_messages_count.eq(&0) {
            break;
        }
    }

    println!("Simulation done");
}
