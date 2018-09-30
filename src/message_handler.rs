//Handles messages from the server, each in their own thread
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn handle_message(msg: String, handlers: HashMap<String, String>, pin_locks: Arc<Vec<Mutex<i8>>>) {
}
