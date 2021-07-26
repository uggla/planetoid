use macroquad::prelude::*;

use crate::{
    asteroid::{Asteroid, Asteroids},
    ship::Ship,
};

pub trait Collided {
    fn size(&self) -> f32;
    fn pos(&self) -> Vec2;
}

pub fn is_collided<A: Collided, B: Collided>(obj1: &A, obj2: &B) -> bool {
    (obj1.pos() - obj2.pos()).length() < obj1.size() + obj2.size()
}

pub fn manage_collisions(
    players: &mut Vec<Ship>,
    asteroids: &mut Asteroids,
    name: String,
    god: bool,
    mode: &str,
    frame_t: f64,
    sync_t: f64,
) {
    let mut opponents = players.clone();
    for ship in players.iter_mut() {
        ship_vs_asteroids(ship, asteroids, name.clone(), god, mode, sync_t);
        ship_vs_opponents(ship, &mut opponents);

        // Garbage collect bullets every 1.5s (bullets can almost cross the screen).
        // This needs to be done only on the local ship as frame_t make sens
        // only for the local data
        if ship.name() == name {
            ship.bullets
                .retain(|bullet| bullet.shot_at() + 1.5 > frame_t);
        }

        // Garbage collect asteroids collided every 200ms.
        // This is mandatory to keep the messages small and limit the bandwidth.
        asteroids.get_asteroids().retain(|_key, value| {
            (value.last_updated() + 0.2) > (get_time() - sync_t) || !value.collided()
        });
    }

    for ship_index in 0..players.len() {
        if opponents[ship_index].collided() {
            players[ship_index].set_collided(true);
        }
    }
}

fn ship_vs_asteroids(
    ship: &mut Ship,
    asteroids: &mut Asteroids,
    name: String,
    god: bool,
    mode: &str,
    sync_t: f64,
) {
    let mut new_asteroids = Vec::new();
    for asteroid in asteroids.get_asteroids().values_mut() {
        if is_collided(asteroid, ship) && !god && mode != "spectator" {
            ship.set_collided(true);
        }
        ship_bullet_vs_asteroid(ship, asteroid, &mut new_asteroids, sync_t);
    }

    // Send new asteroids created only for this player.
    if ship.name() == name {
        for asteroid in new_asteroids {
            asteroids.add_asteroid(name.clone(), asteroid);
        }
    }
}

fn ship_bullet_vs_asteroid(
    ship: &mut Ship,
    asteroid: &mut Asteroid,
    new_asteroids: &mut Vec<Asteroid>,
    sync_t: f64,
) {
    for bullet in ship.bullets.iter_mut() {
        if !bullet.collided() && !asteroid.collided() && is_collided(asteroid, bullet) {
            asteroid.set_collided(true);
            asteroid.set_last_updated(get_time() - sync_t);
            bullet.set_collided(true);
            // Split asteroid into 2 smaller parts except if we have a square.
            if asteroid.sides() > 4 {
                *new_asteroids = Asteroid::new_split(
                    asteroid.pos(),
                    bullet.vel().x,
                    bullet.vel().y,
                    asteroid.size(),
                    asteroid.sides(),
                    asteroid.last_updated(),
                );
            }
            break;
        }
    }
}

fn ship_vs_opponents(ship: &mut Ship, opponents: &mut Vec<Ship>) {
    for opponent in opponents.iter_mut() {
        if opponent.name() != ship.name() {
            ship_bullet_vs_opponents(ship, opponent);
        }
    }
}

fn ship_bullet_vs_opponents(ship: &mut Ship, opponent: &mut Ship) {
    for bullet in ship.bullets.iter_mut() {
        if is_collided(opponent, bullet) {
            bullet.set_collided(true);
            opponent.set_collided(true);
        }
    }
}
