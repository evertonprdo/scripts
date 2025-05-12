use std::{fs::File, io::ErrorKind};

use crate::person::Person;

const DB_PATH: &str = "db.json";
pub struct Database {
    db: Vec<Person>,
}
impl Database {
    pub fn new() -> Self {
        let db = match File::open(DB_PATH) {
            Ok(f) => match serde_json::from_reader(f) {
                Ok(data) => data,
                Err(err) if err.is_eof() => Vec::new(),
                _ => panic!("Problem reading database"),
            },

            Err(err) if err.kind() == ErrorKind::NotFound => Vec::new(),
            Err(err) => panic!("Problem opening database: {:?}", err),
        };

        Database { db }
    }

    fn persist(&mut self) {
        const ERROR: &str = "Error persisting data";

        let file = File::create(DB_PATH).expect(ERROR);
        serde_json::to_writer(file, &self.db).expect(ERROR);
    }

    pub fn insert(&mut self, data: Person) {
        self.db.push(data);
        self.persist();
    }

    pub fn get_all(&self) -> &Vec<Person> {
        &self.db
    }
}
