use crate::MAX_ASTEROIDS;
use macroquad::prelude::*;

use crate::asteroid::Asteroids;
use crate::ship::Ship;

pub fn manage_gameover(
    players: &mut Vec<Ship>,
    asteroids: &mut Asteroids,
    mode: &str,
    name: &str,
    frame_count: &mut u32,
    gameover: &mut bool,
    gameover_msg_sent: &mut bool,
) {
    // Take care this part is executed in a loop !
    // host is looping until the enter key is pressed
    clear_background(LIGHTGRAY);
    let mut status = "You Win!.";
    let text: String;
    let font_size = 30.;

    if !asteroids.is_empty() {
        status = "Game Over.";
    }

    if mode == "host" {
        text = format!("{} Press [enter] to play again.", status);
    } else {
        text = format!("{} Wait host player to restart game.", status);
    }
    let text_size = measure_text(&text, None, font_size as _, 1.0);
    draw_text(
        &text,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height / 2.,
        font_size,
        DARKGRAY,
    );

    if mode != "host" || is_key_down(KeyCode::Enter) {
        log::info!("Restarting game.");
        players.clear();
        players.push(Ship::new(String::from(name)));
        *gameover = false;
        *gameover_msg_sent = false;
        *asteroids = Asteroids::generate_field(String::from(name), 0);
        if mode == "host" {
            *asteroids = Asteroids::generate_field(String::from(name), MAX_ASTEROIDS);
        }
        *frame_count = 0;
    }
}
