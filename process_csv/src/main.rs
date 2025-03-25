use process_csv::CsvReader;

fn main() {
    let path = "sample.csv";
    let process_csv = CsvReader::from(path);

    for line in process_csv {
        println!("{:?}", line);
    }
}
