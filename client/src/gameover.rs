use crate::MAX_ASTEROIDS;
use macroquad::prelude::*;

use crate::{asteroid::Asteroid, ship::Ship};

pub fn manage_gameover(
    players: &mut Vec<Ship>,
    asteroids: &mut Vec<Asteroid>,
    mode: &str,
    name: &str,
    frame_count: &mut u32,
    gameover: &mut bool,
) {
    clear_background(LIGHTGRAY);
    let mut text = "You Win!. Press [enter] to play again.";
    let font_size = 30.;

    if !asteroids.is_empty() {
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
    if mode != "spectator" && is_key_down(KeyCode::Enter) {
        log::info!("Restarting game.");
        players.clear();
        players.push(Ship::new(String::from(name)));
        *asteroids = Vec::new();
        *gameover = false;
        for _ in 0..MAX_ASTEROIDS {
            asteroids.push(Asteroid::new());
        }
        *frame_count = 0;
    }
}
