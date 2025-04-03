use std::sync::mpsc;
use std::time::Instant;
use std::{env, process, thread};

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
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = process_csv.process_file(|x| tx.send(x).unwrap()) {
            eprintln!("Application error: {e}");
            process::exit(1);
        };
    });

    for received in rx {
        match received {
            YieldEvent::NewCell(cell) => {
                print!(" {} |", CellParser::to_string(cell).unwrap());
            }
            YieldEvent::NewLine => print!("\n"),
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
