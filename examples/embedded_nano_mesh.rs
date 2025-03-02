use std::time::Instant;

use embedded_nano_mesh::{ms, ExactAddressType, Node, NodeConfig, NodeString, PacketState};
use network_simulator::{NetworkSimulator, WirelessModemFake};

enum ThreadTestResult {
    Success,
    Failure,
}

const NODE_1_LISTEN_PERIOD: ms = 1;
const NODE_2_LISTEN_PERIOD: ms = 1;

fn main() {
    /* Create simulator, ether, and devices registered in that ether. */
    let mut simulator = NetworkSimulator::new(0);

    simulator.create_ether("1");

    let driver_1 = WirelessModemFake::new("1");
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
    let mut clonned_driver_1 = driver_1.clone();

    let pinger_thread = std::thread::spawn(move || {
        let thread_start_time = Instant::now();
        match mesh_node_1.send_ping_pong(
            NodeString::try_from("Msg from node 1")
                .expect("Fail to convert to NodeString")
                .into_bytes(),
            2.try_into().expect("2 equals to 0"),
            1.into(),
            200 as ms,
            || Instant::now().duration_since(thread_start_time).as_millis() as ms,
            &mut clonned_driver_1,
        ) {
            Ok(_) => ThreadTestResult::Success,
            Err(_) => ThreadTestResult::Failure,
        }
    });

    // Start automatic simulation of ethers to provide communication between their registered
    // nodes.
    simulator.start_simulation_thread();

    let start_time = Instant::now();
    let mut break_loop_at = None;

    loop {
        let current_time = Instant::now().duration_since(start_time).as_millis() as ms;

        let _ = mesh_node_2.update(&mut driver_2, current_time);

        if let Some(_packet) = mesh_node_2.receive() {
            match _packet.get_spec_state() {
                PacketState::Ping => {
                    break_loop_at.replace((current_time + NODE_2_LISTEN_PERIOD) * 4);
                }
                _ => (),
            };
        }

        if let Some(break_at) = break_loop_at {
            if current_time >= break_at {
                break;
            }
        }
    }

    simulator.stop_simulation_thread();

    let result = pinger_thread
        .join()
        .expect("Fail to get result of thread expecting ping pong");

    assert!(match result {
        ThreadTestResult::Success => true,
        ThreadTestResult::Failure => false,
    });

    println!("Simulation done");
}
