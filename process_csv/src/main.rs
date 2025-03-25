use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "sample.csv";

    let process_csv = ProcessCSV::new(path);

    for line in process_csv {
        println!("{:?}", line);
    }
}

struct ProcessCSV {
    buffer: BufReader<File>,
}
impl ProcessCSV {
    fn new(path: &str) -> Self {
        let f = File::open(path).unwrap();
        ProcessCSV {
            buffer: BufReader::new(f),
        }
    }
}
impl Iterator for ProcessCSV {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line: String = String::new();

        match self.buffer.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    return None;
                }
            }
            Err(_) => panic!(),
        }

        let mut iter = line.as_bytes().iter();
        let mut cell: Self::Item = Vec::new();

        let mut skip = false;
        let mut i = 0;
        let mut j = 0;

        while let Some(b) = iter.next() {
            if *b == 34 {
                skip = !skip;
            }

            if !skip && *b == 44 {
                cell.push(line[j..i].to_string());
                j = i + 1;
            }

            i += 1;
        }

        if let Some(10) = line.as_bytes().iter().next_back() {
            i -= 1;
        }

        cell.push(line[j..i].to_string());
        Some(cell)
    }
}
