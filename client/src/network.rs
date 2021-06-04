use crate::{asteroid::Asteroid, ship::Ship};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    net::{TcpStream, ToSocketAddrs},
};
use tungstenite::{client, http::Response};
use tungstenite::{Message, WebSocket};
use url::Url;

type WebSocketResult<T> = Result<T, Box<dyn Error>>;
pub fn connect_stream(url: &Url) -> TcpStream {
    let addr = (url.host_str().unwrap(), url.port().unwrap())
        .to_socket_addrs()
        .unwrap()
        .last()
        .expect("Cannot get host and port from url.");

    log::debug!("Connect to TcpStream {}:{}", addr.ip(), addr.port());
    TcpStream::connect(addr).expect("Cannot connect to specified address.")
}

pub fn connect_ws(
    url: Url,
    stream: &TcpStream,
) -> WebSocketResult<(WebSocket<&TcpStream>, Response<()>)> {
    log::debug!("Connect to WebSocket url {}", url);
    let (socket, response) = client(url, stream).expect("Cannot connect to specified url.");

    stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    log::info!("Connected to the server");
    log::info!("Response HTTP code: {}", response.status());

    Ok((socket, response))
}

#[derive(Serialize, Deserialize)]
struct GameData {
    asteroids: Vec<Asteroid>,
    players: Vec<Ship>,
    gameover: bool,
}

pub fn deserialize_host_data(
    mode: &str,
    msg: Message,
    asteroids: &mut Vec<Asteroid>,
    players: &mut Vec<Ship>,
    gameover: &mut bool,
) {
    if let Message::Text(msg) = msg {
        // Uggly hack to manage msg
        if !msg.contains("joined") {
            log::debug!("{}", msg);

            if mode != "host" {
                asteroids.clear();

                // ship.clear();
                let gamedata: GameData = serde_json::from_str(&msg).unwrap();
                *asteroids = gamedata.asteroids;
                *players = gamedata.players;
                *gameover = gamedata.gameover;
            }
        }
    }
}

pub fn serialize_host_data(
    asteroids: &mut Vec<Asteroid>,
    players: &mut Vec<Ship>,
    gameover: &mut bool,
) -> String {
    let gamedata = GameData {
        asteroids: asteroids.to_vec(),
        players: players.clone(),
        gameover: *gameover,
    };

    serde_json::to_string(&gamedata).unwrap()
}
