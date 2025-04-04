use std::{error::Error, str::FromStr};

use crate::{CR, QUOTES};

enum Trim {
    Quotes = 1,
    QuotesAndCR = 2,
}

pub struct CellParser {}
impl CellParser {
    #[rustfmt::skip]
    pub fn to_string(mut cell: Vec<u8>) -> Result<String, Box<dyn Error>> {
        match (cell.first(), cell.last()) {
            (Some(&QUOTES), Some(&CR)) => Self::normalize(&mut cell, Trim::QuotesAndCR),
            (Some(&QUOTES), _)         => Self::normalize(&mut cell, Trim::Quotes),
            (_, Some(&CR))             => { cell.pop(); }
            _                          => {}
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

    /// Normalizes a quoted CSV cell:
    /// - Removes enclosing quotes
    /// - Replaces double quotes ("") with a single quote (")
    ///
    /// `n`: number of characters to trim from start/end (e.g., 1 or 2)
    fn normalize(cell: &mut Vec<u8>, n: Trim) {
        let n = n as usize;
        if cell.len() < n + 1 {
            return cell.clear();
        }

        let mut w = 0;
        let mut r = 1;

        while r < cell.len() - n {
            cell[w] = cell[r];

            if cell[r] == QUOTES && cell[r + 1] == QUOTES {
                r += 1;
            }

            w += 1;
            r += 1;
        }

        cell.truncate(w);
    }
}
