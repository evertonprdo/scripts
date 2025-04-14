use utils::AverageCollection;

fn main() {
    let mut c = AverageCollection::from(vec![5, 10]);
    println!("{:?}", c.average());
}
