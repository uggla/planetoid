mod asteroid;
mod bullet;
mod collision;
#[cfg(not(target_arch = "wasm32"))]
mod network;
mod screen;
mod ship;
use crate::bullet::Bullet;
use crate::collision::Collided;
#[cfg(not(target_arch = "wasm32"))]
use crate::network::{connect_ws, deserialize_host_data, serialize_host_data};
use crate::ship::Ship;
use crate::{asteroid::Asteroid, collision::is_collided};
use macroquad::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::{sync::mpsc, thread};
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use tungstenite::Message;

#[derive(StructOpt, Debug)]
#[structopt(name = "planetoid", version = "0.1.0")]
/// Planetoid is a asteroid clone

struct Opt {
    /// Url
    #[structopt(
        short,
        long,
        default_value = "ws://localhost:8080/gamedata/planetoid_host"
    )]
    url: String,

    /// God mode
    #[structopt(short, long)]
    god: bool,

    /// Network mode
    #[structopt(short, long, default_value = "host", possible_values = &["host","guest","spectator"])]
    mode: String,

    /// Solo mode
    /// Do not connect to network
    #[structopt(short, long)]
    solo: bool,
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
    println!("{:#?}", opt);
    info!("Starting game.");
    const MAX_ASTEROIDS: u8 = 10;
    let mut gameover = false;
    let mut last_shot = get_time();
    let mut ship = Ship::new();

    let mut bullets = Vec::new();
    let mut asteroids = Vec::new();

    if opt.mode == "host" {
        for _ in 0..MAX_ASTEROIDS {
            asteroids.push(Asteroid::new());
        }
    }

    // let mut asteroids_serde = Vec::new();
    // for asteroid in &asteroids {
    //     asteroids_serde.push(asteroid.to_serde());
    // }
    // let serialized = serde_json::to_string(&asteroids_serde).unwrap();
    // println!("{}", serialized);

    // asteroids.clear();
    // asteroids_serde.clear();
    // asteroids_serde = serde_json::from_str(&serialized).unwrap();
    // // dbg!(&asteroids_serde);
    // for asteroid in &asteroids_serde {
    //     asteroids.push(Asteroid::from_serde(asteroid));
    // }

    #[cfg(not(target_arch = "wasm32"))]
    let (tx_from_socket, rx_from_socket) = mpsc::channel();
    #[cfg(not(target_arch = "wasm32"))]
    let (tx_to_socket, rx_to_socket) = mpsc::channel();

    #[cfg(not(target_arch = "wasm32"))]
    if !opt.solo {
        let url = opt.url.clone();
        let mode = opt.mode.clone();

        thread::spawn(move || {
            let (mut socket, _response) = connect_ws(&url).unwrap();
            loop {
                // let _received = match rx_to_socket.try_recv() {
                //     Ok(msg) => socket
                //         .write_message(Message::Text(msg))
                //         .expect("Cannot write to socket."),
                //     Err(mpsc::TryRecvError::Empty) => (),
                //     Err(mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
                // };
                if mode == "host" {
                    let received = rx_to_socket.recv().unwrap();
                    socket
                        .write_message(Message::Text(received))
                        .expect("Cannot write to socket.");
                }
                let msg = socket.read_message().expect("Error reading message");
                tx_from_socket.send(msg).unwrap();
            }
        });

        if opt.mode != "host" {
            println!("Waiting synchronization data");
            loop {
                let msg = rx_from_socket.recv().unwrap();
                deserialize_host_data(&opt.mode, msg, &mut asteroids, &mut bullets);
                if !asteroids.is_empty() {
                    break;
                }
            }
        }
    }

    let mut frame_count: u32 = 0;

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if !opt.solo {
            let _received = match rx_from_socket.try_recv() {
                Ok(msg) => {
                    deserialize_host_data(&opt.mode, msg, &mut asteroids, &mut bullets);
                }
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
            };

            if frame_count > 4 {
                if opt.mode == "host" {
                    tx_to_socket
                        .send(serialize_host_data(&mut asteroids, &mut bullets))
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

            if is_key_down(KeyCode::Enter) {
                info!("Restarting game.");
                ship = Ship::new();
                bullets = Vec::new();
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

        let frame_t = get_time();
        ship.slow_down();

        if is_key_down(KeyCode::Up) {
            ship.accelerate();
        }

        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.1 {
            let rot_vec = Vec2::new(ship.rotation().sin(), -ship.rotation().cos());
            bullets.push(Bullet::new(
                ship.pos() + rot_vec * Ship::HEIGHT / 2.,
                rot_vec * 7.,
                frame_t,
                false,
            ));
            last_shot = frame_t;
        }

        if is_key_down(KeyCode::Right) {
            ship.set_rot(ship.rot() + 5.);
        } else if is_key_down(KeyCode::Left) {
            ship.set_rot(ship.rot() - 5.);
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        ship.update_pos();

        for bullet in bullets.iter_mut() {
            bullet.update_pos();
        }
        for asteroid in asteroids.iter_mut() {
            asteroid.update_pos();
        }

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            if is_collided(asteroid, &ship) && (opt.mode != "spectator" && !opt.god) {
                gameover = true;
                break;
            }
            for bullet in bullets.iter_mut() {
                if is_collided(asteroid, bullet) {
                    asteroid.set_collided(true);
                    bullet.set_collided(true);
                    if asteroid.sides() > 4 {
                        new_asteroids = Asteroid::new_split(
                            asteroid.pos(),
                            bullet.vel().x,
                            bullet.vel().y,
                            asteroid.size(),
                            asteroid.sides(),
                        );
                    }
                    break;
                }
            }
        }

        bullets.retain(|bullet| bullet.shot_at() + 1.5 > frame_t && !bullet.collided());
        asteroids.retain(|asteroid| !asteroid.collided());
        asteroids.append(&mut new_asteroids);

        if asteroids.is_empty() {
            gameover = true;
        }

        clear_background(LIGHTGRAY);
        for bullet in bullets.iter() {
            bullet.draw();
        }

        for asteroid in asteroids.iter() {
            asteroid.draw();
        }

        ship.draw();

        //println!("{} fps", get_fps());
        next_frame().await;
        frame_count += 1;
    }
}
