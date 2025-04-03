use std::time::Instant;
use std::{env, process};

use process_csv::{CellParser, Config, CsvReader, YieldEvent};

fn main() {
    let config = Config::build_from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut process_csv = CsvReader::build_from(config).unwrap_or_else(|err| {
        eprintln!("Problem to open file: {err}");
        process::exit(1);
    });

    let start = Instant::now();

    if let Err(e) = process_csv.process_file(|x| match x {
        YieldEvent::NewCell(cell) => print!("{:?} | ", CellParser::to_string(cell).unwrap()),
        YieldEvent::NewLine => println!(),
    }) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
