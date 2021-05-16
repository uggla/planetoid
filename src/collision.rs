use macroquad::prelude::*;

pub trait Collided {
    fn size(&self) -> f32;
    fn pos(&self) -> Vec2;
}

pub fn is_collided<A: Collided, B: Collided>(obj1: &A, obj2: &B) -> bool {
    (obj1.pos() - obj2.pos()).length() < obj1.size() + obj2.size()
}
