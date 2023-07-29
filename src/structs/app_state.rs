use std::collections::HashMap;
use crate::structs::entry::Entry;
use std::sync::Mutex;

pub struct AppState {
    pub dict: Mutex<HashMap<String, Entry>>,
}