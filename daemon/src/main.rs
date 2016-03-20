#![feature(custom_derive, plugin, specialization)]
#![plugin(clippy, dotenv_macros, serde_macros)]
extern crate dotenv;
extern crate uuid;
extern crate zmq;
extern crate serde;
extern crate rustc_serialize;
extern crate serde_json;
mod stats;

use stats::parser::Parser;
use dotenv::dotenv;
use std::thread;
use std::path::Path;
use std::io::LineWriter;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

#[allow(dead_code)]
fn stats_connection() {
    let mut context = zmq::Context::new();
    let addr = dotenv!("STATS_SOCKET");
    let mut subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect(&addr).expect("failed to connect to stats port");
    subscriber.set_subscribe(&[0]).expect("could not subscribe to stats port");
    let f = File::create("stats.txt").expect("could not create file");
    let mut file = LineWriter::new(f);
    loop {
        let message = get_message(&mut subscriber);
        file.write(&message.as_bytes()).unwrap();
    }
}

#[allow(dead_code)]
fn events_remaining(socket: &zmq::Socket) -> i32 {
    zmq::poll(&mut [socket.as_poll_item(1)], 100).unwrap()
}

#[allow(dead_code)]
fn get_message(socket: &mut zmq::Socket) -> String {
    let result = socket.recv_msg(0)
                       .expect("could not receive message")
                       .to_vec();
    String::from_utf8(result).unwrap()
}

#[allow(dead_code)]
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
    // then read the output that the command produced
    loop {
        print!("{}", get_message(&mut socket));
        if events_remaining(&socket) <= 0 {
            break;
        }
    }
}

fn spawn(f: String) {
    let path = Path::new(&f);
    let file_name = path.to_str().unwrap();
    let mut file = File::open(&path).expect(format!("could not open {}", file_name).as_str());
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect(format!("could not read {}", &file_name).as_str());
    Parser::parse(buffer);
}

fn main() {
    dotenv().ok();
    let mut children = vec![];
    children.push(thread::spawn(move || {
        spawn("match_started.json".to_owned());
        spawn("match_report.json".to_owned());
        spawn("player_kill.json".to_owned());
        spawn("round_over.json".to_owned());
        spawn("player_death.json".to_owned());
        spawn("player_stats.json".to_owned());
        spawn("player_medal.json".to_owned());
    }));
    // children.push(thread::spawn(move || {
    //     spawn_rcon_command("serverinfo");
    // }));

    // children.push(thread::spawn(move || {
    //     stats_connection();
    // }));

    for child in children {
        let _ = child.join();
    }
}
