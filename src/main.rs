mod asteroid;
mod bullet;
mod collision;
mod screen;
mod ship;
use crate::bullet::Bullet;
use crate::collision::Collision;
use crate::ship::Ship;
use crate::{asteroid::Asteroid, collision::is_collided};
use macroquad::prelude::*;
use std::thread;
use structopt::StructOpt;
use tungstenite::{connect, Message};
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(name = "planetoid", version = "0.1.0")]
/// Planetoid is a asteroid clone

struct Opt {
    // /// Verbose mode (-v, -vv, -vvv, etc.)
    // #[structopt(short, long, parse(from_occurrences))]
    // verbose: u8,
    /// Address
    #[structopt(short, long, default_value = "localhost")]
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

    for _ in 0..10 {
        asteroids.push(Asteroid::new());
    }

    let mut asteroids_serde = Vec::new();
    for asteroid in &asteroids {
        asteroids_serde.push(asteroid.to_serde());
    }
    let serialized = serde_json::to_string(&asteroids_serde).unwrap();
    println!("{}", serialized);

    asteroids.clear();
    asteroids_serde.clear();
    dbg!(&asteroids_serde);
    asteroids_serde = serde_json::from_str(&&serialized).unwrap();
    dbg!(&asteroids_serde);
    for asteroid in &asteroids_serde {
        asteroids.push(Asteroid::from_serde(asteroid));
    }

    thread::spawn(|| {
        let (mut socket, response) =
            connect(Url::parse("ws://localhost:8080/chat/rust-ws").unwrap())
                .expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        socket
            .write_message(Message::Text("Hello WebSocket".into()))
            .unwrap();
        loop {
            let msg = socket.read_message().expect("Error reading message");
            println!("Received: {}", msg);
        }
    });

    loop {
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
            if is_collided(asteroid, &ship) {
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
        next_frame().await
    }
}
