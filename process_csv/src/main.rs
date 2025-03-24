use std::fs;

fn main() {
    let content = fs::read_to_string("sample.csv").unwrap();
    process_csv(content, |x| println!("{x}"));
}

fn process_csv<F>(csv: String, callback: F)
where
    F: Fn(String),
{
    let mut is_between_doublequote = false;
    let mut iter = csv.as_bytes().iter();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while let Some(c) = iter.next() {
        if *c == b'"' {
            is_between_doublequote = !is_between_doublequote;
        }

        if !is_between_doublequote && *c == b',' || b'\n' == *c {
            callback(csv[j..i].to_string());
            j = i + 1;
        }

        i += 1;
    }
}
