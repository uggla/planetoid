use crate::collision::Collision;
use crate::screen;
use macroquad::prelude::*;

pub struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
    acc: Vec2,
    size: f32,
}

impl Ship {
    pub const HEIGHT: f32 = 25.;
    pub const BASE: f32 = 22.;
    const DACC_FACTOR: f32 = 30.;
    const ACC_FACTOR: f32 = 3.;
    pub fn new() -> Self {
        Self {
            pos: screen::center(),
            rot: 0.,
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            size: Ship::HEIGHT / 3.,
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rot.to_radians()
    }

    pub fn draw(&self) {
        let v1 = Vec2::new(
            self.pos.x + self.rotation().sin() * Ship::HEIGHT / 2.,
            self.pos.y - self.rotation().cos() * Ship::HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.pos.x
                - self.rotation().cos() * Ship::BASE / 2.
                - self.rotation().sin() * Ship::HEIGHT / 2.,
            self.pos.y - self.rotation().sin() * Ship::BASE / 2.
                + self.rotation().cos() * Ship::HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.pos.x + self.rotation().cos() * Ship::BASE / 2.
                - self.rotation().sin() * Ship::HEIGHT / 2.,
            self.pos.y
                + self.rotation().sin() * Ship::BASE / 2.
                + self.rotation().cos() * Ship::HEIGHT / 2.,
        );
        let v1_2 = Vec2::new(
            self.pos.x + self.rotation().sin() * Ship::HEIGHT / 4.,
            self.pos.y - self.rotation().cos() * Ship::HEIGHT / 4.,
        );
        let v2_2 = Vec2::new(
            self.pos.x
                - self.rotation().cos() * Ship::BASE / 4.
                - self.rotation().sin() * Ship::HEIGHT / 4.,
            self.pos.y - self.rotation().sin() * Ship::BASE / 4.
                + self.rotation().cos() * Ship::HEIGHT / 4.,
        );
        let v3_2 = Vec2::new(
            self.pos.x + self.rotation().cos() * Ship::BASE / 4.
                - self.rotation().sin() * Ship::HEIGHT / 4.,
            self.pos.y
                + self.rotation().sin() * Ship::BASE / 4.
                + self.rotation().cos() * Ship::HEIGHT / 4.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
        draw_triangle_lines(v1_2, v2_2, v3_2, 2., BLACK);
    }

    pub fn slow_down(&mut self) {
        self.acc = -self.vel() / Ship::DACC_FACTOR;
    }

    pub fn accelerate(&mut self) {
        self.acc = Vec2::new(self.rotation().sin(), -self.rotation().cos()) / Ship::ACC_FACTOR;
    }

    pub fn update_pos(&mut self) {
        self.vel += self.acc;
        if self.vel.length() > 10. {
            self.vel = self.vel.normalize() * 10.;
        }
        self.pos += self.vel;
        self.pos = screen::wrap_around(&self.pos);
    }

    pub fn vel(&self) -> Vec2 {
        self.vel
    }

    pub fn rot(&self) -> f32 {
        self.rot
    }

    pub fn set_rot(&mut self, rot: f32) {
        self.rot = rot;
    }
}

impl Collision for Ship {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn size(&self) -> f32 {
        self.size
    }
}
