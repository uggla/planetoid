use crate::screen;
use crate::{bullet::Bullet, collision::Collided};
use macroquad::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Ship {
    name: String,
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    rot: f32,
    size: f32,
    collided: bool,
    pub bullets: Vec<Bullet>,
}

impl Ship {
    pub const HEIGHT: f32 = 25.;
    pub const BASE: f32 = 22.;
    const DACC_FACTOR: f32 = 30.;
    const ACC_FACTOR: f32 = 3.;
    pub fn new(name: String) -> Self {
        Self {
            name,
            pos: screen::center(),
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            rot: 0.,
            size: Ship::HEIGHT / 3.,
            collided: false,
            bullets: Vec::new(),
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rot.to_radians()
    }

    pub fn draw(&self, color: Color) {
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
        draw_triangle_lines(v1, v2, v3, 2., color);
        draw_triangle_lines(v1_2, v2_2, v3_2, 2., color);
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

    pub fn name(&self) -> String {
        self.name.clone()
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

    pub fn shoot(&mut self, frame_t: f64) {
        let rot_vec = Vec2::new(self.rotation().sin(), -self.rotation().cos());
        self.bullets.push(Bullet::new(
            self.pos() + rot_vec * Ship::HEIGHT / 2.,
            rot_vec * 7.,
            frame_t,
            false,
        ));
    }
}

impl Collided for Ship {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn size(&self) -> f32 {
        self.size
    }
}

impl Serialize for Ship {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Ship", 8)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("pos", &vec![&self.pos[0], &self.pos[1]])?;
        state.serialize_field("vel", &vec![&self.vel[0], &self.vel[1]])?;
        state.serialize_field("acc", &vec![&self.acc[0], &self.acc[1]])?;
        state.serialize_field("rot", &self.rot)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("collided", &self.collided)?;
        state.serialize_field("bullets", &self.bullets)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Ship {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Name,
            Pos,
            Vel,
            Acc,
            Rot,
            Size,
            Collided,
            Bullets,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(
                            "`name`, `pos`, `vel`, `acc`, `rot`, `size`, `collided` or `bullets`",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "pos" => Ok(Field::Pos),
                            "vel" => Ok(Field::Vel),
                            "acc" => Ok(Field::Acc),
                            "rot" => Ok(Field::Rot),
                            "size" => Ok(Field::Size),
                            "collided" => Ok(Field::Collided),
                            "bullets" => Ok(Field::Bullets),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ShipVisitor;

        impl<'de> Visitor<'de> for ShipVisitor {
            type Value = Ship;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Ship")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Ship, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut pos: Option<Vec<f32>> = None;
                let mut vel: Option<Vec<f32>> = None;
                let mut acc: Option<Vec<f32>> = None;
                let mut rot = None;
                let mut size = None;
                let mut collided = None;
                let mut bullets: Option<Vec<Bullet>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Pos => {
                            if pos.is_some() {
                                return Err(de::Error::duplicate_field("pos"));
                            }
                            pos = Some(map.next_value()?);
                        }
                        Field::Vel => {
                            if vel.is_some() {
                                return Err(de::Error::duplicate_field("vel"));
                            }
                            vel = Some(map.next_value()?);
                        }
                        Field::Acc => {
                            if acc.is_some() {
                                return Err(de::Error::duplicate_field("vel"));
                            }
                            acc = Some(map.next_value()?);
                        }
                        Field::Rot => {
                            if rot.is_some() {
                                return Err(de::Error::duplicate_field("rot"));
                            }
                            rot = Some(map.next_value()?);
                        }
                        Field::Size => {
                            if size.is_some() {
                                return Err(de::Error::duplicate_field("size"));
                            }
                            size = Some(map.next_value()?);
                        }
                        Field::Collided => {
                            if collided.is_some() {
                                return Err(de::Error::duplicate_field("collided"));
                            }
                            collided = Some(map.next_value()?);
                        }
                        Field::Bullets => {
                            if bullets.is_some() {
                                return Err(de::Error::duplicate_field("bullets"));
                            }
                            bullets = Some(map.next_value()?);
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let pos = pos.ok_or_else(|| de::Error::missing_field("pos"))?;
                let vel = vel.ok_or_else(|| de::Error::missing_field("vel"))?;
                let acc = acc.ok_or_else(|| de::Error::missing_field("acc"))?;
                let rot = rot.ok_or_else(|| de::Error::missing_field("rot"))?;
                let size = size.ok_or_else(|| de::Error::missing_field("size"))?;
                let collided = collided.ok_or_else(|| de::Error::missing_field("collided"))?;
                let bullets = bullets.ok_or_else(|| de::Error::missing_field("bullets"))?;
                Ok(Ship {
                    name,
                    pos: Vec2::new(pos[0], pos[1]),
                    vel: Vec2::new(vel[0], vel[1]),
                    acc: Vec2::new(acc[0], acc[1]),
                    rot,
                    size,
                    collided,
                    bullets,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "name", "pos", "vel", "acc", "rot", "size", "collided", "bullets",
        ];
        deserializer.deserialize_struct("Ship", FIELDS, ShipVisitor)
    }
}

impl Clone for Ship {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            pos: self.pos,
            vel: self.vel,
            acc: self.acc,
            rot: self.rot,
            size: self.size,
            collided: self.collided,
            bullets: self.bullets.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_serialize_deserialize_test() {
        let mut bullets: Vec<Bullet> = Vec::new();

        let bullet = Bullet::new(Vec2::new(1., 1.), Vec2::new(2., 2.), 5., false);
        bullets.push(bullet);

        let bullet2 = Bullet::new(Vec2::new(2., 1.), Vec2::new(3., 2.), 6., true);
        bullets.push(bullet2);

        let ship = Ship {
            name: String::from("Uggla"),
            pos: Vec2::new(1., 1.),
            vel: Vec2::new(2., 2.),
            acc: Vec2::new(3., 3.),
            rot: 1.,
            size: 1.,
            collided: false,
            bullets,
        };
        let serialize = serde_json::to_string(&ship).unwrap();
        dbg!(&serialize);
        let deserialize: Ship = serde_json::from_str(&serialize).unwrap();
        let serialize2 = serde_json::to_string(&deserialize).unwrap();
        assert_eq!(serialize, serialize2);
        assert_eq!(ship.name, deserialize.name);
        assert_eq!(ship.pos, deserialize.pos);
        assert_eq!(ship.vel, deserialize.vel);
        assert_eq!(ship.acc, deserialize.acc);
        assert_eq!(ship.rot, deserialize.rot);
        assert_eq!(ship.size, deserialize.size);
        assert_eq!(ship.collided, deserialize.collided);
        assert_eq!(ship.bullets[0].pos(), deserialize.bullets[0].pos());
        assert_eq!(ship.bullets[0].vel(), deserialize.bullets[0].vel());
        assert_eq!(ship.bullets[0].shot_at(), deserialize.bullets[0].shot_at());
        assert_eq!(ship.bullets[0].size(), deserialize.bullets[0].size());
        assert_eq!(
            ship.bullets[0].collided(),
            deserialize.bullets[0].collided()
        );
        assert_eq!(ship.bullets[1].pos(), deserialize.bullets[1].pos());
        assert_eq!(ship.bullets[1].vel(), deserialize.bullets[1].vel());
        assert_eq!(ship.bullets[1].shot_at(), deserialize.bullets[1].shot_at());
        assert_eq!(ship.bullets[1].size(), deserialize.bullets[1].size());
        assert_eq!(
            ship.bullets[1].collided(),
            deserialize.bullets[1].collided()
        );
    }

    #[test]
    fn ship_clone_test() {
        let mut bullets: Vec<Bullet> = Vec::new();

        let bullet = Bullet::new(Vec2::new(1., 1.), Vec2::new(2., 2.), 5., false);
        bullets.push(bullet);

        let ship = Ship {
            name: String::from("Uggla"),
            pos: Vec2::new(1., 1.),
            vel: Vec2::new(2., 2.),
            acc: Vec2::new(3., 3.),
            rot: 1.,
            size: 1.,
            collided: false,
            bullets,
        };

        let ship_clone = ship.clone();

        assert_eq!(ship.name, ship_clone.name);
        assert_eq!(ship.pos, ship_clone.pos);
        assert_eq!(ship.vel, ship_clone.vel);
        assert_eq!(ship.acc, ship_clone.acc);
        assert_eq!(ship.rot, ship_clone.rot);
        assert_eq!(ship.size, ship_clone.size);
        assert_eq!(ship.collided, ship_clone.collided);
        assert_eq!(ship.bullets[0].pos(), ship_clone.bullets[0].pos());
        assert_eq!(ship.bullets[0].vel(), ship_clone.bullets[0].vel());
        assert_eq!(ship.bullets[0].shot_at(), ship_clone.bullets[0].shot_at());
        assert_eq!(ship.bullets[0].size(), ship_clone.bullets[0].size());
    }
}
