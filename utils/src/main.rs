use utils::{AverageCollection, biggest_number};

fn main() {
    let mut c = AverageCollection::from(vec![5, 10]);
    println!("{:?}", c.average());

    let digits = vec!['4', '2', '9', '4', '9', '6', '7', '2', '9'];
    println!("{:?}", biggest_number(digits))
}
