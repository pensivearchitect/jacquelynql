#![feature(custom_derive, plugin, specialization, question_mark, type_macros)]
#![plugin(dotenv_macros, serde_macros)]
extern crate dotenv;
extern crate uuid;
extern crate serde;
extern crate rustc_serialize;
extern crate serde_json;
extern crate redis;
extern crate bincode;
extern crate zmq;
extern crate chrono;
extern crate regex;
mod zmq_connection;
mod stats;
mod rcon;
mod redis_connection;

use std::thread;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let mut children = vec![];
    children.push(thread::spawn(move || {
        stats::zmq::connection();
    }));
    for child in children {
        let _ = child.join();
    }
}
