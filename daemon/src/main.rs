#![feature(custom_derive, plugin, specialization)]
#![plugin(clippy, dotenv_macros, serde_macros)]
extern crate dotenv;
extern crate uuid;
extern crate zmq;
extern crate serde;
extern crate serde_json;
mod stats;

use dotenv::dotenv;
use std::thread;
use uuid::Uuid;

fn stats_connection() {
    let mut context = zmq::Context::new();
    let addr = dotenv!("STATS_SOCKET");
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect(&addr).expect("failed to connect to stats port");
    subscriber.set_subscribe(&[0]).expect("could not subscribe to stats port");
    loop {
        print!("{}", get_message(&mut subscriber));
    }
}

fn events_remaining(socket: &zmq::Socket) -> i32 {
    zmq::poll(&mut [socket.as_poll_item(1)], 100).unwrap()
}

fn get_message(socket: &mut zmq::Socket) -> String {
    let result = socket.recv_msg(0)
        .expect("could not receive message")
        .to_vec();
    String::from_utf8(result).unwrap()
}

fn spawn_rcon_command(command: &str) {
    let addr = dotenv!("RCON_SOCKET");
    let mut context = zmq::Context::new();
    let mut socket = context.socket(zmq::DEALER).unwrap();
    let identity = Uuid::new_v4();
    socket.set_identity(&identity.to_simple_string().as_bytes()).unwrap();
    assert!(socket.connect(addr).is_ok());
    socket.send(b"register\n", 0).unwrap();
    socket.send(command.as_bytes(), 0).unwrap();
    // read the message acknowledging that the command was received
    println!("{}", get_message(&mut socket));
    loop {
        print!("{}", get_message(&mut socket));
        if events_remaining(&socket) == 0 {
            break;
        }
    }
}

fn main() {
    dotenv().ok();
    let mut children = vec![];
    children.push(thread::spawn(move || {
        spawn_rcon_command("serverinfo");
    }));

    children.push(thread::spawn(move || {
        stats_connection();
    }));

    for child in children {
        let _ = child.join();
    }
}
