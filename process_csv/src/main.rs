use std::{env, process};

use process_csv::{Config, CsvReader};

fn main() {
    let config = Config::build_from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut process_csv = CsvReader::build_from(config).unwrap_or_else(|err| {
        eprintln!("Problem to open file: {err}");
        process::exit(1);
    });

    if let Err(e) = process_csv.for_each_line(|x| println!("{:?}", x)) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
