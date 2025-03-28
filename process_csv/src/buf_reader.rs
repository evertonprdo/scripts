use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct CsvReader {
    buffer: BufReader<File>,
}
impl CsvReader {
    pub fn from(path: &str) -> Result<Self, &'static str> {
        let f = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open file"),
        };

        Ok(CsvReader {
            buffer: BufReader::new(f),
        })
    }

    fn read_line(&mut self) -> Option<String> {
        let mut line: String = String::new();
        match self.buffer.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    return None;
                }
            }
            Err(_) => panic!(),
        }
        Some(line)
    }

    fn parse_line(line: &str) -> Vec<String> {
        let mut iter = line.as_bytes().iter();
        let mut cell: Vec<String> = Vec::new();

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
        cell
    }
}
impl Iterator for CsvReader {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::parse_line(&self.read_line()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_line() {
        let line = "Sample,Header,Example";
        assert_eq!(
            vec!["Sample", "Header", "Example"],
            CsvReader::parse_line(line)
        );
    }

    #[test]
    fn process_line_doublequote() {
        let line = "Sample,\"He\"\"@add\"\"ader\",Example\n";
        assert_eq!(
            vec!["Sample", "\"He\"\"@add\"\"ader\"", "Example"],
            CsvReader::parse_line(line)
        );
    }
}
