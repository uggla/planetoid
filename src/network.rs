use std::error::Error;
use tungstenite::http::Response;
use tungstenite::{client::AutoStream, connect, Message, WebSocket};
use url::Url;

pub fn connect_ws() -> Result<(WebSocket<AutoStream>, Response<()>), Box<dyn Error>> {
    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8080/chat/rust-ws").unwrap()).expect("Can't connect.");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    // socket
    //     .write_message(Message::Text("Hello WebSocket".into()))
    //     .expect("Cannot write to socket.");

    Ok((socket, response))
}
