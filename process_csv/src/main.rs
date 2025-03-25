use process_csv::ProcessCSV;

fn main() {
    let path = "sample.csv";
    let process_csv = ProcessCSV::from(path);

    for line in process_csv {
        println!("{:?}", line);
    }
}
