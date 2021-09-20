mod asteroid;
mod bullet;
mod collision;
mod gameover;
#[cfg(not(target_arch = "wasm32"))]
mod network;
mod screen;
mod ship;
mod sound;
use crate::asteroid::Asteroids;
use crate::collision::manage_collisions;
#[cfg(not(target_arch = "wasm32"))]
use crate::network::{
    connect_stream, connect_ws, deserialize_host_data, serialize_guest_data, serialize_host_data,
    wait_synchronization_data,
};
use crate::{gameover::manage_gameover, ship::Ship};
use macroquad::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use simple_logger::SimpleLogger;
use sound::Sound;
#[cfg(not(target_arch = "wasm32"))]
use std::{net::TcpStream, sync::mpsc, thread, thread::sleep, time::Duration};
use structopt::clap::{crate_name, crate_version};
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use tungstenite::Message;
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(name = crate_name!(), version = crate_version!())]
/// Planetoid is an asteroid clone.

struct Opt {
    /// Debug mode (ϕ (error), -d (info), -dd (debug), -ddd (trace))
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

    /// Display fps
    #[structopt(short, long)]
    fps: bool,

    /// Player name
    #[structopt(short, long, default_value = "planetoid")]
    name: String,
}

const MAX_ASTEROIDS: usize = 10;

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
        _ => log::LevelFilter::Error,
    }
}

fn display_fps(fps: &mut i32, frame_t: f64, fps_refresh: &mut f64) {
    if frame_t - *fps_refresh > 0.2 {
        *fps = get_fps();
        *fps_refresh = frame_t;
    }
    let text = format!("{} fps", fps);
    let font_size = 30.;
    draw_text(&text, 5., 20., font_size, DARKGRAY)
}

