extern crate tungstenite;
extern crate url;

use url::Url;
use tungstenite::{Message, connect, WebSocket};
use tungstenite::client::AutoStream;

fn main() {
    let mut handshake_done = false;
    let mut handshake_stage = 0;

    println!("Hello, world!");

    //Read config (file and args)
    let server_url = "ws://192.168.0.200:3001";
    //Bail out on same errors as JS
    //Open websoocket to server
    let (mut socket, response) = connect(Url::parse(server_url).unwrap()).expect(format!("Can't connect to {}", server_url).as_str());
    loop {
        let msg = socket.read_message().expect("Read failed");

        if msg.is_text() {
            let msg_text = msg.into_text().expect("Failed to get message text");

            if handshake_done {
            } else {
                handshake_done = handshake(msg_text, handshake_stage, &mut socket);
                handshake_stage += 1;
            }
        }
    }
    //Wait for incoming message
    //Get contents, break up and add lock / unlock commands first?
    //  Start new thread
    //  Have array of bools for pin locks in a mutex
    //  Get locks for all required pins
    //  exec commands
    //  release pin locks
}

fn handshake(msg: String, stage: u32, socket: &mut WebSocket<AutoStream>) -> bool {
    let mut done = false;

    match stage {
        0 => {
            if !msg.starts_with("controlify.io server2") {
                panic!("Unrecognised handshake from server");
            }

            socket.write_message(Message::Text("Some client info here".to_owned()));
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

            socket.write_message(Message::Text("ok".to_owned()));
            done = true;
        }
        _ => { panic!("Call to handshake() after handshake finished"); }
    }

    done
}
