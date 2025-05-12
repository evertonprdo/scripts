use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    id: u64,
    name: String,
    age: u8,
    height_cm: f32,
    hobbies: Vec<String>,
}
impl Person {
    pub fn new(name: String, age: u8, height_cm: f32, hobbies: Vec<String>) -> Self {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Person {
            id: id,
            name,
            age,
            height_cm,
            hobbies,
        }
    }
}
