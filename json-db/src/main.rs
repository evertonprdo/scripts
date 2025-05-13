use std::io;

use json_db::{Database, Insert, Person};

fn main() {
    let mut db = Database::new();
    println!("### Current users on database ###");
    println!("{:?}", db.get_person());

    println!("### User Registration ###");

    println!("Please type the name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name.pop();

    println!("Please type the age:");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age = age.trim().parse::<u8>().expect("Invalid age");

    println!("Please type the height_cm:");
    let mut height_cm = String::new();
    io::stdin()
        .read_line(&mut height_cm)
        .expect("Failed to read line");

    let height_cm = height_cm.trim().parse::<f32>().expect("Invalid height_cm");

    let person = Person::new(name, age, height_cm, vec![]);
    db.insert(Insert::Person(person));

    println!("{:?}", db.get_person());
}
