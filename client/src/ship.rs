use crate::collision::Collided;
use crate::screen;
use macroquad::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Ship {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    rot: f32,
    size: f32,
}

impl Ship {
    pub const HEIGHT: f32 = 25.;
    pub const BASE: f32 = 22.;
    const DACC_FACTOR: f32 = 30.;
    const ACC_FACTOR: f32 = 3.;
    pub fn new() -> Self {
        Self {
            pos: screen::center(),
            rot: 0.,
            vel: Vec2::new(0., 0.),
            acc: Vec2::new(0., 0.),
            size: Ship::HEIGHT / 3.,
        }
    }

    pub fn rotation(&self) -> f32 {
        self.rot.to_radians()
    }

    pub fn draw(&self) {
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
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
        draw_triangle_lines(v1_2, v2_2, v3_2, 2., BLACK);
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

    pub fn vel(&self) -> Vec2 {
        self.vel
    }

    pub fn rot(&self) -> f32 {
        self.rot
    }

    pub fn set_rot(&mut self, rot: f32) {
        self.rot = rot;
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
        let mut state = serializer.serialize_struct("Ship", 5)?;
        state.serialize_field("pos", &vec![&self.pos[0], &self.pos[1]])?;
        state.serialize_field("vel", &vec![&self.vel[0], &self.vel[1]])?;
        state.serialize_field("acc", &vec![&self.acc[0], &self.acc[1]])?;
        state.serialize_field("rot", &self.rot)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Ship {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Pos,
            Vel,
            Acc,
            Rot,
            Size,
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
                        formatter.write_str("`pos`, `vel`, `acc`, `rot` or `size`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "pos" => Ok(Field::Pos),
                            "vel" => Ok(Field::Vel),
                            "acc" => Ok(Field::Acc),
                            "rot" => Ok(Field::Rot),
                            "size" => Ok(Field::Size),
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
                let mut pos: Option<Vec<f32>> = None;
                let mut vel: Option<Vec<f32>> = None;
                let mut acc: Option<Vec<f32>> = None;
                let mut rot = None;
                let mut size = None;
                while let Some(key) = map.next_key()? {
                    match key {
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
                    }
                }
                let pos = pos.ok_or_else(|| de::Error::missing_field("pos"))?;
                let vel = vel.ok_or_else(|| de::Error::missing_field("vel"))?;
                let acc = acc.ok_or_else(|| de::Error::missing_field("acc"))?;
                let rot = rot.ok_or_else(|| de::Error::missing_field("rot"))?;
                let size = size.ok_or_else(|| de::Error::missing_field("size"))?;
                Ok(Ship {
                    pos: Vec2::new(pos[0], pos[1]),
                    vel: Vec2::new(vel[0], vel[1]),
                    acc: Vec2::new(acc[0], acc[1]),
                    rot,
                    size,
                })
            }
        }

        const FIELDS: &[&str] = &["pos", "vel", "acc", "rot", "size"];
        deserializer.deserialize_struct("Ship", FIELDS, ShipVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_serialize_deserialize_test() {
        let ship = Ship {
            pos: Vec2::new(1., 1.),
            vel: Vec2::new(2., 2.),
            acc: Vec2::new(3., 3.),
            rot: 1.,
            size: 1.,
        };
        let serialize = serde_json::to_string(&ship).unwrap();
        dbg!(&serialize);
        let deserialize: Ship = serde_json::from_str(&serialize).unwrap();
        let serialize2 = serde_json::to_string(&deserialize).unwrap();
        dbg!(&serialize2);
        assert_eq!(serialize, serialize2);
        assert_eq!(ship.pos, deserialize.pos);
        assert_eq!(ship.vel, deserialize.vel);
        assert_eq!(ship.acc, deserialize.acc);
        assert_eq!(ship.rot, deserialize.rot);
        assert_eq!(ship.size, deserialize.size);
    }
}
