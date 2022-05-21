use crate::asteroid::synchronize_asteroids;
use crate::{asteroid::Asteroids, ship::Ship};
use macroquad::prelude::get_time;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, Sender};
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
    asteroids: Asteroids,
    players: Vec<Ship>,
    gameover: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn deserialize_host_data(
    name: &str,
    mode: &str,
    msg: Message,
    asteroids: &mut Asteroids,
    players: &mut Vec<Ship>,
    gameover: &mut bool,
    host_msg_received: &mut bool,
    sync_t: &mut f64,
) {
    if let Message::Text(msg) = msg {
        log::debug!("{}", msg);
        if msg.contains("Hello from ") {
            let name = msg.strip_prefix("Hello from ").unwrap();
            players.push(Ship::new(String::from(name)));
            *sync_t = get_time();
            asteroids.refresh_last_updated(get_time() - *sync_t);
        }

        if mode == "host" && msg.contains("GuestData: ") {
            let msg = msg.strip_prefix("GuestData: ").unwrap();
            let guestdata: GuestData = serde_json::from_str(msg).unwrap();
            let opponent = guestdata.ship;
            for ship in players.iter_mut() {
                if ship.name() == opponent.name() {
                    *ship = opponent.clone();
                }
            }
            synchronize_asteroids(asteroids, guestdata.asteroids);
        }

        if mode != "host" && msg.contains("GameData: ") {
            let msg = msg.strip_prefix("GameData: ").unwrap();

            // Backup player ship
            let mut current_ship: Ship = Ship::new(name.to_string());
            for ship in players.clone() {
                if ship.name() == name {
                    current_ship = ship;
                }
            }

            let gamedata: GameData = serde_json::from_str(msg).unwrap();
            synchronize_asteroids(asteroids, gamedata.asteroids);
            *gameover = gamedata.gameover;
            *players = gamedata.players;

            // Restore current ship
            for ship in players {
                if ship.name() == name {
                    *ship = current_ship.clone();
                }
            }
            *host_msg_received = true;
        }
    }
}

pub fn serialize_host_data(
    asteroids: &mut Asteroids,
    players: &mut [Ship],
    gameover: &mut bool,
) -> String {
    let gamedata = GameData {
        asteroids: asteroids.clone(),
        players: players.to_vec(),
        gameover: *gameover,
    };

    format!("GameData: {}", serde_json::to_string(&gamedata).unwrap())
}

#[derive(Serialize, Deserialize)]
struct GuestData {
    asteroids: Asteroids,
    ship: Ship,
}

pub fn serialize_guest_data(ship: &Ship, asteroids: &mut Asteroids) -> String {
    let guestdata = GuestData {
        asteroids: asteroids.clone(),
        ship: ship.clone(),
    };
    format!("GuestData: {}", serde_json::to_string(&guestdata).unwrap())
}

#[allow(clippy::too_many_arguments)]
pub fn wait_synchronization_data(
    rx_from_socket: &Receiver<Message>,
    tx_to_socket: &Sender<String>,
    name: &str,
    mode: &str,
    asteroids: &mut Asteroids,
    players: &mut Vec<Ship>,
    gameover: &mut bool,
    host_msg_received: &mut bool,
    sync_t: &mut f64,
) {
    if mode != "host" {
        log::info!("Waiting synchronization data");
        loop {
            let msg = rx_from_socket.recv().unwrap();
            deserialize_host_data(
                name,
                mode,
                msg,
                asteroids,
                players,
                gameover,
                host_msg_received,
                sync_t,
            );
            if !asteroids.is_empty() {
                break;
            }
        }

        if mode == "guest" {
            tx_to_socket.send(format!("Hello from {}", name)).unwrap();
        }
    }
}
