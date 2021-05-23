use crate::collision::Collided;
use macroquad::prelude::*;

pub struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
    size: f32,
}

impl Bullet {
    pub fn new(pos: Vec2, vel: Vec2, shot_at: f64, collided: bool) -> Self {
        Self {
            pos,
            vel,
            shot_at,
            collided,
            size: 2.,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., BLACK);
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
    }

    pub fn shot_at(&self) -> f64 {
        self.shot_at
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.collided = collided;
    }

    pub fn collided(&self) -> bool {
        self.collided
    }

    pub fn vel(&self) -> Vec2 {
        self.vel
    }
}

impl Collided for Bullet {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn size(&self) -> f32 {
        self.size
    }
}
