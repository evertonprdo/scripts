use std::fs;

fn main() {
    let content = fs::read_to_string("sample.csv").unwrap();
    process_csv(content, |x| println!("{:?}", x));
}

const DOUBLEQUOTE: u8 = 34;
const LINE_FEED: u8 = 10;
const COMMA: u8 = 44;

fn process_csv<F>(csv: String, callback: F)
where
    F: Fn(Vec<String>),
{
    let mut yeld: Vec<String> = Vec::new();
    let mut is_between_doublequote = false;
    let mut iter = csv.as_bytes().iter();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while let Some(c) = iter.next() {
        if *c == DOUBLEQUOTE {
            is_between_doublequote = !is_between_doublequote;
        }

        if !is_between_doublequote && *c == COMMA || LINE_FEED == *c {
            yeld.push(csv[j..i].to_string());
            j = i + 1;
        }

        if !is_between_doublequote && LINE_FEED == *c {
            callback(yeld.clone());
            yeld.clear();
        }

        i += 1;
    }
}
