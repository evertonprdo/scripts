use json_db::{Database, Department, Insert, Person, utils};

fn main() {
    let mut db = Database::new();

    println!("### Current users on database ###");
    println!("{:#?}", db.get_person());

    println!("### Current departments on database ###");
    println!("{:#?}", db.get_department());

    loop {
        println!();
        println!("Select an option:");

        println!("1. User registration");
        println!("2. Department registration");
        println!("3. Retrieve Users");
        println!("4. Retrieve Departments");
        println!("0. exit");

        let opt: u8 = utils::stdin_num();
        match opt {
            1 => register_user(&mut db),
            2 => register_department(&mut db),
            3 => println!("\n{:#?}", db.get_person()),
            4 => println!("\n{:#?}", db.get_department()),
            0 => break,
            _ => println!("Invalid option!"),
        }
    }
}

fn register_user(db: &mut Database) {
    println!("### User Registration ###");
    println!("Please type the name:");
    let name = utils::stdin_str();

    println!("Please type the age:");
    let age = utils::stdin_num();

    println!("Please type the height_cm:");
    let height_cm = utils::stdin_num();

    let person = Person::new(name, age, height_cm, vec![]);
    db.insert(Insert::Person(person));

    println!("User registered successfully");
}

fn register_department(db: &mut Database) {
    println!("### Department Registration ###");
    println!("Please type the name:");
    let name = utils::stdin_str();

    println!("Please type the employee count:");
    let employee_count = utils::stdin_num();

    println!("Please type the budget:");
    let budget = utils::stdin_num();

    let department = Department::new(name, employee_count, budget);
    db.insert(Insert::Department(department));

    println!("User registered successfully");
}
