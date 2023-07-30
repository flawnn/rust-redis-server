use std::{net::TcpListener, thread};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::core::networking::handle_client;
use crate::structs::app_state::AppState;

mod core;
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
