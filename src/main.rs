mod asteroid;
mod bullet;
mod screen;
mod ship;
use crate::asteroid::Asteroid;
use crate::bullet::Bullet;
use crate::ship::Ship;
use macroquad::prelude::*;
use std::thread;
use tungstenite::{connect, Message};
use url::Url;

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
    info!("Starting game.");
    let mut gameover = false;
    let mut last_shot = get_time();
    let mut ship = Ship::new();

    let mut bullets = Vec::new();
    let mut asteroids = Vec::new();

    for _ in 0..10 {
        asteroids.push(Asteroid::new());
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
            next_frame().await;
            continue;
        }
        let frame_t = get_time();

        let mut acc = -ship.vel() / 30.0;
        if is_key_down(KeyCode::Up) {
            acc = Vec2::new(ship.rotation().sin(), -ship.rotation().cos()) / 3.;
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

        ship.update_pos(acc);

        for bullet in bullets.iter_mut() {
            bullet.update_pos();
        }
        for asteroid in asteroids.iter_mut() {
            asteroid.update_pos();
        }

        bullets.retain(|bullet| bullet.shot_at() + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            if (asteroid.pos() - ship.pos()).length() < asteroid.size() + Ship::HEIGHT / 3. {
                gameover = true;
                break;
            }
            for bullet in bullets.iter_mut() {
                if (asteroid.pos() - bullet.pos()).length() < asteroid.size() {
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

        if gameover {
            continue;
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
