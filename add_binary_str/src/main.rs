use std::{env, process};

use add_binary_str::Binary;

fn main() {
    let (a, b) = parse_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{} + {} = {}", a, b, Binary::add(&a, &b));
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<(Binary, Binary), &'static str> {
    args.next();

    let a = match args.next() {
        Some(arg) => Binary::build(arg)?,
        None => return Err("Dint't get an 'a' value"),
    };

    let b = match args.next() {
        Some(arg) => Binary::build(arg)?,
        None => return Err("Dint't get a 'b' value"),
    };

    Ok((a, b))
}
