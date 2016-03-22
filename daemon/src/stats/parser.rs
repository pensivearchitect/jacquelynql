extern crate serde;
extern crate serde_json;
extern crate redis;
extern crate chrono;

use serde_json::Value;
use chrono::*;
use stats::models::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde::de::Deserialize;
use std::fmt::Debug;
use redis::{Commands, PipelineCommands};
pub struct Parser;

trait Event {
    fn new(&str, &str) -> ();
}

// I <3 impl specialization
impl<T> Event for T
    where T: Deserialize + Debug
{
    fn new(data: &str, event_type: &str) {
        // TODO: use dotenv to grab the url
        let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
        let conn: redis::Connection = client.get_connection()
                                            .expect("could not obtain conn");
        let key = event_type;
        let _: () = conn.lpush(key, data).expect("could not push onto key");
        // make inspecting the most recent values easier
        // it seems to be a safe assumption that we wont generate 10 events
        // in the time frame required to kick someone, will revisit if necessary
        let _: () = conn.lpush("last10keys", data).expect("could not push key onto last10keys");
        let _: () = conn.ltrim("last10keys", 0, 9).expect("could not trim last10keys");
    }
}

impl Event for MatchReport {}
impl Event for MatchStarted {}
impl Event for PlayerKill {}
impl Event for PlayerDeath {}
impl Event for RoundOver {}
impl Event for PlayerStats {}
impl Event for PlayerMedal {}
impl Event for PlayerConnect {}
impl Event for PlayerDisconnect {}
impl Event for PlayerSwitchTeam {}

pub enum Events {
    MatchReport,
    MatchStarted,
    PlayerKill,
    PlayerDeath,
    RoundOver,
    PlayerStats,
    PlayerMedal,
    PlayerConnect,
    PlayerDisconnect,
    TeamChange,
    Unknown,
}

impl Parser {
    pub fn parse(string: String) -> Events {
        let mut message_data: Value = serde_json::from_str(&string.as_str()).unwrap();
        let local_time = Local::now();
        let timestamp = local_time.nanosecond();
        let mut object = message_data.as_object_mut().unwrap();
        object.insert("time_stamp".to_string(), Value::I64(timestamp as i64));
        let (data, event_type) = if object.contains_key("stats") {
            let stats = object.get("stats").unwrap().as_object().unwrap();
            (serde_json::to_string(&stats.get("DATA").unwrap()).unwrap(),
             stats.get("TYPE").unwrap().as_string().unwrap())
        } else if object.contains_key("DATA") {
            (serde_json::to_string(&object.get("DATA").unwrap()).unwrap(),
             object.get("TYPE").unwrap().as_string().unwrap())
        } else {
            (serde_json::to_string(&object.get("KILLER").unwrap()).unwrap(),
             "TEAM_CHANGE")
        };
        match event_type {
            "MATCH_STARTED" => {
                MatchStarted::new(&data, &event_type.to_lowercase());
                Events::MatchStarted
            }
            "MATCH_REPORT" => {
                MatchReport::new(&data, &event_type.to_lowercase());
                Events::MatchReport
            }
            "PLAYER_KILL" => {
                PlayerKill::new(&data, &event_type.to_lowercase());
                Events::PlayerKill
            }
            "PLAYER_DEATH" => {
                PlayerDeath::new(&data, &event_type.to_lowercase());
                Events::PlayerDeath
            }
            "ROUND_OVER" => {
                RoundOver::new(&data, &event_type.to_lowercase());
                Events::RoundOver
            }
            "PLAYER_STATS" => {
                PlayerStats::new(&data, &event_type.to_lowercase());
                Events::PlayerStats
            }
            "PLAYER_MEDAL" => {
                PlayerMedal::new(&data, &event_type.to_lowercase());
                Events::PlayerMedal
            }
            "PLAYER_CONNECT" => {
                PlayerConnect::new(&data, &event_type.to_lowercase());
                Events::PlayerConnect
            }
            "PLAYER_DISCONNECT" => {
                PlayerDisconnect::new(&data, &event_type.to_lowercase());
                Events::PlayerDisconnect
            }
            "PLAYER_SWITCHTEAM" => {
                PlayerSwitchTeam::new(&data, &event_type.to_lowercase());
                Events::TeamChange
            }
            _ => {
                println!("{}", event_type);
                println!("unknown event found {}", &string);
                Events::Unknown
            }
        }
    }
    pub fn parse_file(f: String) {
        let path = Path::new(&f);
        let file_name = path.to_str().unwrap();
        let mut file = File::open(&path).expect(format!("could not open {}", file_name).as_str());
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect(format!("could not read {}", &file_name).as_str());
        Parser::parse(buffer);
    }
}
