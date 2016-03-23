extern crate redis;

pub fn establish_connection() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").expect("could not open conn");
    client.get_connection().expect("could not obtain conn")
}
