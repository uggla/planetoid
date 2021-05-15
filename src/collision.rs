use macroquad::prelude::*;

pub trait Collision {
    fn size(&self) -> f32;
    fn pos(&self) -> Vec2;
}

pub fn is_collided<A: Collision, B: Collision>(obj1: &A, obj2: &B) -> bool {
    (obj1.pos() - obj2.pos()).length() < obj1.size() + obj2.size()
}
