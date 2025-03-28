use std::{env, process};

use roman_to_int::RomanNumeral;

fn main() {
    let args = env::args();

    let roman = RomanNumeral::build_from(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{}", roman.to_integer());
}
