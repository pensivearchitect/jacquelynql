extern crate zmq;
extern crate redis;
extern crate serde;
extern crate serde_json;
use stats::parser::Parser;
use stats::models::*;
use redis::Commands;
use std::env;
use zmq_connection::*;

#[allow(dead_code)]
pub fn connection() {
    let mut context = zmq::Context::new();
    let addr = env::var("STATS_SOCKET").unwrap();
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect(&addr).expect("failed to connect to stats port");
    subscriber.set_subscribe("".to_string().as_bytes()).expect("could not subscribe to stats port");
    loop {
        let message = get_message(&mut subscriber);
        Parser::parse(message);
        did_anyone_spectate();
    }
}

fn did_anyone_spectate() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
    let conn: redis::Connection = client.get_connection()
                                        .expect("could not obtain conn");
    let last_ten_keys: Vec<String> = conn.lrange("last10events", 0, 9).unwrap();
    let mut match_started: Vec<MatchStarted> = vec![];
    for value in &last_ten_keys {
        let data: serde_json::Value = serde_json::from_str(&value.as_str()).unwrap();
    }
    println!("{:?}", last_ten_keys);
}

fn kick_player() {}
