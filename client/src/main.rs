mod asteroid;
mod bullet;
mod collision;
#[cfg(not(target_arch = "wasm32"))]
mod network;
mod screen;
mod ship;
#[cfg(not(target_arch = "wasm32"))]
use crate::network::{connect_stream, connect_ws, deserialize_host_data, serialize_host_data};
use crate::ship::Ship;
use crate::{asteroid::Asteroid, collision::manage_collisions};
use macroquad::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use simple_logger::SimpleLogger;
use std::{net::TcpStream, thread::sleep, time::Duration};
#[cfg(not(target_arch = "wasm32"))]
use std::{sync::mpsc, thread};
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use tungstenite::Message;
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
    name: String,
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Planetoid"),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let opt = Opt::from_args();

    let log_level = match opt.debug {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Trace,
    };

    #[cfg(not(target_arch = "wasm32"))]
    SimpleLogger::new().with_level(log_level).init().unwrap();
    log::debug!("{:#?}", opt);
    log::info!("Starting game.");

    const MAX_ASTEROIDS: u8 = 10;
    let mut gameover = false;
    let mut last_shot = get_time();
    let mut players: Vec<Ship> = Vec::new();
    players.push(Ship::new(String::from(&opt.name)));
    players.push(Ship::new(String::from("Player 2")));
    players.push(Ship::new(String::from("Player 3")));
    players.push(Ship::new(String::from("Player 4")));

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
                deserialize_host_data(&opt.mode, msg, &mut asteroids, &mut players, &mut gameover);
                if !asteroids.is_empty() {
                    break;
                }
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
                        &opt.mode,
                        msg,
                        &mut asteroids,
                        &mut players,
                        &mut gameover,
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
                }
                frame_count = 0;
            }
        }

        if gameover {
            clear_background(LIGHTGRAY);
            let mut text = "You Win!. Press [enter] to play again.";
            let font_size = 30.;

            if !asteroids.is_empty() {
                text = "Game Over. Press [enter] to play again.";
            }

            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
            if opt.mode != "spectator" && is_key_down(KeyCode::Enter) {
                log::info!("Restarting game.");
                players.clear();
                players.push(Ship::new(String::from(&opt.name)));
                asteroids = Vec::new();
                gameover = false;
                for _ in 0..MAX_ASTEROIDS {
                    asteroids.push(Asteroid::new());
                }
                frame_count = 0;
            }

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

        manage_collisions(&mut players, &mut asteroids, opt.god, &opt.mode, frame_t);

        if asteroids.is_empty() || players.is_empty() {
            gameover = true;
        }

        clear_background(LIGHTGRAY);
        for ship in &players {
            for bullet in ship.bullets.iter() {
                bullet.draw();
            }
        }

        for asteroid in asteroids.iter() {
            asteroid.draw();
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
