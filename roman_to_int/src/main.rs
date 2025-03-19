use std::collections::HashMap;

fn main() {
    let s = "MCMXCIV";
    println!("{s}: {}", roman_to_int(s));
}

fn roman_to_int(s: &str) -> i32 {
    let roman_map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut prev = 0;
    let mut result = 0;

    for c in s.chars().rev() {
        let val = roman_map[&c];

        if val < prev {
            result -= val;
        } else {
            result += val;
        }

        prev = val;
    }

    return result;
}
