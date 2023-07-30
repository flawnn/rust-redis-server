use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;

use crate::message::commands::{
    echo::handle_echo, get::handle_get, ping::handle_ping, set::handle_set,
};
use crate::{message::message::Message, structs::app_state::AppState};

pub fn handle_client(mut stream: TcpStream, app_state: &Arc<AppState>) {
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
                        // match message.command.as_str() {}
                        if message.command == "ping" {
                            handle_ping(&mut stream);
                        }

                        if message.command == "echo" {
                            handle_echo(&message, &mut stream);
                        }

                        if message.command == "get" {
                            handle_get(app_state, &message, &mut stream);
                        }

                        if message.command == "set" {
                            handle_set(app_state, message, &mut stream);
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