use std::sync::mpsc;
use std::time::Instant;
use std::{env, mem, process, thread};

use process_csv::{CellParser, Config, CsvReader, YieldEvent};

fn main() {
    let config = Config::build_from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let process_csv = CsvReader::build_from(config).unwrap_or_else(|err| {
        eprintln!("Problem to open file: {err}");
        process::exit(1);
    });

    let start = Instant::now();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut arr = Vec::new();

        if let Err(e) = process_csv.process_file(|event| match event {
            YieldEvent::NewCell(cell) => {
                arr.push(cell);
            }
            YieldEvent::NewLine => {
                let cap = arr.len();
                tx.send(mem::replace(&mut arr, Vec::with_capacity(cap)))
                    .unwrap();
            }
        }) {
            eprintln!("Application error: {e}");
            process::exit(1);
        };
    });

    let _ = rx.recv();

    for received in rx {
        let [n, a, m, c]: [Vec<u8>; 4] = received.try_into().expect("Expected 4 elements");

        let user = User {
            name: CellParser::to_string(n).unwrap(),
            age: CellParser::to_int(a).unwrap(),
            mail: CellParser::to_string(m).unwrap(),
            country: CellParser::to_string(c).unwrap(),
        };

        println!(
            "user: {}\nage: {}\nmail: {}\ncountry: {}\n",
            user.name, user.age, user.mail, user.country
        );
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

struct User {
    name: String,
    age: u8,
    mail: String,
    country: String,
}
