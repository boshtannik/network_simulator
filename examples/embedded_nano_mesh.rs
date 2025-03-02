use std::time::Instant;

use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString};
use network_simulator::{EtherSimulator, NetworkSimulator};

fn main() {
    let mut simulator = NetworkSimulator::new(3);

    simulator.create_ether("1");

    let mut driver_1 = EtherSimulator::create_driver("1");
    let mut driver_2 = EtherSimulator::create_driver("2");
    let mut driver_3 = EtherSimulator::create_driver("3");

    simulator
        .get_ether("1")
        .expect("Failed to find ether 1")
        .register_driver(driver_1.clone());

    simulator
        .get_ether("1")
        .expect("Failed to find ether 1")
        .register_driver(driver_2.clone());

    simulator
        .get_ether("1")
        .expect("Failed to find ether 1")
        .register_driver(driver_3.clone());

    let mut mesh_node_1 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(1).expect("1 equals to 0"),
        listen_period: 1 as ms,
    });

    let mut mesh_node_2 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(2).expect("2 equals to 0"),
        listen_period: 5 as ms,
    });

    let mut mesh_node_3 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(3).expect("3 equals to 0"),
        listen_period: 10 as ms,
    });

    mesh_node_1
        .send_to_exact(
            NodeString::try_from("Msg from node 1")
                .expect("Fail to convert to NodeString")
                .into_bytes(),
            2.try_into().expect("2 equals to 0"),
            1,
            false,
        )
        .expect("Fail to send message form mesh_node_1");

    mesh_node_3
        .send_to_exact(
            NodeString::try_from("Msg from node 3")
                .expect("Fail to convert to NodeString")
                .into_bytes(),
            2.try_into().expect("2 equals to 0"),
            1,
            false,
        )
        .expect("Fail to send message form mesh_node_3");

    simulator.start_simulation_thread();

    let start_time = Instant::now();
    loop {
        let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

        let _ = mesh_node_1.update(&mut driver_1, current_time);
        let _ = mesh_node_2.update(&mut driver_2, current_time);
        let _ = mesh_node_3.update(&mut driver_3, current_time);

        if let Some(_packet) = mesh_node_2.receive() {
            break;
        }
    }

    simulator.stop_simulation_thread();

    println!("Simulation done");
}
