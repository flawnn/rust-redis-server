use std::{net::TcpStream, io::Write};



pub fn handle_ping(stream: &mut TcpStream) {
    let _ = stream.write_all(b"+PONG\r\n").unwrap();
}
