use std::collections::HashMap;

pub struct RomanNumeral {
    numeral: String,
}
impl RomanNumeral {
    pub fn build(string: String) -> Result<Self, &'static str> {
        if Self::is_valid(&string) {
            return Ok(RomanNumeral { numeral: string });
        }

        return Err("Only roman characters are supported 'I, V, X, L, C, D, M'");
    }

    pub fn build_from(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let roman = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a roman string"),
        };

        Ok(Self::build(roman)?)
    }
}

impl RomanNumeral {
    /// Only checks whether the String contain any invalid char
    ///
    /// # Examples
    ///
    /// ```
    ///     use roman_to_int::RomanNumeral;
    ///
    ///     let valid_n = "MCMXCIV";
    ///     assert!(RomanNumeral::is_valid(&valid_n));
    ///
    ///     let invalid_n = "ABCD";
    ///     assert!(!RomanNumeral::is_valid(&invalid_n));
    /// ```
    pub fn is_valid(s: &str) -> bool {
        s.chars()
            .all(|c| "IVXLCDM".contains(c.to_ascii_uppercase()))
    }

    /// Converts a Roman numeral to a `u32`.
    ///
    /// # Examples
    ///
    /// ```
    ///     use roman_to_int::RomanNumeral;
    ///
    ///     let roman = RomanNumeral::build("MCMXCIV".to_string()).unwrap();
    ///     assert_eq!(1994, roman.to_integer());
    /// ```
    pub fn to_integer(&self) -> u32 {
        let roman_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut prev = 0;
        let mut result = 0;

        for c in self.numeral.chars().rev() {
            let val = roman_map[&c];

            if val < prev {
                result -= val;
            } else {
                result += val;
            }

            prev = val;
        }

        return result;
    }
}
