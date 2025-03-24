use std::fs;

fn main() {
    let content = fs::read_to_string("sample.csv").unwrap();
    process_csv(content);
}

const DOUBLEQUOTE: u8 = 34;
const LINE_FEED: u8 = 10;
const COMMA: u8 = 44;

fn process_csv(csv: String) {
    let mut is_between_doublequote = false;
    let mut iter = csv.as_bytes().iter();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while let Some(c) = iter.next() {
        if *c == DOUBLEQUOTE {
            is_between_doublequote = !is_between_doublequote;
        }

        if !is_between_doublequote && *c == COMMA {
            print!("| {} ", csv[j..i].to_string());
            j = i + 1;
        }

        if !is_between_doublequote && *c == LINE_FEED {
            print!("{} |", csv[j..i].to_string());
            j = i + 1;

            print!("\n");
        }

        i += 1;
    }
    print!(" |\n");
}

//     j
//         v
// Name,Age,Email,Country
