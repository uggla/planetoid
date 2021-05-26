use std::error::Error;
use tungstenite::http::Response;
use tungstenite::{client::AutoStream, connect, Message, WebSocket};
use url::Url;

use crate::asteroid::Asteroid;

pub fn connect_ws(url: &str) -> Result<(WebSocket<AutoStream>, Response<()>), Box<dyn Error>> {
    let (socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect.");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    Ok((socket, response))
}

pub fn deserialize_host_data(mode: &str, msg: Message, asteroids: &mut Vec<Asteroid>) {
    if let Message::Text(msg) = msg {
        // Uggly hack to manage msg
        if !msg.contains("joined") {
            // let msg = msg.strip_prefix(">>  : ").unwrap().to_string();
            println!("{}", msg);

            if mode != "host" {
                asteroids.clear();
                *asteroids = serde_json::from_str(&msg).unwrap();
            }
        }
    }
}

pub fn serialize_host_data(asteroids: &mut Vec<Asteroid>) -> String {
    serde_json::to_string(&asteroids).unwrap()
}
