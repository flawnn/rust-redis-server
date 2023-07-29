use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::message::commands::{
    echo::handle_echo, get::handle_get, ping::handle_ping, set::handle_set,
};
use crate::{message::message::Message, structs::app_state::AppState};

mod message;
mod structs;
fn main() {
    println!("Server started up! 💯");

    let app_state = Arc::new(AppState {
        dict: Mutex::new(HashMap::new()),
    });

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");

                let app_state = app_state.clone();

                thread::spawn(move || {
                    handle_client(_stream, &app_state);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

static NIL_STRING: &str = "$-1\r\n";
static OK_STRING: &str = "+OK\r\n";

fn handle_client(mut stream: TcpStream, app_state: &Arc<AppState>) {
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

fn send_nil(stream: &mut TcpStream) {
    let _ = stream.write_all(&NIL_STRING[..].as_bytes()).unwrap();
}

fn send_ok(stream: &mut TcpStream) {
    let _ = stream.write_all(&OK_STRING[..].as_bytes()).unwrap();
}
