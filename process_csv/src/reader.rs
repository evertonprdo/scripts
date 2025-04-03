use std::{error::Error, fs::File, io::Read, mem};

use crate::Config;

pub struct CsvReader {
    file: File,
    watermark: usize,
}
impl CsvReader {
    pub fn build_from(config: Config) -> Result<Self, Box<dyn Error>> {
        let file = File::open(config.file_path)?;

        let watermark = config.watermark.unwrap_or(1024 * 8); // 8KB

        Ok(CsvReader { file, watermark })
    }
}

type ByteCell = Vec<u8>;
enum BoundaryEvent<'a> {
    NewCell(&'a [u8]),
    NewLine,
}
pub enum YieldEvent {
    NewCell(ByteCell),
    NewLine,
}

// Consumes a CSV file at once and receives an on_new_line callback for new lines.
impl CsvReader {
    const LF: u8 = 10;
    const COMMA: u8 = 44;
    const QUOTES: u8 = 34;

    pub fn process_file<F>(&mut self, on_yield: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(YieldEvent),
    {
        let mut chunk = vec![0; self.watermark];
        let mut unp_bytes: Vec<u8> = Vec::new(); // unprocessed_bytes

        loop {
            let n = self.file.read(&mut chunk)?;
            if n == 0 {
                break;
            }

            chunk.truncate(n);

            if !unp_bytes.is_empty() {
                unp_bytes.append(&mut chunk);
                chunk = mem::take(&mut unp_bytes);
            }

            let remaining = Self::split_chunk(&mut chunk, |boundary| match boundary {
                BoundaryEvent::NewCell(c) => on_yield(YieldEvent::NewCell(c.to_vec())),
                BoundaryEvent::NewLine => on_yield(YieldEvent::NewLine),
            });

            unp_bytes = Vec::from(remaining);
        }

        on_yield(YieldEvent::NewCell(unp_bytes));
        return Ok(());
    }

    fn split_chunk<'a, F>(chunk: &'a mut [u8], mut on_boundary: F) -> &'a [u8]
    where
        F: FnMut(BoundaryEvent<'a>),
    {
        let mut between_quotes = false;
        let mut j = 0;

        for (i, byte) in chunk.iter().enumerate() {
            if *byte == Self::QUOTES {
                between_quotes = !between_quotes;
            }
            if between_quotes {
                continue;
            }

            if *byte == Self::COMMA || *byte == Self::LF {
                on_boundary(BoundaryEvent::NewCell(&chunk[j..i]));
                j = i + 1;
            }
            if *byte == Self::LF {
                on_boundary(BoundaryEvent::NewLine);
            }
        }

        &chunk[j..]
    }
}
