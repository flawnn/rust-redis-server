use std::{net::TcpStream, io::Write};

use crate::message::message::Message;


pub fn handle_echo(message: &Message, stream: &mut TcpStream) {
    let response = [
        "$",
        message.args[1].len().to_string().as_str(),
        "\r\n".to_string().as_str(),
        (message.args[1]).as_str(),
        "\r\n".to_string().as_str(),
    ]
    .concat();
    let _ = stream.write_all(&response[..].as_bytes()).unwrap();
}
