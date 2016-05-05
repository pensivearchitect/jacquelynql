extern crate serde;
extern crate serde_json;
extern crate redis;
extern crate chrono;
extern crate regex;

// bit of a mess, will break everything out into modules once this workse
// aka this is here to stay forever
use serde_json::Value;
use regex::Regex;
use chrono::{UTC, Local, Timelike};
use stats::models::*;
use serde::de::Deserialize;
use std::fmt::Debug;
use redis::{Commands, PipelineCommands};
use redis_connection::establish_connection;
use hyper::Client;
use rcon;
pub struct Parser;

#[derive(Debug, Serialize, Deserialize)]
struct EventWithTimeStamp {
    time: u32,
    event: String,
}

trait Event {
    fn new(&str, &str) -> ();
}


pub fn last_player_to_switch() -> String {
    let conn = establish_connection();
    let last_player_raw: Vec<String> = conn.lrange("player_switchteam", 0, 0)
                                           .unwrap_or(vec!["".to_string()]);
    let last_player: PlayerSwitchTeam = serde_json::from_str(last_player_raw.first()
                                                                            .unwrap()
                                                                            .as_str())
                                            .unwrap();
    last_player.killer.unwrap().steam_id.unwrap()
}

pub fn find_last_event() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
    let conn: redis::Connection = client.get_connection()
                                        .expect("could not obtain conn");
    let events: Vec<String> = conn.lrange("last10events", 0, 9).unwrap();
    let mut time_of_last_teamchange: u32 = 16;
    for value in events.into_iter() {
        let event: EventWithTimeStamp = serde_json::from_str(value.as_str()).unwrap();
        println!("event type: {}", event.event);
        println!("last teamchange {}", time_of_last_teamchange);
        println!("current second mod 60: {}", Local::now().second());
        match event.event.as_str() {
            "match_started" if (Local::now().second() % time_of_last_teamchange) <= 15 => {
                time_of_last_teamchange = kick_player()
            }
            "player_switchteam" => time_of_last_teamchange = event.time,
            _ => (),
        }
    }
}

pub fn grab_steam_id() -> Vec<(String, i32)> {
    let conn = establish_connection();
    rcon::command("players");
    // format: server_player_id steam_id (admin status or blank) ql_formatted_steam_name
    let name_capture = Regex::new(r"\Aprint.* (?P<player_id>\d) (?P<steam_id>\d+?) (?P<admin_status>\w*?) (?P<name>.*)\^7").unwrap();
    let response: Vec<String> = conn.lrange("last_ten_rcon_messages", 0, 0).unwrap();
    println!("{:?}", response);
    let mut steam_ids: Vec<(String, i32)> = vec![];
    for player in response.into_iter() {
        let captures = name_capture.captures(player.as_str()).unwrap();
        let player: (String, i32) = (captures.name("steam_id").unwrap_or("").to_owned(),
                                     captures.name("player_id")
                                             .unwrap_or("-1")
                                             .parse::<i32>()
                                             .unwrap_or(-1));
        steam_ids.push(player);
    }
    println!("{:?}", steam_ids);
    steam_ids
}

pub fn kick_player() -> u32 {
    let users = grab_steam_id();
    let user_to_kick = users.first().unwrap();
    let steam_id_of_user = user_to_kick.1;
    let command: String = format!("say {} s", steam_id_of_user);
    rcon::command(command.as_str());
    16
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
        let event = EventWithTimeStamp {
            time: Local::now().second(),
            event: event_type.to_string(),
        };
        let _: () = conn.lpush("last10events", serde_json::to_string(&event).unwrap())
                        .expect("could not push key onto last10events");
        let _: () = conn.ltrim("last10events", 0, 9).expect("could not trim last10events");
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

impl MatchRecorder {
    pub fn start() {
        let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
        let conn: redis::Connection = client.get_connection()
                                            .expect("could not obtain conn");
        let _: () = conn.set("ongoing_match", "1").expect("could not reset ongoing_match status");
    }

    pub fn end() {
        // reset match recording status
        let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
        let conn: redis::Connection = client.get_connection()
                                            .expect("could not obtain conn");
        let _: () = conn.set("ongoing_match", "0").expect("could not reset ongoing_match status");
        // phone home
        let client = Client::new();
        let match_data_vec: Vec<String> = conn.lrange("current_match", 0, -1)
                                              .unwrap_or(vec!["".to_string()]);
        // push_str returns () so we make this mut for readability reasons
        let mut match_data: String = String::from("{ \"match_data\": [");
        match_data.push_str(&*match_data_vec.join(", "));
        match_data.push_str("] }");
        match client.post("http://127.0.0.1:3000/matches")
                    .body(&match_data)
                    .send() {
            Ok(response) => (),
            Err(err) => println!("{}", err),
        }
        // archive match
        // TODO: delete archived matches after x number of games
        let _: () = conn.rename("current_match", &*UTC::now().timestamp().to_string())
                        .expect("could not rename current_match to timestamped archive");
    }
}

impl Parser {
    pub fn parse(string: String) {
        let conn = establish_connection();
        let ongoing: String = conn.get("ongoing_match")
                                  .expect("could not retrieve ongoing_match status");
        if ongoing == "1" {
            let data = string.clone();
            let _: () = conn.lpush("current_match", data)
                            .unwrap();
        }
        let mut message_data: Value = serde_json::from_str(&string.as_str()).unwrap();
        let object = message_data.as_object_mut().unwrap();
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
                MatchRecorder::start();
                MatchStarted::new(&data, &event_type.to_lowercase());
            }
            "MATCH_REPORT" => {
                MatchReport::new(&data, &event_type.to_lowercase());
                MatchRecorder::end();
            }
            "PLAYER_KILL" => {
                PlayerKill::new(&data, &event_type.to_lowercase());
            }
            "PLAYER_DEATH" => {
                PlayerDeath::new(&data, &event_type.to_lowercase());
            }
            "ROUND_OVER" => {
                RoundOver::new(&data, &event_type.to_lowercase());
            }
            "PLAYER_STATS" => {
                PlayerStats::new(&data, &event_type.to_lowercase());
            }
            "PLAYER_MEDAL" => {
                PlayerMedal::new(&data, &event_type.to_lowercase());
            }
            "PLAYER_CONNECT" => {
                PlayerConnect::new(&data, &event_type.to_lowercase());
            }
            "PLAYER_DISCONNECT" => {
                PlayerDisconnect::new(&data, &event_type.to_lowercase());
            }
            "PLAYER_SWITCHTEAM" => {
                PlayerSwitchTeam::new(&data, &event_type.to_lowercase());
            }
            _ => {
                println!("{}", event_type);
                println!("unknown event found {}", &string);
            }
        }
    }
}
