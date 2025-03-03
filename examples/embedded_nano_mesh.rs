use std::time::Instant;

use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString};
use network_simulator::{NetworkSimulator, WirelessModemFake};

const NODE_1_LISTEN_PERIOD: ms = 1;
const NODE_2_LISTEN_PERIOD: ms = 1;

fn main() {
    /* Create simulator, ether, and devices registered in that ether. */
    let mut simulator = NetworkSimulator::new(1);

    simulator.create_ether("1");

    let mut driver_1 = WirelessModemFake::new("1");
    let mut driver_2 = WirelessModemFake::new("2");

    simulator
        .get_ether("1")
        .expect("Failed to find ether 1")
        .register_driver(driver_1.clone());

    simulator
        .get_ether("1")
        .expect("Failed to find ether 1")
        .register_driver(driver_2.clone());

    /* Create tested nodes. */
    let mut mesh_node_1 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(1).expect("1 equals to 0"),
        listen_period: NODE_1_LISTEN_PERIOD,
    });

    let mut mesh_node_2 = Node::new(NodeConfig {
        device_address: ExactAddressType::try_from(2).expect("2 equals to 0"),
        listen_period: NODE_2_LISTEN_PERIOD,
    });

    /* Prepare separate thread for node_1 perform send_ping_pong */

    let _ = mesh_node_1.send_to_exact(
        NodeString::try_from("Message from node 1")
            .expect("Fail to pack message")
            .into_bytes(),
        ExactAddressType::try_from(2).expect("2 is 0"),
        1.into(),
        false,
    );

    let start_time = Instant::now();

    simulator.start_simulation_thread();

    loop {
        let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

        let _ = mesh_node_1.update(&mut driver_1, current_time);
        let _ = mesh_node_2.update(&mut driver_2, current_time);

        if current_time >= 200 {
            panic!("Simulation timeout");
        }

        if let Some(_packet) = mesh_node_2.receive() {
            println!("Packet got");
            break;
        }
    }

    simulator.stop_simulation_thread();

    println!("Simulation done");
}
