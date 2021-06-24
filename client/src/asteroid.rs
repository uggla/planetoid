use crate::collision::Collided;
use crate::screen;
use macroquad::prelude::*;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Asteroids {
    count: u8,
    asteroids: BTreeMap<String, Asteroid>,
}

impl Asteroids {
    pub fn generate_field(name: String, number: u8) -> Self {
        let mut asteroids = BTreeMap::new();
        for item in 0..number {
            let asteroid = Asteroid::new();
            asteroids.insert(format!("{}_{:06}", name, item), asteroid);
        }

        Self {
            count: number,
            asteroids,
        }
    }

    pub fn add_asteroid(&mut self, name: String, asteroid: Asteroid) {
        // dbg!(&name);
        // dbg!(&asteroid);
        self.asteroids
            .insert(format!("{}_{:06}", name, self.count), asteroid);
        self.count += 1;
    }

    pub fn refresh_last_updated(&mut self, last_updated: f64) {
        for asteroid in self.asteroids.values_mut() {
            asteroid.set_last_updated(last_updated);
        }
    }

    pub fn get_asteroids(&mut self) -> &mut BTreeMap<String, Asteroid> {
        &mut self.asteroids
    }

    pub fn is_empty(&self) -> bool {
        self.asteroids.is_empty()
    }
}

pub fn synchronize_asteroids(
    field1: &mut Asteroids,
    field2: Asteroids,
    name_field2: String,
    // ) -> &mut Asteroids {
) {
    for (key_field2, value_field2) in &field2.asteroids {
        match field1.asteroids.get(key_field2) {
            None => {
                // field1.add_asteroid(key_field2.clone(), value_field2.clone());
                field1
                    .asteroids
                    .insert(key_field2.clone(), value_field2.clone());
                field1.count += 1;
                // dbg!("here");
                dbg!(&key_field2);
            }

            Some(value_field1) => {
                if value_field2.last_updated() > value_field1.last_updated() {
                    // field1
                    //     .asteroids
                    //     .get_mut(key_field2)
                    //     .unwrap()
                    //     .set_last_updated(value_field2.last_updated());
                    *field1.asteroids.get_mut(key_field2).unwrap() = value_field2.clone();
                }
            }
        }
    }
    // field1
}

