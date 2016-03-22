extern crate zmq;
use uuid::Uuid;
use std::env;
use zmq_connection::*;
#[allow(dead_code)]
pub fn spawn_command(command: &str) {
    let addr = env::var("RCON_SOCKET").unwrap();
    let mut context = zmq::Context::new();
    let mut socket = context.socket(zmq::DEALER).unwrap();
    let identity = Uuid::new_v4();
    socket.set_identity(&identity.to_simple_string().as_bytes()).unwrap();
    assert!(socket.connect(addr.as_str()).is_ok());
    socket.send(b"register\n", 0).unwrap();
    socket.send(command.as_bytes(), 0).unwrap();
    // read the message acknowledging that the command was received
    println!("{}", get_message(&mut socket));
    // then read the output that the command produced
    loop {
        print!("{}", get_message(&mut socket));
        if events_remaining(&socket) <= 0 {
            break;
        }
    }
}
