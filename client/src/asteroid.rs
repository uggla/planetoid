use crate::collision::Collided;
use crate::screen;
use macroquad::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

pub struct Asteroids {
    name: String,
    count: usize,
    asteroids: Vec<Asteroid>,
}

impl Asteroids {
    fn generate_field(name: String, number: usize) -> Self {
        let mut asteroids = Vec::new();
        for _item in 0..number {
            let asteroid = Asteroid::new();
            asteroids.push(asteroid);
        }

        Self {
            name,
            count: number,
            asteroids,
        }
    }
}

pub struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
}

impl Asteroid {
    pub fn new() -> Self {
        Self {
            pos: screen::center()
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * screen_width().min(screen_height())
                    / 2.,
            vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: 8,
            collided: false,
        }
    }

    pub fn new_split(pos: Vec2, velx: f32, vely: f32, size: f32, sides: u8) -> Vec<Asteroid> {
        let mut new_asteroids = Vec::new();

        let asteroid1 = Self {
            pos,
            vel: Vec2::new(vely, -velx).normalize() * rand::gen_range(1., 3.),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: size * 0.8,
            sides: sides - 1,
            collided: false,
        };

        let asteroid2 = Self {
            pos,
            vel: Vec2::new(-vely, velx).normalize(),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: size * 0.8,
            sides: sides - 1,
            collided: false,
        };

        new_asteroids.push(asteroid1);
        new_asteroids.push(asteroid2);
        new_asteroids
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
        self.pos = screen::wrap_around(&self.pos);
        self.rot += self.rot_speed;
    }

    pub fn draw(&self) {
        draw_poly_lines(
            self.pos.x, self.pos.y, self.sides, self.size, self.rot, 2., BLACK,
        )
    }

    pub fn sides(&self) -> u8 {
        self.sides
    }

    pub fn collided(&self) -> bool {
        self.collided
    }

    pub fn set_collided(&mut self, collided: bool) {
        self.collided = collided;
    }
}

impl Collided for Asteroid {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn size(&self) -> f32 {
        self.size
    }
}

impl Serialize for Asteroid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Asteroid", 7)?;
        state.serialize_field("pos", &vec![&self.pos[0], &self.pos[1]])?;
        state.serialize_field("vel", &vec![&self.vel[0], &self.vel[1]])?;
        state.serialize_field("rot", &self.rot)?;
        state.serialize_field("rot_speed", &self.rot_speed)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("sides", &self.sides)?;
        state.serialize_field("collided", &self.collided)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Asteroid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Pos,
            Vel,
            Rot,
            RotSpeed,
            Size,
            Sides,
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
                        formatter.write_str(
                            "`pos`, `vel`, `rot`, `rot_speed`, `size`, `sides` or `collided`",
                        )
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "pos" => Ok(Field::Pos),
                            "vel" => Ok(Field::Vel),
                            "rot" => Ok(Field::Rot),
                            "rot_speed" => Ok(Field::RotSpeed),
                            "size" => Ok(Field::Size),
                            "sides" => Ok(Field::Sides),
                            "collided" => Ok(Field::Collided),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AsteroidVisitor;

        impl<'de> Visitor<'de> for AsteroidVisitor {
            type Value = Asteroid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Asteroid")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Asteroid, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut pos: Option<Vec<f32>> = None;
                let mut vel: Option<Vec<f32>> = None;
                let mut rot = None;
                let mut rot_speed = None;
                let mut size = None;
                let mut sides = None;
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
                        Field::Rot => {
                            if rot.is_some() {
                                return Err(de::Error::duplicate_field("rot"));
                            }
                            rot = Some(map.next_value()?);
                        }
                        Field::RotSpeed => {
                            if rot_speed.is_some() {
                                return Err(de::Error::duplicate_field("rot_speed"));
                            }
                            rot_speed = Some(map.next_value()?);
                        }
                        Field::Size => {
                            if size.is_some() {
                                return Err(de::Error::duplicate_field("size"));
                            }
                            size = Some(map.next_value()?);
                        }
                        Field::Sides => {
                            if sides.is_some() {
                                return Err(de::Error::duplicate_field("sides"));
                            }
                            sides = Some(map.next_value()?);
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
                let rot = rot.ok_or_else(|| de::Error::missing_field("rot"))?;
                let rot_speed = rot_speed.ok_or_else(|| de::Error::missing_field("rot_speed"))?;
                let size = size.ok_or_else(|| de::Error::missing_field("size"))?;
                let sides = sides.ok_or_else(|| de::Error::missing_field("sides"))?;
                let collided = collided.ok_or_else(|| de::Error::missing_field("collided"))?;
                Ok(Asteroid {
                    pos: Vec2::new(pos[0], pos[1]),
                    vel: Vec2::new(vel[0], vel[1]),
                    rot,
                    rot_speed,
                    size,
                    sides,
                    collided,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "pos",
            "vel",
            "rot",
            "rot_speed",
            "size",
            "sides",
            "collided",
        ];
        deserializer.deserialize_struct("Asteroid", FIELDS, AsteroidVisitor)
    }
}

impl Clone for Asteroid {
    fn clone(&self) -> Self {
        Self {
            pos: self.pos,
            vel: self.vel,
            rot: self.rot,
            rot_speed: self.rot_speed,
            size: self.size,
            sides: self.sides,
            collided: self.collided,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    fn window_conf() -> Conf {
        Conf {
            window_title: String::from("Planetoid"),
            fullscreen: false,
            window_width: 1024,
            window_height: 768,
            window_resizable: false,
            ..Default::default()
        }
    }

    #[test]
    fn asteroid_serialize_deserialize_test() {
        let asteroid = Asteroid {
            pos: Vec2::new(1., 1.),
            vel: Vec2::new(2., 2.),
            rot: 1.,
            rot_speed: 1.,
            size: 1.,
            sides: 8,
            collided: false,
        };
        let serialize = serde_json::to_string(&asteroid).unwrap();
        dbg!(&serialize);
        let deserialize: Asteroid = serde_json::from_str(&serialize).unwrap();
        let serialize2 = serde_json::to_string(&deserialize).unwrap();
        dbg!(&serialize2);
        assert_eq!(serialize, serialize2);
        assert_eq!(asteroid.pos, deserialize.pos);
        assert_eq!(asteroid.vel, deserialize.vel);
        assert_eq!(asteroid.rot, deserialize.rot);
        assert_eq!(asteroid.rot_speed, deserialize.rot_speed);
        assert_eq!(asteroid.size, deserialize.size);
        assert_eq!(asteroid.sides, deserialize.sides);
        assert_eq!(asteroid.collided, deserialize.collided);
    }

    #[test]
    fn gen_rand_test() {
        // This is not a real test just a snippet to check how the quad-rand crate is working
        // If the random generator is feed with the same seed, it gives random numbers, but the
        // numbers are the same between 2 runs.
        // Using the UNIX_EPOCH as the seed avoid to use the same seed between runs.
        rand::srand(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        for _i in 0..10 {
            dbg!(rand::rand());
        }
        dbg!(rand::rand());
    }

    #[test]
    fn generate_field_test() {
        async fn amain() {
            let field = Asteroids::generate_field("planetoid".to_string(), 3);
            assert_eq!(field.name, "planetoid".to_string());
            assert_eq!(field.count, 3);
            assert_eq!(field.asteroids.len(), 3);
        }
        macroquad::Window::from_config(window_conf(), amain());
    }
}
