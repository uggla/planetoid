use std::error::Error;
use tungstenite::http::Response;
use tungstenite::{client::AutoStream, connect, Message, WebSocket};
use url::Url;

use crate::asteroid::{Asteroid, AsteroidSerde};

pub fn connect_ws(url: &str) -> Result<(WebSocket<AutoStream>, Response<()>), Box<dyn Error>> {
    let (mut socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect.");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    // socket
    //     .write_message(Message::Text("Hello WebSocket".into()))
    //     .expect("Cannot write to socket.");

    Ok((socket, response))
}

pub fn deserialize_host_data(mode: &str, msg: Message, asteroids: &mut Vec<Asteroid>) {
    if let Message::Text(msg) = msg {
        // Uggly hack to manage msg
        if !msg.contains("joined") {
            // let msg = msg.strip_prefix(">>  : ").unwrap().to_string();
            println!("{}", msg.to_string());

            if mode != "host" {
                let asteroids_serde: Vec<AsteroidSerde> = serde_json::from_str(&msg).unwrap();

                asteroids.clear();
                for asteroid in asteroids_serde {
                    asteroids.push(Asteroid::from_serde(&asteroid));
                }
            }
        }
    }
}

pub fn serialize_host_data(asteroids: &mut Vec<Asteroid>) -> String {
    let mut asteroids_serde = Vec::new();
    for asteroid in asteroids {
        asteroids_serde.push(asteroid.to_serde());
    }
    let serialized = serde_json::to_string(&asteroids_serde).unwrap();
    serialized
}
