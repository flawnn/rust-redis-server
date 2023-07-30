use std::{net::TcpStream, io::Write};


static NIL_STRING: &str = "$-1\r\n";
static OK_STRING: &str = "+OK\r\n";


pub fn send_nil(stream: &mut TcpStream) {
    let _ = stream.write_all(&NIL_STRING[..].as_bytes()).unwrap();
}

pub fn send_ok(stream: &mut TcpStream) {
    let _ = stream.write_all(&OK_STRING[..].as_bytes()).unwrap();
}
