use std::fmt::Display;

pub struct Binary {
    pub value: String,
}
impl Binary {
    pub fn build(str: String) -> Result<Self, &'static str> {
        if Self::is_valid(&str) {
            return Ok(Self { value: str });
        }
        Err("Invalid String, only '0', '1' are supported")
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Binary {
    pub fn is_valid(str: &str) -> bool {
        str.chars().all(|c| "01".contains(c))
    }

    /// Adds two binary numbers represented as `Binary` objects and returns the sum as a new `Binary`.
    ///
    /// # Arguments
    ///
    /// * `a` - A reference to the first binary number.
    /// * `b` - A reference to the second binary number.
    ///
    /// # Returns
    ///
    /// A `Binary` object representing the sum of the two binary numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use add_binary_str::Binary;
    ///
    /// let a = Binary { value: "1101".to_string() };
    /// let b = Binary { value: "1011".to_string() };
    /// assert_eq!(Binary::add(&a, &b).value, "11000");
    /// ```
    pub fn add(a: &Binary, b: &Binary) -> Binary {
        let mut iter_a = a.value.bytes();
        let mut iter_b = b.value.bytes();

        let max_size = iter_a.len().max(iter_b.len()) + 1;
        let mut sum: Vec<u8> = vec![0; max_size];

        let mut carry = false;
        let mut i: usize = sum.len() - 1;

        loop {
            let byte_a = iter_a.next_back();
            let byte_b = iter_b.next_back();

            if byte_a == None && byte_b == None {
                break;
            }

            let byte_a = byte_a.unwrap_or(b'0');
            let byte_b = byte_b.unwrap_or(b'0');

            sum[i] = match (carry, byte_a == byte_b) {
                (true, true) => {
                    if byte_a == b'0' {
                        carry = false
                    };
                    b'1'
                }
                (false, true) => {
                    if byte_a == b'1' {
                        carry = true
                    };
                    b'0'
                }

                (true, false) => b'0',
                (false, false) => b'1',
            };

            i -= 1;
        }

        if carry {
            sum[i] = b'1';
        } else {
            sum = sum[1..].to_vec();
        }

        Binary {
            value: String::from_utf8(sum).unwrap(),
        }
    }
}
