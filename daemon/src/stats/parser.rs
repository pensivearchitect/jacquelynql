extern crate serde;
extern crate serde_json;
use serde_json::Value;
use stats::events::*;
pub struct Parser;

impl Parser {
    pub fn parse(string: String) {
        let message_data: Value = serde_json::from_str(&string.as_str()).unwrap();
        let object = message_data.as_object().unwrap();
        let stats = object.get("stats").unwrap().as_object().unwrap();
        let event_type = stats.get("TYPE").unwrap().as_string().unwrap();
        let data: String = serde_json::to_string(&stats.get("DATA").unwrap()).unwrap();
        match event_type {
            "MATCH_STARTED" => {
                let serialized_match: MatchStarted = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            "MATCH_REPORT" => {
                let serialized_match: MatchReport = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            "PLAYER_KILL" => {
                let serialized_match: PlayerKill = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            "PLAYER_DEATH" => {
                let serialized_match: PlayerDeath = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            "ROUND_OVER" => {
                let serialized_match: RoundOver = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            "PLAYER_STATS" => {
                let serialized_match: PlayerStats = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            "PLAYER_MEDAL" => {
                let serialized_match: PlayerMedal = serde_json::from_str(&data).unwrap();
                println!("{:?}", serialized_match);
            }
            _ => println!("unknown event found"),
        }
    }
}
