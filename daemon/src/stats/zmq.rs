extern crate zmq;
extern crate redis;
extern crate serde;
extern crate serde_json;
use stats::parser::Parser;
use stats::parser;
use std::env;
use zmq_connection::*;

#[allow(dead_code)]
pub fn connection() {
    let mut context = zmq::Context::new();
    let addr = env::var("STATS_SOCKET").unwrap();
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect(&addr).expect("failed to connect to stats port");
    subscriber.set_subscribe("".to_string().as_bytes()).expect("could not subscribe to stats port");
    println!("subscribed...\nlistening for events...");
    loop {
        let message = get_message(&mut subscriber);
        Parser::parse(message);
        // parser::find_last_event();
    }
}
