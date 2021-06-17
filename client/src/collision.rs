use macroquad::prelude::*;

use crate::{asteroid::Asteroid, ship::Ship};

pub trait Collided {
    fn size(&self) -> f32;
    fn pos(&self) -> Vec2;
}

pub fn is_collided<A: Collided, B: Collided>(obj1: &A, obj2: &B) -> bool {
    (obj1.pos() - obj2.pos()).length() < obj1.size() + obj2.size()
}

pub fn manage_collisions(
    players: &mut Vec<Ship>,
    asteroids: &mut Vec<Asteroid>,
    god: bool,
    mode: &str,
    frame_t: f64,
) {
    let mut opponents = players.clone();
    for ship in players.iter_mut() {
        let mut new_asteroids = ship_vs_asteroids(ship, asteroids, god, mode);
        ship_vs_opponents(ship, &mut opponents);

        // if mode == "host" {
        ship.bullets
            // .retain(|bullet| bullet.shot_at() + 1.5 > frame_t && !bullet.collided());
            .retain(|bullet| bullet.shot_at() + 1.5 > frame_t);
        asteroids.retain(|asteroid| !asteroid.collided());
        // }
        asteroids.append(&mut new_asteroids);
    }

    for ship_index in 0..players.len() {
        if opponents[ship_index].collided() {
            players[ship_index].set_collided(true);
        }
    }

    players.retain(|ship| !ship.collided());
}

fn ship_vs_asteroids(
    ship: &mut Ship,
    asteroids: &mut Vec<Asteroid>,
    god: bool,
    mode: &str,
) -> Vec<Asteroid> {
    let mut new_asteroids = Vec::new();
    for asteroid in asteroids.iter_mut() {
        if is_collided(asteroid, ship) && !god && mode != "spectator" {
            ship.set_collided(true);
        }
        ship_bullet_vs_asteroid(ship, asteroid, &mut new_asteroids);
    }
    new_asteroids
}

fn ship_bullet_vs_asteroid(
    ship: &mut Ship,
    asteroid: &mut Asteroid,
    new_asteroids: &mut Vec<Asteroid>,
) {
    for bullet in ship.bullets.iter_mut() {
        if !bullet.collided() && !asteroid.collided() && is_collided(asteroid, bullet) {
            asteroid.set_collided(true);
            bullet.set_collided(true);
            if asteroid.sides() > 4 {
                *new_asteroids = Asteroid::new_split(
                    asteroid.pos(),
                    bullet.vel().x,
                    bullet.vel().y,
                    asteroid.size(),
                    asteroid.sides(),
                );
            }
            break;
        }
    }
}

fn ship_vs_opponents(ship: &mut Ship, opponents: &mut Vec<Ship>) {
    for opponent in opponents.iter_mut() {
        if opponent.name() != ship.name() {
            // for bullet in ship.bullets.iter_mut() {
            //     if is_collided(opponent, bullet) {
            //         bullet.set_collided(true);
            //         opponent.set_collided(true);
            //     }
            // }
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
