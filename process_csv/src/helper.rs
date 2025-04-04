use std::{error::Error, str::FromStr};

use crate::{CR, QUOTES};

pub struct CellParser {}
impl CellParser {
    pub fn to_string(mut cell: Vec<u8>) -> Result<String, Box<dyn Error>> {
        // Remove trailing carriage return (`\r`) from CRLF-terminated cells
        if cell.last() == Some(&CR) {
            cell.pop();
        }

        if cell.get(0) == Some(&QUOTES) {
            Self::normalize(&mut cell);
        }

        String::from_utf8(cell).map_err(|e| e.into())
    }

    pub fn to_int<T>(cell: Vec<u8>) -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        T::Err: Error + 'static,
    {
        let cell = String::from_utf8(cell).map_err(|e| -> Box<dyn Error> { Box::new(e) })?;
        cell.parse::<T>()
            .map_err(|e| -> Box<dyn Error> { Box::new(e) })
    }

    /// Normalizes a quoted CSV cell by handling escaped quotes removing enclosing quotes
    /// and replacing double double-quotes ("") with a single quote (")
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
