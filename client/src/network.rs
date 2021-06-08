use crate::{asteroid::Asteroid, collision::Collided, ship::Ship};
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
    name: &str,
    mode: &str,
    msg: Message,
    asteroids: &mut Vec<Asteroid>,
    players: &mut Vec<Ship>,
    gameover: &mut bool,
    host_msg_received: &mut bool,
) {
    if let Message::Text(msg) = msg {
        log::debug!("{}", msg);
        if mode == "host" {
            if msg.contains("Hello from ") {
                let name = msg.strip_prefix("Hello from ").unwrap();
                players.push(Ship::new(String::from(name)));
            }

            if msg.contains("GuestData: ") {
                let msg = msg.strip_prefix("GuestData: ").unwrap();
                let opponent: Ship = serde_json::from_str(&msg).unwrap();
                for ship in players.iter_mut() {
                    if ship.name() == opponent.name() {
                        *ship = opponent.clone();
                    }
                }
            }
        }

        if mode != "host" {
            if msg.contains("GameData: ") {
                let msg = msg.strip_prefix("GameData: ").unwrap();

                asteroids.clear();

                let mut current_ship: Ship = Ship::new(name.to_string());
                for ship in players.clone() {
                    if ship.name() == name {
                        current_ship = ship;
                    }
                }

                let gamedata: GameData = serde_json::from_str(&msg).unwrap();
                *asteroids = gamedata.asteroids;
                *gameover = gamedata.gameover;
                // for ship_index in 0..players.len() {
                //     if players[ship_index].name() != name {
                //         players[ship_index] = gamedata.players[ship_index].clone();
                *players = gamedata.players;
                // }
                // }
                for ship in players {
                    if ship.name() == name {
                        // ship.set_pos(current_ship.pos());
                        // ship.set_vel(current_ship.vel());
                        // ship.set_acc(current_ship.acc());
                        // ship.set_rot(current_ship.rot());
                        // ship.set_size(current_ship.size());
                        // ship.set_collided(current_ship.collided());
                        *ship = current_ship.clone();
                    }
                }
                *host_msg_received = true;
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

    format!("GameData: {}", serde_json::to_string(&gamedata).unwrap())
}

pub fn serialize_guest_data(ship: &Ship) -> String {
    format!("GuestData: {}", serde_json::to_string(ship).unwrap())
}