#[macroquad::main(window_conf)]
async fn main() {
    // Seed random generator
    rand::srand(miniquad::date::now() as u64);

    let opt = Opt::from_args();

    #[cfg(not(target_arch = "wasm32"))]
    let log_level = get_log_level(opt.debug);

    #[cfg(not(target_arch = "wasm32"))]
    SimpleLogger::new().with_level(log_level).init().unwrap();
    log::debug!("{:#?}", opt);
    log::info!("Starting game.");

    let mut show_fps = opt.fps;
    let mut fps: i32 = 0;
    let mut gameover = false;
    let mut gameover_msg_sent = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut host_msg_received: bool = false;
    // Timing values
    let mut lastshot_t = get_time();
    let mut thrust_t = get_time();
    let mut fps_t = get_time();
    let mut debounce_t = get_time();

    let mut sound = Sound::new().await;

    #[allow(unused_mut)]
    let mut sync_t: f64 = 0.0;
    let mut players: Vec<Ship> = vec![Ship::new(String::from(&opt.name))];

    let mut asteroids: Asteroids = Asteroids::generate_field(opt.name.clone(), 0);
    if opt.mode == "host" {
        asteroids = Asteroids::generate_field(opt.name.clone(), MAX_ASTEROIDS);
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

        // Thread to manage network web socket
        // This thread uses a channel to pass messages to the main thread (game)
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

        wait_synchronization_data(
            &rx_from_socket,
            &tx_to_socket,
            &opt.name,
            &opt.mode,
            &mut asteroids,
            &mut players,
            &mut gameover,
            &mut host_msg_received,
            &mut sync_t,
        );
    }

    let mut frame_count: u32 = 0;
    let time_before_entering_loop = get_time();

    // Game loop
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if !opt.solo {
            loop {
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
                    Err(mpsc::TryRecvError::Empty) => (break),
                    Err(mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
                };
            }

            if frame_count > 4 && opt.mode == "host" {
                tx_to_socket
                    .send(serialize_host_data(
                        &mut asteroids,
                        &mut players,
                        &mut gameover,
                    ))
                    .unwrap();
                frame_count = 0;
            }

            if host_msg_received && opt.mode == "guest" {
                for ship in players.iter() {
                    if ship.name() == opt.name {
                        tx_to_socket
                            .send(serialize_guest_data(ship, &mut asteroids))
                            .unwrap();
                    }
                }
                host_msg_received = false;
            }
        }

        if gameover {
            // Send a last message to all guests that the game is over
            #[cfg(not(target_arch = "wasm32"))]
            if !opt.solo && opt.mode == "host" && !gameover_msg_sent {
                tx_to_socket
                    .send(serialize_host_data(
                        &mut asteroids,
                        &mut players,
                        &mut gameover,
                    ))
                    .unwrap();
                frame_count = 0;
                gameover_msg_sent = true;
            }

            manage_gameover(
                &mut players,
                &mut asteroids,
                &opt.mode,
                &opt.name,
                &mut frame_count,
                &mut gameover,
                &mut gameover_msg_sent,
                &mut sound,
            );

            // Display frame but do not increase frame_count to not send new messages
            next_frame().await;

            // Guest will be blocked waiting for the next message from the host
            // The host will send a new message as soon as the user will hit enter
            #[cfg(not(target_arch = "wasm32"))]
            if !opt.solo {
                wait_synchronization_data(
                    &rx_from_socket,
                    &tx_to_socket,
                    &opt.name,
                    &opt.mode,
                    &mut asteroids,
                    &mut players,
                    &mut gameover,
                    &mut host_msg_received,
                    &mut sync_t,
                );
            }
            continue;
        }

        let frame_t = get_time() - time_before_entering_loop;
        for ship in players.iter_mut() {
            ship.slow_down();
        }

        if opt.mode != "spectator" {
            if is_key_down(KeyCode::Up) {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name && !ship.collided() {
                        ship.accelerate();
                        if frame_t - thrust_t > 0.5 {
                            sound.thrust();
                            thrust_t = frame_t;
                        }
                    }
                }
            }

            if is_key_down(KeyCode::Space) && frame_t - lastshot_t > 0.1 {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name && !ship.collided() {
                        ship.shoot(frame_t);
                        sound.laser();
                    }
                }
                lastshot_t = frame_t;
            }

            if is_key_down(KeyCode::Right) {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name && !ship.collided() {
                        ship.set_rot(ship.rot() + 5.);
                    }
                }
            } else if is_key_down(KeyCode::Left) {
                for ship in players.iter_mut() {
                    if ship.name() == opt.name && !ship.collided() {
                        ship.set_rot(ship.rot() - 5.);
                    }
                }
            }
        }
        if is_key_down(KeyCode::F) && frame_t - debounce_t > 0.2 {
            if show_fps {
                show_fps = false;
            } else {
                show_fps = true;
            }
            debounce_t = frame_t;
        }

        if cfg!(not(target_arch = "wasm32")) && is_key_down(KeyCode::Escape) {
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

        for asteroid in asteroids.get_asteroids().values_mut() {
            asteroid.update_pos();
        }

        manage_collisions(
            &mut players,
            &mut asteroids,
            opt.name.clone(),
            opt.god,
            &opt.mode,
            frame_t,
            sync_t,
        );

        if players
            .iter()
            .any(|ship| ship.name() == opt.name && ship.collided())
        {
            sound.explosion();
        }

        if asteroids.is_empty() || players.iter().all(|ship| ship.collided()) {
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

        for asteroid in asteroids.get_asteroids().values_mut() {
            if !asteroid.collided() {
                asteroid.draw();
            }
        }

        for ship in &players {
            if !ship.collided() {
                if ship.name() == opt.name {
                    ship.draw(BLACK);
                } else {
                    ship.draw(RED);
                }
            }
        }

        log::trace!("{} fps", get_fps());
        if show_fps {
            display_fps(&mut fps, frame_t, &mut fps_t);
        }
        next_frame().await;
        frame_count += 1;
    }
}
