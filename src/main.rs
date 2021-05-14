use macroquad::prelude::*;
use std::thread;
use tungstenite::{connect, Message};
use url::Url;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

impl Ship {
    fn new() -> Self {
        Self {
            pos: screen_center(),
            rot: 0.,
            vel: Vec2::new(0., 0.),
        }
    }

    fn rotation(&self) -> f32 {
        self.rot.to_radians()
    }

    fn draw(&self) {
        let v1 = Vec2::new(
            self.pos.x + self.rotation().sin() * SHIP_HEIGHT / 2.,
            self.pos.y - self.rotation().cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.pos.x
                - self.rotation().cos() * SHIP_BASE / 2.
                - self.rotation().sin() * SHIP_HEIGHT / 2.,
            self.pos.y - self.rotation().sin() * SHIP_BASE / 2.
                + self.rotation().cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.pos.x + self.rotation().cos() * SHIP_BASE / 2.
                - self.rotation().sin() * SHIP_HEIGHT / 2.,
            self.pos.y
                + self.rotation().sin() * SHIP_BASE / 2.
                + self.rotation().cos() * SHIP_HEIGHT / 2.,
        );
        let v1_2 = Vec2::new(
            self.pos.x + self.rotation().sin() * SHIP_HEIGHT / 4.,
            self.pos.y - self.rotation().cos() * SHIP_HEIGHT / 4.,
        );
        let v2_2 = Vec2::new(
            self.pos.x
                - self.rotation().cos() * SHIP_BASE / 4.
                - self.rotation().sin() * SHIP_HEIGHT / 4.,
            self.pos.y - self.rotation().sin() * SHIP_BASE / 4.
                + self.rotation().cos() * SHIP_HEIGHT / 4.,
        );
        let v3_2 = Vec2::new(
            self.pos.x + self.rotation().cos() * SHIP_BASE / 4.
                - self.rotation().sin() * SHIP_HEIGHT / 4.,
            self.pos.y
                + self.rotation().sin() * SHIP_BASE / 4.
                + self.rotation().cos() * SHIP_HEIGHT / 4.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
        draw_triangle_lines(v1_2, v2_2, v3_2, 2., BLACK);
    }
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

impl Bullet {
    fn new(pos: Vec2, vel: Vec2, shot_at: f64, collided: bool) -> Self {
        Self {
            pos,
            vel,
            shot_at,
            collided,
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., BLACK);
    }
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

impl Asteroid {
    fn new() -> Self {
        Self {
            pos: screen_center()
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * screen_width().min(screen_height())
                    / 2.,
            vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: 8,
            collided: false,
        }
    }

    fn new_split(pos: Vec2, velx: f32, vely: f32, size: f32, sides: u8) -> Vec<Asteroid> {
        let mut new_asteroids = Vec::new();

        let asteriod1 = Self {
            pos,
            vel: Vec2::new(vely, -velx).normalize() * rand::gen_range(1., 3.),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: size * 0.8,
            sides: sides - 1,
            collided: false,
        };

        let asteriod2 = Self {
            pos,
            vel: Vec2::new(-vely, velx).normalize(),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: size * 0.8,
            sides: sides - 1,
            collided: false,
        };

        new_asteroids.push(asteriod1);
        new_asteroids.push(asteriod2);
        new_asteroids
    }

    fn draw(&self) {
        draw_poly_lines(
            self.pos.x, self.pos.y, self.sides, self.size, self.rot, 2., BLACK,
        )
    }
}

fn wrap_around(pos: &Vec2) -> Vec2 {
    let mut wrapped_pos = Vec2::new(pos.x, pos.y);
    if wrapped_pos.x > screen_width() {
        wrapped_pos.x = 0.;
    }
    if wrapped_pos.x < 0. {
        wrapped_pos.x = screen_width()
    }
    if wrapped_pos.y > screen_height() {
        wrapped_pos.y = 0.;
    }
    if wrapped_pos.y < 0. {
        wrapped_pos.y = screen_height()
    }
    return wrapped_pos;
}

fn screen_center() -> Vec2 {
    let screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
    screen_center
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

        let mut acc = -ship.vel / 30.0;
        if is_key_down(KeyCode::Up) {
            acc = Vec2::new(ship.rotation().sin(), -ship.rotation().cos()) / 3.;
        }

        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.1 {
            let rot_vec = Vec2::new(ship.rotation().sin(), -ship.rotation().cos());
            bullets.push(Bullet::new(
                ship.pos + rot_vec * SHIP_HEIGHT / 2.,
                rot_vec * 7.,
                frame_t,
                false,
            ));
            last_shot = frame_t;
        }
        if is_key_down(KeyCode::Right) {
            ship.rot += 5.;
        } else if is_key_down(KeyCode::Left) {
            ship.rot -= 5.;
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        ship.vel += acc;
        if ship.vel.length() > 10. {
            ship.vel = ship.vel.normalize() * 10.;
        }
        ship.pos += ship.vel;
        ship.pos = wrap_around(&ship.pos);
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }
        for asteroid in asteroids.iter_mut() {
            asteroid.pos += asteroid.vel;
            asteroid.pos = wrap_around(&asteroid.pos);
            asteroid.rot += asteroid.rot_speed;
        }

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            if (asteroid.pos - ship.pos).length() < asteroid.size + SHIP_HEIGHT / 3. {
                gameover = true;
                break;
            }
            for bullet in bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;
                    if asteroid.sides > 4 {
                        new_asteroids = Asteroid::new_split(
                            asteroid.pos,
                            bullet.vel.x,
                            bullet.vel.y,
                            asteroid.size,
                            asteroid.sides,
                        );
                        // new_asteroids.push(Asteroid::new(
                        //     asteroid.pos,
                        //     Vec2::new(bullet.vel.y, -bullet.vel.x).normalize()
                        //         * rand::gen_range(1., 3.),
                        //     rand::gen_range(0., 360.),
                        //     rand::gen_range(-2., 2.),
                        //     asteroid.size * 0.8,
                        //     asteroid.sides - 1,
                        //     false,
                        // ));
                        // new_asteroids.push(Asteroid::new(
                        //     asteroid.pos,
                        //     Vec2::new(-bullet.vel.y, bullet.vel.x).normalize()
                        //         * rand::gen_range(1., 3.),
                        //     rand::gen_range(0., 360.),
                        //     rand::gen_range(-2., 2.),
                        //     asteroid.size * 0.8,
                        //     asteroid.sides - 1,
                        //     false,
                        // ))
                    }
                    break;
                }
            }
        }

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
        asteroids.retain(|asteroid| !asteroid.collided);
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
