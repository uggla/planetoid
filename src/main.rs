mod asteroid;
mod bullet;
mod collision;
#[cfg(not(target_arch = "wasm32"))]
mod network;
mod screen;
mod ship;
use crate::collision::Collided;
#[cfg(not(target_arch = "wasm32"))]
use crate::network::connect_ws;
use crate::ship::Ship;
use crate::{asteroid::Asteroid, collision::is_collided};
use crate::{asteroid::AsteroidSerde, bullet::Bullet};
use macroquad::prelude::*;
use std::sync::mpsc;
use std::thread;
use structopt::StructOpt;
#[cfg(not(target_arch = "wasm32"))]
use tungstenite::Message;
#[derive(StructOpt, Debug)]
#[structopt(name = "planetoid", version = "0.1.0")]
/// Planetoid is a asteroid clone

struct Opt {
    /// Address
    #[structopt(short, long, default_value = "ws://localhost:8080/chat/planetoid_host")]
    address: String,

    /// Port
    #[structopt(short, long, default_value = "8080")]
    port: u16,

    /// Network mode
    #[structopt(short, long, default_value = "host", possible_values = &["host","guest","spectator"])]
    mode: String,
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
    let mut gameover = false;
    let mut last_shot = get_time();
    let mut ship = Ship::new();

    let mut bullets = Vec::new();
    let mut asteroids = Vec::new();

    if opt.mode == "host" {
        for _ in 0..2 {
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

    let url = opt.address.clone();
    let mode = opt.mode.clone();

    #[cfg(not(target_arch = "wasm32"))]
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
            // println!("Received: {}", msg);
            tx_from_socket.send(msg).unwrap();
        }
    });

    // if opt.mode == "guest" {
    //     let received = rx_from_socket.recv().unwrap();
    //     if let Message::Text(msg) = received {
    //         // Uggly hack to manage msg
    //         if !msg.contains("joined") {
    //             let msg = msg.strip_prefix(">> rust-ws: ").unwrap().to_string();
    //             println!("{}", msg.to_string());

    //             let asteroids_serde: Vec<AsteroidSerde> = serde_json::from_str(&msg).unwrap();

    //             asteroids.clear();
    //             for asteroid in asteroids_serde {
    //                 asteroids.push(Asteroid::from_serde(&asteroid));
    //             }
    //         }
    //     }
    // }

    let mut frame_count: u32 = 0;
    loop {
        #[cfg(not(target_arch = "wasm32"))]
        let _received = match rx_from_socket.try_recv() {
            Ok(msg) => {
                if let Message::Text(msg) = msg {
                    // Uggly hack to manage msg
                    if !msg.contains("joined") && !msg.contains("guest") {
                        // let msg = msg.strip_prefix(">>  : ").unwrap().to_string();
                        println!("{}", msg.to_string());

                        if opt.mode != "host" {
                            let asteroids_serde: Vec<AsteroidSerde> =
                                serde_json::from_str(&msg).unwrap();

                            asteroids.clear();
                            for asteroid in asteroids_serde {
                                asteroids.push(Asteroid::from_serde(&asteroid));
                            }
                        }
                    }
                }
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => panic!("Disconnected"),
        };

        if frame_count > 4 {
            if opt.mode == "host" {
                let mut asteroids_serde = Vec::new();
                for asteroid in &asteroids {
                    asteroids_serde.push(asteroid.to_serde());
                }
                let serialized = serde_json::to_string(&asteroids_serde).unwrap();
                tx_to_socket.send(serialized).unwrap();
            } else {
                // tx_to_socket.send("guest".to_string()).unwrap();
            }
            frame_count = 0;
        }

        if gameover {
            clear_background(LIGHTGRAY);
            let mut text = "You Win!. Press [enter] to play again.";
            let font_size = 30.;

            if asteroids.len() > 0 {
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
                for _ in 0..10 {
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
            if is_collided(asteroid, &ship) && opt.mode != "spectator" {
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

        if asteroids.len() == 0 {
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
