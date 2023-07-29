use std::{io::Write, net::TcpStream, time::Duration};

use std::sync::Arc;
use std::time::SystemTime;

use crate::{message::message::Message, send_nil, structs::app_state::AppState};

pub fn handle_get(app_state: &Arc<AppState>, message: &Message, stream: &mut TcpStream) {
    let dict = app_state.dict.lock().unwrap();

    let entry = dict.get(&message.args[1]);

    let mut val: Option<String> = None;

    match entry {
        Some(refer) => match &refer.1 {
            Some(px_time) => {
                let now = SystemTime::now();

                let is_already_past =
                    px_time.0 + Duration::from_millis(px_time.1.parse::<u64>().unwrap()) < now;

                if is_already_past {
                    send_nil(stream);
                } else {
                    val = Some(refer.0.clone());
                }
            }
            None => {
                val = Some(refer.0.clone());
            }
        },
        None => send_nil(stream),
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
        None => send_nil(stream),
    }

    drop(dict);
}
