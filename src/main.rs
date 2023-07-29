use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::message::message::Message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
mod message;

struct AppState {
    dict: Mutex<HashMap<String, String>>,
}

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

                        if message.command == "get" {
                            let mut dict = app_state.dict.lock().unwrap();

                            let nil_value: String = "nil".to_string();

                            let val = dict.get(&message.args[1]).unwrap_or(&nil_value);

                            let response = [
                                "$",
                                val.len().to_string().as_str(),
                                "\r\n".to_string().as_str(),
                                (val).as_str(),
                                "\r\n".to_string().as_str(),
                            ]
                            .concat();
                            let _ = stream.write_all(&response[..].as_bytes()).unwrap();

                            drop(dict);
                        }

                        if message.command == "set" {
                            let mut dict = app_state.dict.lock().unwrap();

                            let ret: String = "OK".to_string();

                            let insert_op =
                                dict.insert(message.args[1].clone(), message.args[2].clone());

                            // TODO: Add returning old value
                            match insert_op {
                                Some(old) => (),
                                None => (),
                            }

                            let response = [
                                "$",
                                ret.len().to_string().as_str(),
                                "\r\n".to_string().as_str(),
                                (ret).as_str(),
                                "\r\n".to_string().as_str(),
                            ]
                            .concat();

                            let _ = stream.write_all(&response[..].as_bytes()).unwrap();

                            drop(dict);
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