#[derive(Debug)]
pub struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
    last_updated: f64,
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
            last_updated: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn new_pos_and_size(x: f32, y: f32, size: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size,
            sides: 8,
            collided: false,
            last_updated: 0.,
        }
    }

    pub fn new_split(
        pos: Vec2,
        velx: f32,
        vely: f32,
        size: f32,
        sides: u8,
        last_updated: f64,
    ) -> Vec<Asteroid> {
        let mut new_asteroids = Vec::new();

        let asteroid1 = Self {
            pos,
            vel: Vec2::new(vely, -velx).normalize() * rand::gen_range(1., 3.),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: size * 0.8,
            sides: sides - 1,
            collided: false,
            last_updated,
        };

        let asteroid2 = Self {
            pos,
            vel: Vec2::new(-vely, velx).normalize(),
            rot: rand::gen_range(0., 360.),
            rot_speed: rand::gen_range(-2., 2.),
            size: size * 0.8,
            sides: sides - 1,
            collided: false,
            last_updated,
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

    pub fn last_updated(&self) -> f64 {
        self.last_updated
    }

    pub fn set_last_updated(&mut self, last_updated: f64) {
        self.last_updated = last_updated;
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
        let mut state = serializer.serialize_struct("Asteroid", 8)?;
        state.serialize_field("pos", &vec![&self.pos[0], &self.pos[1]])?;
        state.serialize_field("vel", &vec![&self.vel[0], &self.vel[1]])?;
        state.serialize_field("rot", &self.rot)?;
        state.serialize_field("rot_speed", &self.rot_speed)?;
        state.serialize_field("size", &self.size)?;
        state.serialize_field("sides", &self.sides)?;
        state.serialize_field("collided", &self.collided)?;
        state.serialize_field("last_updated", &self.last_updated)?;
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
            LastUpdated,
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
                            "`pos`, `vel`, `rot`, `rot_speed`, `size`, `sides`, `collided` or `last_updated`",
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
                            "last_updated" => Ok(Field::LastUpdated),
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
                let mut last_updated = None;
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
                        Field::LastUpdated => {
                            if last_updated.is_some() {
                                return Err(de::Error::duplicate_field("last_updated"));
                            }
                            last_updated = Some(map.next_value()?);
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
                let last_updated =
                    last_updated.ok_or_else(|| de::Error::missing_field("last_updated"))?;
                Ok(Asteroid {
                    pos: Vec2::new(pos[0], pos[1]),
                    vel: Vec2::new(vel[0], vel[1]),
                    rot,
                    rot_speed,
                    size,
                    sides,
                    collided,
                    last_updated,
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
            "last_updated",
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
            last_updated: self.last_updated,
        }
    }
}

impl PartialOrd for Asteroid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.last_updated.partial_cmp(&other.last_updated)
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.last_updated == other.last_updated
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
            last_updated: 0.,
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
        assert_eq!(asteroid.last_updated, deserialize.last_updated);
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
    fn compare_asteroid_test() {
        let mut asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        asteroid1.set_last_updated(0.0);
        asteroid2.set_last_updated(2.0);
        assert!(asteroid1 < asteroid2);
        asteroid1.set_last_updated(3.0);
        asteroid2.set_last_updated(2.0);
        assert!(asteroid1 > asteroid2);
        asteroid1.set_last_updated(3.0);
        asteroid2.set_last_updated(3.0);
        assert!(asteroid1 == asteroid2);
    }

    /// Asteroids are the same
    #[test]
    fn asteroid_synchronize_1_test() {
        let mut asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        asteroid1.set_last_updated(0.0);
        asteroid2.set_last_updated(2.0);

        let asteroids = BTreeMap::new();
        let asteroids_c = asteroids.clone();

        let mut field1 = Asteroids {
            count: 0,
            asteroids,
        };

        let mut field2 = Asteroids {
            count: 0,
            asteroids: asteroids_c,
        };

        field1.add_asteroid("f1".to_string(), asteroid1.clone());
        field1.add_asteroid("f1".to_string(), asteroid2.clone());
        field2.add_asteroid("f1".to_string(), asteroid1.clone());
        field2.add_asteroid("f1".to_string(), asteroid2.clone());

        // let asteroids = synchronize_asteroids(&mut field1, field2, "f2".to_string());
        synchronize_asteroids(&mut field1, field2, "f2".to_string());
        // assert!(asteroids.asteroids.get("f1_000000").unwrap() == &asteroid1);
        // assert!(asteroids.asteroids.get("f1_000001").unwrap() == &asteroid2);
        assert!(field1.asteroids.get("f1_000000").unwrap() == &asteroid1);
        assert!(field1.asteroids.get("f1_000001").unwrap() == &asteroid2);
    }

    /// New asteroid in field2
    #[test]
    fn asteroid_synchronize_2_test() {
        let mut asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        let asteroid3 = Asteroid::new_pos_and_size(0., 0., 20.);
        asteroid1.set_last_updated(0.0);
        asteroid2.set_last_updated(2.0);

        let asteroids = BTreeMap::new();
        let asteroids_c = asteroids.clone();

        let mut field1 = Asteroids {
            count: 0,
            asteroids,
        };

        let mut field2 = Asteroids {
            count: 0,
            asteroids: asteroids_c,
        };

        field1.add_asteroid("f1".to_string(), asteroid1.clone());
        field1.add_asteroid("f1".to_string(), asteroid2.clone());
        field2.add_asteroid("f2".to_string(), asteroid1.clone());
        field2.add_asteroid("f2".to_string(), asteroid2.clone());
        field2.add_asteroid("f2".to_string(), asteroid3.clone());

        // let asteroids = synchronize_asteroids(&mut field1, field2, "f2".to_string());
        // assert!(asteroids.asteroids.get("f1_000000").unwrap() == &asteroid1);
        // assert!(asteroids.asteroids.get("f1_000001").unwrap() == &asteroid2);
        // assert!(asteroids.asteroids.get("f2_000002").unwrap() == &asteroid3);
        synchronize_asteroids(&mut field1, field2, "f2".to_string());
        assert!(field1.asteroids.get("f1_000000").unwrap() == &asteroid1);
        assert!(field1.asteroids.get("f1_000001").unwrap() == &asteroid2);
        assert!(field1.asteroids.get("f2_000002").unwrap() == &asteroid3);
    }

    /// New asteroid in field1
    #[test]
    fn asteroid_synchronize_6_test() {
        let mut asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        let asteroid3 = Asteroid::new_pos_and_size(0., 0., 20.);
        asteroid1.set_last_updated(0.0);
        asteroid2.set_last_updated(2.0);

        let asteroids = BTreeMap::new();
        let asteroids_c = asteroids.clone();

        let mut field1 = Asteroids {
            count: 0,
            asteroids,
        };

        let mut field2 = Asteroids {
            count: 0,
            asteroids: asteroids_c,
        };

        field1.add_asteroid("f1".to_string(), asteroid1.clone());
        field1.add_asteroid("f1".to_string(), asteroid2.clone());
        field2.add_asteroid("f1".to_string(), asteroid1.clone());
        field2.add_asteroid("f1".to_string(), asteroid2.clone());
        field2.add_asteroid("f1".to_string(), asteroid3.clone());

        // let asteroids = synchronize_asteroids(&mut field1, field2, "f2".to_string());
        // assert!(asteroids.asteroids.get("f1_000000").unwrap() == &asteroid1);
        // assert!(asteroids.asteroids.get("f1_000001").unwrap() == &asteroid2);
        // assert!(asteroids.asteroids.get("f1_000002").unwrap() == &asteroid3);
        synchronize_asteroids(&mut field1, field2, "f2".to_string());
        assert!(field1.asteroids.get("f1_000000").unwrap() == &asteroid1);
        assert!(field1.asteroids.get("f1_000001").unwrap() == &asteroid2);
        assert!(field1.asteroids.get("f1_000002").unwrap() == &asteroid3);
    }

    /// Asteroid updated in field2
    #[test]
    fn asteroid_synchronize_3_test() {
        let mut asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid3 = Asteroid::new_pos_and_size(0., 0., 10.);
        asteroid1.set_last_updated(0.0);
        asteroid2.set_last_updated(2.0);
        asteroid3.set_last_updated(50.0);

        let asteroids = BTreeMap::new();
        let asteroids_c = asteroids.clone();

        let mut field1 = Asteroids {
            count: 0,
            asteroids,
        };

        let mut field2 = Asteroids {
            count: 0,
            asteroids: asteroids_c,
        };

        field1.add_asteroid("f1".to_string(), asteroid1.clone());
        field1.add_asteroid("f1".to_string(), asteroid2.clone());
        field2.add_asteroid("f1".to_string(), asteroid1.clone());
        field2.add_asteroid("f1".to_string(), asteroid3.clone());

        // let asteroids = synchronize_asteroids(&mut field1, field2, "f2".to_string());
        // assert!(asteroids.asteroids.get("f1_000000").unwrap() == &asteroid1);
        // assert!(asteroids.asteroids.get("f1_000001").unwrap() == &asteroid3);
        synchronize_asteroids(&mut field1, field2, "f2".to_string());
        assert!(field1.asteroids.get("f1_000000").unwrap() == &asteroid1);
        assert!(field1.asteroids.get("f1_000001").unwrap() == &asteroid3);
    }

    /// Asteroid not updated in field2
    #[test]
    fn asteroid_synchronize_4_test() {
        let mut asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        let mut asteroid3 = Asteroid::new_pos_and_size(0., 0., 10.);
        asteroid1.set_last_updated(0.0);
        asteroid2.set_last_updated(60.0);
        asteroid3.set_last_updated(50.0);

        let asteroids = BTreeMap::new();
        let asteroids_c = asteroids.clone();

        let mut field1 = Asteroids {
            count: 0,
            asteroids,
        };

        let mut field2 = Asteroids {
            count: 0,
            asteroids: asteroids_c,
        };

        field1.add_asteroid("f1".to_string(), asteroid1.clone());
        field1.add_asteroid("f1".to_string(), asteroid2.clone());
        field2.add_asteroid("f1".to_string(), asteroid1.clone());
        field2.add_asteroid("f1".to_string(), asteroid3.clone());

        // let asteroids = synchronize_asteroids(&mut field1, field2, "f2".to_string());
        // assert!(asteroids.asteroids.get("f1_000000").unwrap() == &asteroid1);
        // assert!(asteroids.asteroids.get("f1_000001").unwrap() == &asteroid2);
        synchronize_asteroids(&mut field1, field2, "f2".to_string());
        assert!(field1.asteroids.get("f1_000000").unwrap() == &asteroid1);
        assert!(field1.asteroids.get("f1_000001").unwrap() == &asteroid2);
    }

    #[test]
    fn asteroid_refresh_last_updated_test() {
        let asteroid1 = Asteroid::new_pos_and_size(0., 0., 10.);
        let asteroid2 = Asteroid::new_pos_and_size(0., 0., 10.);
        let asteroid3 = Asteroid::new_pos_and_size(0., 0., 10.);

        let asteroids = BTreeMap::new();

        let mut field = Asteroids {
            count: 0,
            asteroids,
        };

        field.add_asteroid("f1".to_string(), asteroid1.clone());
        field.add_asteroid("f1".to_string(), asteroid2.clone());
        field.add_asteroid("f1".to_string(), asteroid3.clone());

        field.refresh_last_updated(10.0);

        assert_eq!(
            field.asteroids.get("f1_000000").unwrap().last_updated(),
            10.
        );
        assert_eq!(
            field.asteroids.get("f1_000001").unwrap().last_updated(),
            10.
        );
        assert_eq!(
            field.asteroids.get("f1_000002").unwrap().last_updated(),
            10.
        );
    }

    #[test]
    fn generate_field_nb_item_test() {
        async fn amain() {
            let field = Asteroids::generate_field("planetoid".to_string(), 3);
            assert_eq!(field.count, 3);
            assert_eq!(field.asteroids.len(), 3);

            let field = Asteroids::generate_field("planetoid".to_string(), 3);
            let mut keys: Vec<&String> = field.asteroids.keys().collect();
            keys.sort();
            assert_eq!(
                keys,
                ["planetoid_000000", "planetoid_000001", "planetoid_000002"]
            );
        }
        macroquad::Window::from_config(window_conf(), amain());
    }
}
