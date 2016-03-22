#![feature(custom_derive, plugin, specialization, custom_attribute)]
#![plugin(dotenv_macros, serde_macros, stainless)]
extern crate dotenv;
extern crate uuid;
extern crate serde;
extern crate rustc_serialize;
extern crate serde_json;
extern crate redis;
extern crate bincode;
extern crate zmq;
extern crate chrono;
mod zmq_connection;
mod stats;
mod tests;
mod rcon;

use stats::parser::Parser;
use redis::Commands;
use std::thread;
use dotenv::dotenv;
use std::path::Path;
use std::io::LineWriter;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

fn main() {
    dotenv().ok();
    // let mut children = vec![];
    // children.push(thread::spawn(move || {
    //     Parser::parse_file("match_started.json".to_owned());
    //     Parser::parse_file("match_report.json".to_owned());
    //     Parser::parse_file("player_kill.json".to_owned());
    //     Parser::parse_file("round_over.json".to_owned());
    //     Parser::parse_file("player_death.json".to_owned());
    //     Parser::parse_file("player_stats.json".to_owned());
    //     Parser::parse_file("player_medal.json".to_owned());
    //     let client = redis::Client::open("redis://127.0.0.1")
    //                      .expect("could not open client for reading");
    //     let conn = client.get_connection().expect("could not obtain client for reading");
    //     let first_match: String = conn.get("match_started:1").expect("could not get first event");
    //     println!("{}", first_match);
    // }));
    // children.push(thread::spawn(move || {
    stats::zmq::connection();
    // }));
    // for child in children {
    //     let _ = child.join();
    // }
}
