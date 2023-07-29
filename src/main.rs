use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::message::message::Message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

mod message;

struct AppState {
    dict: Mutex<HashMap<String, Entry>>,
}

struct Entry(String, Option<EntryDate>);
struct EntryDate(SystemTime, String);

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
                            let dict = app_state.dict.lock().unwrap();

                            let entry = dict.get(&message.args[1]);

                            let mut val: Option<String> = None;

                            match entry {
                                Some(refer) => match &refer.1 {
                                    Some(px_time) => {
                                        let now = SystemTime::now();

                                        let is_already_past = px_time.0
                                            + Duration::from_millis(
                                                px_time.1.parse::<u64>().unwrap(),
                                            )
                                            < now;

                                        if is_already_past {
                                            send_nil(&mut stream);
                                            continue;
                                        } else {
                                            val = Some(refer.0.clone());
                                        }
                                    }
                                    None => {
                                        val = Some(refer.0.clone());
                                    }
                                },
                                None => send_nil(&mut stream),
                            }

                            // If val is not empty, we got a value back
                            match val {
                                Some(val) => {
                                    let response = [
                                        "$",
                                        val.len().to_string().as_str(),
                                        "\r\n".to_string().as_str(),
                                        (val).as_str(),
                                        "\r\n".to_string().as_str(),
                                    ]
                                    .concat();
                                    let _ = stream.write_all(&response[..].as_bytes()).unwrap();
                                }
                                None => send_nil(&mut stream),
                            }

                            drop(dict);
                        }

                        if message.command == "set" {
                            let mut dict = app_state.dict.lock().unwrap();

                            let mut insert_op: Option<Entry> = None;
                            let mut inserted: bool = false;

                            if let Some(third_arg) = message.args.get(3) {
                                if third_arg == "px" {
                                    if let Some(exp_time) = message.args.get(4) {
                                        insert_op = dict.insert(
                                            message.args[1].clone(),
                                            Entry(
                                                message.args[2].clone(),
                                                Some(EntryDate(
                                                    SystemTime::now(),
                                                    exp_time.to_string(),
                                                )),
                                            ),
                                        );

                                        inserted = true;
                                    } else {
                                        send_nil(&mut stream);
                                    }
                                }
                            } else {
                                insert_op = dict.insert(
                                    message.args[1].clone(),
                                    Entry(message.args[2].clone(), None),
                                );

                                inserted = true;
                            }

                            // TODO: Add returning old value
                            match insert_op {
                                Some(old) => (),
                                None => (),
                            }

                            if inserted {
                                send_ok(&mut stream);
                            }

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

fn send_nil(stream: &mut TcpStream) {
    let _ = stream.write_all(&NIL_STRING[..].as_bytes()).unwrap();
}

fn send_ok(stream: &mut TcpStream) {
    let _ = stream.write_all(&OK_STRING[..].as_bytes()).unwrap();
}
