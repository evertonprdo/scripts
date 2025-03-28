use std::{env, process};

use process_csv::{ChunkReader, Config};

fn main() {
    let config = Config::build_from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut process_csv = ChunkReader::build_from(config).unwrap_or_else(|err| {
        eprintln!("Problem to open file: {err}");
        process::exit(1);
    });

    process_csv.run(|x| println!("{:?}", x));
}
