use macroquad::{audio, prelude::*};

pub struct Sound {
    laser: macroquad::audio::Sound,
    thrust: macroquad::audio::Sound,
    // Use a tuple (Sound, already_played)
    explosion: (macroquad::audio::Sound, bool),
    victory: (macroquad::audio::Sound, bool),
}

impl Sound {
    pub async fn new() -> Self {
        set_pc_assets_folder("sounds");
        Self {
            laser: audio::load_sound("laser.wav").await.unwrap(),
            thrust: audio::load_sound("thrust.wav").await.unwrap(),
            explosion: (audio::load_sound("explosion.wav").await.unwrap(), false),
            victory: (audio::load_sound("victory.wav").await.unwrap(), false),
        }
    }

    pub fn laser(&self) {
        audio::play_sound_once(self.laser);
    }

    pub fn thrust(&self) {
        audio::play_sound_once(self.thrust);
    }

    pub fn explosion(&mut self) {
        if !self.explosion.1 {
            audio::play_sound_once(self.explosion.0);
            self.explosion.1 = true;
        }
    }

    pub fn victory(&mut self) {
        if !self.victory.1 {
            audio::play_sound_once(self.victory.0);
            self.victory.1 = true;
        }
    }

    pub fn reset_played_sound(&mut self) {
        self.victory.1 = false;
        self.explosion.1 = false;
    }
}
