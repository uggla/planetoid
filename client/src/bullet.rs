use crate::collision::Collided;
use macroquad::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    size: f32,
    collided: bool,
}

impl Bullet {
    pub fn new(pos: Vec2, vel: Vec2, shot_at: f64, collided: bool) -> Self {
        Self {
            pos,
            vel,
            shot_at,
            collided,
            size: 2.,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2., BLACK);
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
    }

    pub fn shot_at(&self) -> f64 {
        self.shot_at
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.collided = collided;
    }

    pub fn collided(&self) -> bool {
        self.collided
    }

    pub fn vel(&self) -> Vec2 {
        self.vel
    }
}

impl Collided for Bullet {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn size(&self) -> f32 {
        self.size
    }
}

impl Serialize for Bullet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Bullet", 5)?;
        state.serialize_field("pos", &vec![&self.pos[0], &self.pos[1]])?;
        state.serialize_field("vel", &vec![&self.vel[0], &self.vel[1]])?;
        state.serialize_field("shot_at", &self.shot_at)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("collided", &self.collided)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Bullet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Pos,
            Vel,
            ShotAt,
            Size,
            Collided,
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
                        formatter.write_str("`pos`, `vel`, `shot_at`, `size` or `collided`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "pos" => Ok(Field::Pos),
                            "vel" => Ok(Field::Vel),
                            "shot_at" => Ok(Field::ShotAt),
                            "size" => Ok(Field::Size),
                            "collided" => Ok(Field::Collided),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct BulletVisitor;

        impl<'de> Visitor<'de> for BulletVisitor {
            type Value = Bullet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Bullet")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Bullet, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut pos: Option<Vec<f32>> = None;
                let mut vel: Option<Vec<f32>> = None;
                let mut shot_at = None;
                let mut size = None;
                let mut collided = None;
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
                        Field::ShotAt => {
                            if shot_at.is_some() {
                                return Err(de::Error::duplicate_field("shot_at"));
                            }
                            shot_at = Some(map.next_value()?);
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
                    }
                }
                let pos = pos.ok_or_else(|| de::Error::missing_field("pos"))?;
                let vel = vel.ok_or_else(|| de::Error::missing_field("vel"))?;
                let shot_at = shot_at.ok_or_else(|| de::Error::missing_field("shot_at"))?;
                let size = size.ok_or_else(|| de::Error::missing_field("size"))?;
                let collided = collided.ok_or_else(|| de::Error::missing_field("collided"))?;
                Ok(Bullet {
                    pos: Vec2::new(pos[0], pos[1]),
                    vel: Vec2::new(vel[0], vel[1]),
                    shot_at,
                    size,
                    collided,
                })
            }
        }

        const FIELDS: &[&str] = &["pos", "vel", "shot_at", "size", "collided"];
        deserializer.deserialize_struct("Bullet", FIELDS, BulletVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bullet_serialize_deserialize_test() {
        let bullet = Bullet {
            pos: Vec2::new(1., 1.),
            vel: Vec2::new(2., 2.),
            shot_at: 1.,
            size: 1.,
            collided: false,
        };
        let serialize = serde_json::to_string(&bullet).unwrap();
        dbg!(&serialize);
        let deserialize: Bullet = serde_json::from_str(&serialize).unwrap();
        let serialize2 = serde_json::to_string(&deserialize).unwrap();
        dbg!(&serialize2);
        assert_eq!(serialize, serialize2);
        assert_eq!(bullet.pos, deserialize.pos);
        assert_eq!(bullet.vel, deserialize.vel);
        assert_eq!(bullet.shot_at, deserialize.shot_at);
        assert_eq!(bullet.size, deserialize.size);
        assert_eq!(bullet.collided, deserialize.collided);
    }
}
