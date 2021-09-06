extern crate websocket;

mod types;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

fn main() {
    let address = String::from("192.168.1.128:3000");
    let server = Server::bind(&address).unwrap();

    println!("Server started at address {}", &address);

    let connections = server.filter_map(Result::ok);

    for connection in connections {
        thread::spawn(move || {
            let mut client = connection.accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let message = OwnedMessage::Text("Connected to server".to_string());
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);

                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Text(m) => {
                        let raw = String::from(&m);
                        let parsed: Result<types::SocketEvent, serde_json::Error> =
                            serde_json::from_str(&raw.to_string());

                        match parsed {
                            Ok(data) => endpoint_handler(data),
                            Err(err) => {
                                let error = OwnedMessage::Text(
                                    ["Unable to parse JSON:", &err.to_string()]
                                        .join(" ")
                                        .to_string(),
                                );
                                sender.send_message(&error).unwrap();
                            }
                        }
                    }
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}

// todo: send message to all connected clients
#[allow(dead_code)]
fn broadcast_message(msg: &str) {
    println!("The broadcasted message is: {}", &msg);
}

// todo: add branches for each event type (start game, join game, make turn, etc)
fn endpoint_handler(data: types::SocketEvent) {
    println!("The incoming websocket is: {:?}", &data);

    match data.event.as_str() {
        "message" => {
            println!("Event is a message");
        }
        "data" => {
            println!("Event is data");
        }
        _ => {
            println!("Unknown event type");
        }
    }
}
