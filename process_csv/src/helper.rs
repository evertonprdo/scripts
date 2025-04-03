use std::error::Error;

use crate::{CR, QUOTES};

pub struct CellParser {}
impl CellParser {
    /// Converts a byte vector into a UTF-8 string, handling quoted cells.
    ///
    /// # Parameters
    /// - `cell: Vec<u8>` - The byte vector representing the CSV cell.
    ///
    /// # Returns
    /// - `Result<String, Box<dyn Error>>` - The parsed string or an error if UTF-8 conversion fails.
    pub fn to_string(mut cell: Vec<u8>) -> Result<String, Box<dyn Error>> {
        if cell.get(0) == Some(&QUOTES) {
            Self::normalize(&mut cell);
        }

        // Remove trailing carriage return (`\r`) from CRLF-terminated cells
        if cell.last() == Some(&CR) {
            cell.pop();
        }

        String::from_utf8(cell).map_err(|e| e.into())
    }

    /// Normalizes a quoted CSV cell by handling escaped quotes removing enclosing quotes
    /// and replacing double double-quotes ("") with a single quote (")
    ///
    /// # Parameters
    /// - `cell: &mut Vec<u8>` - The mutable byte vector to normalize.
    fn normalize(cell: &mut Vec<u8>) {
        let mut write = 0;
        let mut read = 1;

        while read < cell.len() - 1 {
            cell[write] = cell[read];

            if cell[read] == QUOTES && cell[read + 1] == QUOTES {
                read += 1;
            }

            write += 1;
            read += 1;
        }

        cell.truncate(write);
    }
}
