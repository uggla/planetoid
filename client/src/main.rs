mod asteroid;
mod bullet;
mod collision;
mod gameover;
#[cfg(not(target_arch = "wasm32"))]
mod network;
mod screen;
mod ship;
#[cfg(not(target_arch = "wasm32"))]
use crate::network::{
    connect_stream, connect_ws, deserialize_host_data, serialize_guest_data, serialize_host_data,
};
use crate::{asteroid::Asteroid, collision::manage_collisions};
use crate::{gameover::manage_gameover, ship::Ship};
use macroquad::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use simple_logger::SimpleLogger;
#[cfg(not(target_arch = "wasm32"))]
use std::{net::TcpStream, sync::mpsc, thread, thread::sleep, time::Duration};
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use tungstenite::Message;
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(name = "planetoid", version = "0.1.0")]
/// Planetoid is a asteroid clone

struct Opt {
    /// Debug mode (_ (error), -d (info), -dd (debug), -ddd (trace))
    #[structopt(short, long, parse(from_occurrences))]
    debug: u8,

    /// Host
    #[structopt(short, long, default_value = "localhost")]
    host: String,

    /// Port
    #[structopt(short, long, default_value = "8080")]
    port: u16,

    /// God mode
    #[structopt(short, long)]
    god: bool,

    /// Network mode
    #[structopt(short, long, default_value = "host", possible_values = &["host","guest","spectator"])]
    mode: String,

    /// Solo mode, do not connect to network
    #[structopt(short, long, conflicts_with = "mode")]
    solo: bool,

    /// Player name
    #[structopt(short, long, default_value = "planetoid")]
    name: String,
}

const MAX_ASTEROIDS: u8 = 10;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Planetoid"),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,
        window_resizable: false,

        ..Default::default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_log_level(debug_occurence: u8) -> log::LevelFilter {
    match debug_occurence {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Trace,
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let opt = Opt::from_args();

    #[cfg(not(target_arch = "wasm32"))]
    let log_level = get_log_level(opt.debug);

    #[cfg(not(target_arch = "wasm32"))]
    SimpleLogger::new().with_level(log_level).init().unwrap();
    log::debug!("{:#?}", opt);
    log::info!("Starting game.");

    let mut gameover = false;
    let mut host_msg_received: bool = false;
    let mut last_shot = get_time();
    let mut sync_t = get_time();
    let mut players: Vec<Ship> = Vec::new();
    players.push(Ship::new(String::from(&opt.name)));
    // Temporary stuff for debugging
    // players.push(Ship::new(String::from("Player 2")));
    // players.push(Ship::new(String::from("Player 3")));
    // players.push(Ship::new(String::from("Player 4")));

    let mut asteroids = Vec::new();

    if opt.mode == "host" {
        for _ in 0..MAX_ASTEROIDS {
            asteroids.push(Asteroid::new());
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    let (tx_from_socket, rx_from_socket) = mpsc::channel();
    #[cfg(not(target_arch = "wasm32"))]
    let (tx_to_socket, rx_to_socket) = mpsc::channel();

    #[cfg(not(target_arch = "wasm32"))]
    if !opt.solo {
        let url = Url::parse(&format!(
            "ws://{}:{}/gamedata/{}",
            &opt.host, &opt.port, &opt.name
        ))
        .expect("Cannot parse url.");

        thread::spawn(move || {
            let stream: TcpStream = connect_stream(&url);
            let (mut socket, _response) = connect_ws(url, &stream).unwrap();
            loop {
                match rx_to_socket.try_recv() {
                    Ok(msg) => socket
                        .write_message(Message::Text(msg))
                        .expect("Cannot write to WebSocket."),
                    Err(mpsc::TryRecvError::Empty) => (),
                    Err(mpsc::TryRecvError::Disconnected) => panic!("Client disconnected."),
                };

                if let Ok(msg) = socket.read_message() {
                    tx_from_socket.send(msg).unwrap();
                }
                sleep(Duration::from_millis(5));
            }
        });

        if opt.mode != "host" {
            log::info!("Waiting synchronization data");
            loop {
                let msg = rx_from_socket.recv().unwrap();
                deserialize_host_data(
                    &opt.name,
                    &opt.mode,
                    msg,
                    &mut asteroids,
                    &mut players,
                    &mut gameover,
                    &mut host_msg_received,
                    &mut sync_t,
                );
                if !asteroids.is_empty() {
                    break;
                }
            }
            if opt.mode == "guest" {
                tx_to_socket
                    .send(format!("Hello from {}", opt.name))
                    .unwrap();
            }
        }
    }

    let mut frame_count: u32 = 0;
    let time_before_entering_loop = get_time();
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if !opt.solo {
            let _received = match rx_from_socket.try_recv() {
                Ok(msg) => {
                    deserialize_host_data(
                        &opt.name,
                        &opt.mode,
                        msg,
                        &mut asteroids,
                        &mut players,
                        &mut gameover,
                        &mut host_msg_received,
                        &mut sync_t,
                    );
                }
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
            };

            if frame_count > 4 {
                if opt.mode == "host" {
                    tx_to_socket
                        .send(serialize_host_data(
                            &mut asteroids,
                            &mut players,
                            &mut gameover,
                        ))
                        .unwrap();
                    frame_count = 0;
                }
            }

            if host_msg_received {
                if opt.mode == "guest" {
                    for ship in players.iter_mut() {
                        if ship.name() == opt.name {
                            tx_to_socket.send(serialize_guest_data(ship)).unwrap();
                        }
                    }
                    host_msg_received = false;
                    // frame_count = 0;
                }
            }
        }

        if gameover {
            manage_gameover(
                &mut players,
                &mut asteroids,
                &opt.mode,
                &opt.name,
                &mut frame_count,
                &mut gameover,
            );

            if is_key_down(KeyCode::Escape) {
                break;
            }
            next_frame().await;
            continue;
        }

        let frame_t = get_time() - time_before_entering_loop;
        for ship in players.iter_mut() {
            ship.slow_down();
        }

        if opt.mode != "spectator" {
            if is_key_down(KeyCode::Up) {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name {
                        ship.accelerate();
                    }
                }
            }

            if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.1 {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name {
                        ship.shoot(frame_t);
                    }
                }
                last_shot = frame_t;
            }

            if is_key_down(KeyCode::Right) {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name {
                        ship.set_rot(ship.rot() + 5.);
                    }
                }
            } else if is_key_down(KeyCode::Left) {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name {
                        ship.set_rot(ship.rot() - 5.);
                    }
                }
            }
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        for ship in players.iter_mut() {
            ship.update_pos();
        }

        for ship in players.iter_mut() {
            for bullet in ship.bullets.iter_mut() {
                bullet.update_pos();
            }
        }

        for asteroid in asteroids.iter_mut() {
            asteroid.update_pos();
        }

        // if opt.mode == "host" {
        manage_collisions(&mut players, &mut asteroids, opt.god, &opt.mode, frame_t);
        // }

        if asteroids.is_empty() || players.is_empty() {
            gameover = true;
        }

        clear_background(LIGHTGRAY);
        for ship in &players {
            for bullet in ship.bullets.iter() {
                if !bullet.collided() {
                    bullet.draw();
                }
            }
        }

        for asteroid in asteroids.iter() {
            if !asteroid.collided() {
                asteroid.draw();
            }
        }

        for ship in &players {
            if ship.name() == opt.name {
                ship.draw(BLACK);
            } else {
                ship.draw(RED);
            }
        }

        log::trace!("{} fps", get_fps());
        next_frame().await;
        frame_count += 1;
    }
}
