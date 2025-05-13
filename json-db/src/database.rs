use std::{fs::File, io::ErrorKind};

use serde::{Deserialize, Serialize};

use crate::person::Person;

pub enum Insert {
    Person(Person),
}

#[derive(Deserialize, Serialize)]
struct Schema {
    person: Vec<Person>,
}
impl Schema {
    fn new() -> Self {
        Schema { person: Vec::new() }
    }
}

const DB_PATH: &str = "db.json";
pub struct Database {
    db: Schema,
}
impl Database {
    pub fn new() -> Self {
        let db = match File::open(DB_PATH) {
            Ok(f) => match serde_json::from_reader(f) {
                Ok(data) => data,
                Err(err) if err.is_eof() => Schema::new(),
                _ => panic!("Problem reading database"),
            },

            Err(err) if err.kind() == ErrorKind::NotFound => Schema::new(),
            Err(err) => panic!("Problem opening database: {:?}", err),
        };

        Database { db }
    }

    fn persist(&mut self) {
        const ERROR: &str = "Error persisting data";

        let file = File::create(DB_PATH).expect(ERROR);
        serde_json::to_writer(file, &self.db).expect(ERROR);
    }

    pub fn insert(&mut self, data: Insert) {
        match data {
            Insert::Person(person) => self.db.person.push(person),
        }
        self.persist();
    }

    pub fn get_person(&self) -> &Vec<Person> {
        &self.db.person
    }
}
