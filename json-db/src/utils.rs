use std::{fmt::Debug, io, str::FromStr};

pub fn stdin_str() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    str.pop();
    str
}

pub fn stdin_num<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    stdin_str().parse::<T>().unwrap()
}
