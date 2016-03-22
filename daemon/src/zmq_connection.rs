extern crate zmq;
#[allow(dead_code)]
pub fn events_remaining(socket: &zmq::Socket) -> i32 {
    zmq::poll(&mut [socket.as_poll_item(1)], 100).unwrap()
}

#[allow(dead_code)]
pub fn get_message(socket: &mut zmq::Socket) -> String {
    let result = socket.recv_msg(0)
                       .expect("could not receive message")
                       .to_vec();
    String::from_utf8(result).unwrap()
}
