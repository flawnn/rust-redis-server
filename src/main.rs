use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::message::message::Message;
mod message;

fn main() {
    println!("Server started up! 💯");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

                thread::spawn(move || {
                    handle_client(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let raw_string = String::from_utf8_lossy(&buffer[..n]);
                println!("Received message: {}", raw_string);

                // Conver to Message
                let msg = Message::from_bytes(&buffer[..n]);

                match msg {
                    Ok(message) => {
                        if message.command == "ping" {
                            let _ = stream.write_all(b"+PONG\r\n").unwrap();
                        }

                        if message.command == "echo" {
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
                    }
                    Err(error) => {
                        println!("Not a message");
                    }
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
