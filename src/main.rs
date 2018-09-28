extern crate tungstenite;
use tungstenite::{Message, connect};

extern crate url;
use url::Url;

use std::collections::HashMap;

mod message_handler;
use message_handler::MessageHandler;

fn main() {
    println!("Hello, world!");

    //Read config (file and args)
    let server_url = "ws://192.168.0.200:3001";
    //Bail out on same errors as JS
    let mut handlers = HashMap::new();
    handlers.insert(
        "pin".to_string(),
        "pi-pin".to_string(),
    );

    let (mut socket, response) = connect(Url::parse(server_url).unwrap()).expect(format!("Can't connect to {}", server_url).as_str());
    let mut message_handler = MessageHandler::new(handlers);

    loop {
        let mut handshake_done = false;
        let mut handshake_stage = 0;
        let msg = socket.read_message().expect("Read failed");

        if msg.is_text() {
            let msg_text = msg.into_text().expect("Failed to get message text");

            if handshake_done {
                message_handler.handle_message(msg_text);
            } else {
                let handshake_response: String;

                let (handshake_done, handshake_response) = handshake(msg_text, handshake_stage);
                socket.write_message(Message::Text(handshake_response));
                handshake_stage += 1;
            }
        }
    }
}

fn handshake(msg: String, stage: u32) -> (bool, String) {
    let mut done = false;
    let mut handshake_response: String;

    match stage {
        0 => {
            if !msg.starts_with("controlify.io server2") {
                panic!("Unrecognised handshake from server");
            }

            handshake_response = "Some client info here".to_owned();
        }
        1 => {
            if msg.trim() == "ok" {
                //possibly debug here
            } else if msg.starts_with("deprecated") {
                //some warning here
            } else if msg.starts_with("unsupported") {
                panic!(format!("Error: {}", msg));
            }
            else {
                panic!("Unrecognised handshake response from server");
            }

            handshake_response = "ok".to_owned();
            done = true;
        }
        _ => { panic!("Call to handshake() after handshake finished"); }
    }

    (done, handshake_response)
}
