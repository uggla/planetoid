use macroquad::prelude::*;
pub fn wrap_around(pos: &Vec2) -> Vec2 {
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
    wrapped_pos
}

pub fn center() -> Vec2 {
    Vec2::new(screen_width() / 2., screen_height() / 2.)
}
