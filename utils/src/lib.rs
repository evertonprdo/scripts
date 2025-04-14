pub mod average_collection;
pub mod collection_analyzer;

use std::collections::HashMap;

pub use average_collection::*;
pub use collection_analyzer::*;

/// Returns the largest number that can be formed from a vector of digit characters.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
///
/// ```
/// use utils::biggest_number;
///
/// let digits = vec!['3', '1', '4', '1'];
/// assert_eq!(biggest_number(digits), 4311);
/// ```
pub fn biggest_number(digits: Vec<char>) -> u32 {
    let mut result = 0;
    let mut map: HashMap<char, i32> = ('0'..='9').map(|c| (c, 0)).collect();

    let mut i = digits.len() as u32;
    let mut j: u32 = 57;

    for dig in digits {
        map.entry(dig).and_modify(|count| *count += 1);
    }

    while i > 0 {
        let mut k = *map.get(&(j as u8 as char)).unwrap();

        while k > 0 {
            i -= 1;
            k -= 1;

            println!("{result} += {} * {}", j - 48, 10_u32.pow(i));

            result += (j - 48) * 10_u32.pow(i);
        }

        j -= 1;
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit() {
        assert_eq!(biggest_number(vec!['5']), 5);
    }

    #[test]
    fn test_multiple_unique_digits() {
        assert_eq!(biggest_number(vec!['1', '2', '3']), 321);
        assert_eq!(biggest_number(vec!['9', '0', '8']), 980);
    }

    #[test]
    fn test_repeated_digits() {
        assert_eq!(biggest_number(vec!['1', '1', '1']), 111);
        assert_eq!(biggest_number(vec!['2', '2', '1']), 221);
    }

    #[test]
    fn test_mixed_digits() {
        assert_eq!(biggest_number(vec!['3', '1', '4', '1']), 4311);
        assert_eq!(biggest_number(vec!['0', '0', '9']), 900);
    }

    #[test]
    fn test_all_digits() {
        let digits = vec!['4', '2', '2', '2', '2', '2', '2', '2', '2', '2'];
        assert_eq!(biggest_number(digits), 4222222222);
    }
}
