use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct ProcessCSV {
    buffer: BufReader<File>,
}
impl ProcessCSV {
    pub fn from(path: &str) -> Self {
        let f = File::open(path).unwrap();
        ProcessCSV {
            buffer: BufReader::new(f),
        }
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
}
impl Iterator for ProcessCSV {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.read_line()?;

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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    // And the "unit" test make it clear that the implementation is extremely coupled.
    #[test]
    fn process_line() {
        let path = "test.csv";
        fs::write(path, "Sample,Header,Example").unwrap();

        let mut csv = ProcessCSV::from(path);

        assert_eq!(vec!["Sample", "Header", "Example"], csv.next().unwrap());
    }
}
