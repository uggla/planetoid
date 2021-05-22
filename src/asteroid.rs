use crate::collision::Collided;
use crate::screen;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsteroidSerde {
    pos: (f32, f32),
    vel: (f32, f32),
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AsteroidsSerde {
    pub asteroids: Vec<AsteroidSerde>,
}

pub struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

impl Asteroid {
    pub fn new() -> Self {
        Self {
            pos: screen::center()
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

    pub fn new_split(pos: Vec2, velx: f32, vely: f32, size: f32, sides: u8) -> Vec<Asteroid> {
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

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
        self.pos = screen::wrap_around(&self.pos);
        self.rot += self.rot_speed;
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.pos.x, self.pos.y, self.sides, self.size, self.rot, 2., BLACK,
        )
    }

    pub fn sides(&self) -> u8 {
        self.sides
    }

    pub fn collided(&self) -> bool {
        self.collided
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.collided = collided;
    }

    pub fn to_serde(&self) -> AsteroidSerde {
        let pos_t: (f32, f32) = self.pos.into();
        let vel_t: (f32, f32) = self.vel.into();
        let asteroid = AsteroidSerde {
            pos: pos_t,
            vel: vel_t,
            rot: self.rot,
            rot_speed: self.rot_speed,
            size: self.size,
            sides: self.sides,
            collided: self.collided,
        };
        asteroid
    }

    pub fn from_serde(asteroid: &AsteroidSerde) -> Self {
        Self {
            pos: Vec2::from(asteroid.pos),
            vel: Vec2::from(asteroid.vel),
            rot: asteroid.rot,
            rot_speed: asteroid.rot_speed,
            size: asteroid.size,
            sides: asteroid.sides,
            collided: asteroid.collided,
        }
    }
}

impl Collided for Asteroid {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn size(&self) -> f32 {
        self.size
    }
}
