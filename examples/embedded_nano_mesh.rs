use std::time::Instant;

use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString};
use network_simulator::NetworkSimulator;

fn main() {
    let mut simulator = NetworkSimulator::new(1);

    let mut ether_1 = simulator.create_ether("1");

    let mut ether_1_driver_1 = ether_1.create_driver("1");
    let mut ether_1_driver_2 = ether_1.create_driver("2");
    let mut ether_1_driver_3 = ether_1.create_driver("3");

    ether_1.register_driver(ether_1_driver_1);
    ether_1.register_driver(ether_1_driver_2);
    ether_1.register_driver(ether_1_driver_3);

    let mut mesh_node_1 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(1).expect("1 equals to 0"),
        listen_period: 10 as ms,
    });

    let mut mesh_node_2 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(2).expect("2 equals to 0"),
        listen_period: 11 as ms,
    });

    let mut mesh_node_3 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(3).expect("3 equals to 0"),
        listen_period: 12 as ms,
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
        .expect("Fail to send message form mesh_node_2");

    simulator.start_simulation_thread();

    let start_time = Instant::now();
    loop {
        let current_time = Instant::now().duration_since(start_time).as_millis() as ms;
        let _ = mesh_node_1.update(
            ether_1
                .get_driver("1")
                .expect("Fail to find driver for mesh_node_1"),
            current_time,
        );
    }

    simulator.stop_simulation_thread();
}
