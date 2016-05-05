extern crate zmq;
use uuid::Uuid;
use std::env;
use zmq_connection::*;
use redis::Commands;
use redis_connection::establish_connection;

#[allow(dead_code)]
pub fn command(command: &str) {
    let addr = env::var("RCON_SOCKET").unwrap();
    let mut context = zmq::Context::new();
    let mut socket = context.socket(zmq::DEALER).unwrap();
    let identity = Uuid::new_v4();
    socket.set_identity(&identity.to_simple_string().as_bytes()).unwrap();
    assert!(socket.connect(addr.as_str()).is_ok());
    let conn = establish_connection();
    socket.send(b"register\n", 0).unwrap();
    socket.send(command.as_bytes(), 0).unwrap();
    // read the message acknowledging that the command was received
    // but dont store it
    get_message(&mut socket);
    // then read the output that the command produced
    loop {
        if events_remaining(&socket) <= 0 {
            break;
        }
        let _: () = conn.lpush("last_ten_rcon_messages", get_message(&mut socket)).unwrap();
    }
}
