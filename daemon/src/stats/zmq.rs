extern crate zmq;
extern crate redis;
use stats::parser::{Parser, Events};
use std::env;
use zmq_connection::*;

#[allow(dead_code)]
pub fn connection() {
    let mut context = zmq::Context::new();
    let addr = env::var("STATS_SOCKET").unwrap();
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect(&addr).expect("failed to connect to stats port");
    subscriber.set_subscribe("".to_string().as_bytes()).expect("could not subscribe to stats port");
    let mut flag: bool = false;
    loop {
        let message = get_message(&mut subscriber);
        match Parser::parse(message) {
            Events::MatchStarted => flag = true,
            Events::MatchReport => flag = false,
            Events::TeamChange if flag == true => kick_player(),
            _ => (),
        }
    }
}

fn did_anyone_spectate() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
    let conn: redis::Connection = client.get_connection()
                                        .expect("could not obtain conn");
    let last_ten_keys = conn.lrange("last10keys");
}

fn kick_player() {
    println!("player kick condition reached");
}
