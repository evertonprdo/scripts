/// `CsvReader` looks more like a byte splitter. It takes a file path and a
/// watermark indicating the size of the chunk that is read each time.
///
/// In order to return a byte cell whenever one is found, the `process_file` function takes
/// a callback with a `YieldEvent` enum parameter indicating which boundary was triggered,
/// `NewCell` (i.e., Comma (`,`)) or `NewLine` (i.e., Line Feed(`\n`)).
///
/// Quoted cells are handled correctly, allowing boundaries (such as commas or line feeds)
/// to be included as part of the cell content without splitting the cell. The quotes remain in the cell.
///
/// Note that carriage returns (`\r`) are not removed by the parser and will remain at the end of each cell.
/// If you prefer not to have `\r` characters, consider preprocessing or postprocessing the input to remove them.
///
/// The reader processes data in chunks and invokes user-defined callbacks for further processing.
use std::{error::Error, fs::File, io::Read, mem};

use crate::{COMMA, Config, LF, QUOTES};

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
/// Enum representing yielded events when processing the CSV file.
pub enum YieldEvent {
    NewCell(ByteCell),
    NewLine,
}

impl CsvReader {
    /// Processes the CSV file in chunks and triggers a callback `on_yield` for each cell or line encountered.
    ///
    /// # Parameters
    /// - `on_yield: F`: A function that handles `YieldEvent` occurrences.
    ///
    /// # Returns
    /// - `Result<(), Box<dyn Error>>`
    pub fn process_file<F>(mut self, on_yield: F) -> Result<(), Box<dyn Error>>
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

    /// Splits a chunk of CSV data into individual cells and lines.
    ///
    /// # Parameters
    /// - `chunk: &'a mut [u8]`: The mutable byte slice containing CSV data.
    /// - `on_boundary: F`: A function handling boundary events.
    ///
    /// # Returns
    /// - The remaining unprocessed portion of the chunk.
    fn split_chunk<'a, F>(chunk: &'a mut [u8], mut on_boundary: F) -> &'a [u8]
    where
        F: FnMut(BoundaryEvent<'a>),
    {
        let mut between_quotes = false;
        let mut j = 0;

        for (i, byte) in chunk.iter().enumerate() {
            if *byte == QUOTES {
                between_quotes = !between_quotes;
            }
            if between_quotes {
                continue;
            }

            if *byte == COMMA || *byte == LF {
                on_boundary(BoundaryEvent::NewCell(&chunk[j..i]));
                j = i + 1;
            }
            if *byte == LF {
                on_boundary(BoundaryEvent::NewLine);
            }
        }

        &chunk[j..]
    }
}
