use process_csv::chunk_reader::ChunkReader;

fn main() {
    let path = "sample.csv";
    let mut process_csv = ChunkReader::from(path, None);

    process_csv.run(|x| println!("{:?}", x));
}
