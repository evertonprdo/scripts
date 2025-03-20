fn main() {
    let a = "1010";
    let b = "1011";

    println!("{a} + {b} = {}", add_binary(&a, &b));
}

const ZERO: u8 = 48;
const ONE: u8 = 49;

#[rustfmt::skip]
pub fn add_binary(a: &str, b: &str) -> String {
    let mut iter_a = a.bytes();
    let mut iter_b = b.bytes();

    let mut sum: Vec<u8> = 
        vec![0; if a.len() > b.len() { a.len() } else { b.len() } + 1];
    
    let mut carry = false;
    let mut i: usize = sum.len() - 1;

    loop {
        let bit_a = iter_a.next_back();
        let bit_b = iter_b.next_back();

        if bit_a == None && bit_b == None { break; }

        let bit_a = if let Some(v) = bit_a { v } else { ZERO };
        let bit_b = if let Some(v) = bit_b { v } else { ZERO };

        sum[i] = match (carry, bit_a == bit_b) {
            (true, true) => { if bit_a == ZERO { carry = false }; ONE }
            (false, true) => { if bit_a == ONE { carry = true }; ZERO }

            (true, false) => ZERO,
            (false, false) => ONE,
        };

        i -= 1;
    }

    if carry {
        sum[i] = ONE;
        return String::from_utf8(sum).unwrap();
    }
    String::from_utf8(sum[1..sum.len()].to_vec()).unwrap()
}
