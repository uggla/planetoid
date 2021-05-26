use crate::{asteroid::Asteroid, bullet::Bullet};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tungstenite::http::Response;
use tungstenite::{client::AutoStream, connect, Message, WebSocket};
use url::Url;

pub fn connect_ws(url: &str) -> Result<(WebSocket<AutoStream>, Response<()>), Box<dyn Error>> {
    let (socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect.");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    Ok((socket, response))
}

#[derive(Serialize, Deserialize)]
struct GameData {
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
}

pub fn deserialize_host_data(
    mode: &str,
    msg: Message,
    asteroids: &mut Vec<Asteroid>,
    bullets: &mut Vec<Bullet>,
) {
    if let Message::Text(msg) = msg {
        // Uggly hack to manage msg
        if !msg.contains("joined") {
            // let msg = msg.strip_prefix(">>  : ").unwrap().to_string();
            println!("{}", msg);

            if mode != "host" {
                asteroids.clear();
                bullets.clear();
                let gamedata: GameData = serde_json::from_str(&msg).unwrap();
                *asteroids = gamedata.asteroids;
                *bullets = gamedata.bullets;
            }
        }
    }
}

pub fn serialize_host_data(asteroids: &mut Vec<Asteroid>, bullets: &mut Vec<Bullet>) -> String {
    let gamedata = GameData {
        asteroids: asteroids.to_vec(),
        bullets: bullets.to_vec(),
    };

    serde_json::to_string(&gamedata).unwrap()
}
