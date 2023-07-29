
use std::{time::SystemTime, sync::Arc, net::TcpStream};

use crate::{structs::{entry::Entry, app_state::AppState, entry_date::EntryDate}, send_nil, send_ok, message::message::Message};


pub fn handle_set(app_state: &Arc<AppState>, message: Message, stream: &mut TcpStream) {
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
                        Some(EntryDate(SystemTime::now(), exp_time.to_string())),
                    ),
                );

                inserted = true;
            } else {
                send_nil(stream);
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
        send_ok(stream);
    }

    drop(dict);
}

