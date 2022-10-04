use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use vydws::{get_localhost, get_unused_port};

fn init_ws() {
    let port = get_unused_port(9024);
    println!(
        "your local dev server is runing at: ws://127.0.0.1:{}",
        port
    );
    let server = TcpListener::bind(&get_localhost(port)).unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg_result = websocket.read_message();

                match msg_result {
                    Ok(msg) => {
                        // We do not want to send back ping/pong messages.
                        if msg.is_binary() || msg.is_text() {
                            websocket.write_message(msg).unwrap();
                        }
                    }
                    Err(err) => {
                        // closed etc.
                    }
                }
            }
        });
    }
}
/// A WebSocket echo server
fn main() {
    init_ws();
}
