//Handles messages from the server, each in their own thread
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

const NUM_PINS: i8 = 15;

pub struct MessageHandler {
    handlers: HashMap<String, String>,
    pin_locks: Arc<Vec<Mutex<i8>>>,
}

impl MessageHandler {
    pub fn new(handlers: HashMap<String, String>) -> MessageHandler {
        let mut pin_locks: Vec<Mutex<i8>> = Vec::new();
        for lock in 0..NUM_PINS {
            pin_locks.push(Mutex::new(0));
        }

        MessageHandler {
            handlers: handlers,
            pin_locks: Arc::new(pin_locks),
        }
    }

    pub fn handle_message(&mut self, msg: String) {
        let pin_locks = Arc::clone(&self.pin_locks);
    }
}
    // Start new thread
    // Get contents, break up and parse / order pin numbers
    // Get locks for all required pins
    // exec commands
    // scope releases pin locks
