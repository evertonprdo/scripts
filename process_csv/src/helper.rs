use std::error::Error;

use crate::QUOTES;

pub struct CellParser {}
impl CellParser {
    pub fn to_string(mut cell: Vec<u8>) -> Result<String, Box<dyn Error>> {
        if cell.get(0) == Some(&QUOTES) {
            Self::normalize(&mut cell);
        }

        String::from_utf8(cell).map_err(|e| e.into())
    }

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
