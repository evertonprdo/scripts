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

// Consumes a CSV file at once and receives an on_new_line callback for new lines.
impl CsvReader {
    const LF: u8 = 10;
    const COMMA: u8 = 44;
    const QUOTES: u8 = 34;

    fn parse_cell(line: &[u8]) -> Result<String, Box<dyn Error>> {
        String::from_utf8(line.to_vec()).map_err(|e| e.into())
    }
    /*
        pub fn for_each_line<F>(&mut self, on_new_line: F) -> Result<(), Box<dyn Error>>
        where
            F: Fn(Vec<String>),
        {
            let mut line_holder: Vec<String> = Vec::new();
            let mut unp_bytes: Vec<u8> = Vec::new(); // unprocessed_bytes

            while let Some(mut chunk) = self.read_chunk()? {
                if !unp_bytes.is_empty() {
                    unp_bytes.append(&mut chunk);
                    chunk = mem::take(&mut unp_bytes);
                }

                let remaining = Self::split_to_string(&mut chunk, |bond| match bond {
                    BoundaryEvent::NewCell(c) => {
                        line_holder.push(Self::parse_cell(c)?);
                        Ok(())
                    }

                    BoundaryEvent::NewLine => {
                        let capacity = line_holder.capacity();
                        on_new_line(mem::replace(&mut line_holder, Vec::with_capacity(capacity)));
                        Ok(())
                    }
                })?;

                unp_bytes = Vec::from(remaining);
            }

            line_holder.push(Self::parse_cell(&unp_bytes)?);
            on_new_line(mem::take(&mut line_holder));

            return Ok(());
        }
    */

    pub fn for_each_raw_line<F>(&mut self, on_new_line: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(Vec<ByteCell>),
    {
        let mut chunk = vec![0; self.watermark];
        let mut unp_bytes: Vec<u8> = Vec::new(); // unprocessed_bytes
        let mut line_holder: Vec<ByteCell> = Vec::new();

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
                BoundaryEvent::NewCell(c) => line_holder.push(c.to_vec()),

                BoundaryEvent::NewLine => {
                    let capacity = line_holder.capacity();
                    on_new_line(mem::replace(&mut line_holder, Vec::with_capacity(capacity)));
                }
            });

            unp_bytes = Vec::from(remaining);
        }

        line_holder.push(unp_bytes);
        on_new_line(mem::take(&mut line_holder));

        return Ok(());
    }

    fn split_to_string<'a, F>(
        chunk: &'a mut [u8],
        mut on_boundary: F,
    ) -> Result<&'a [u8], Box<dyn Error>>
    where
        F: FnMut(BoundaryEvent<'a>) -> Result<(), Box<dyn Error>>,
    {
        let mut cb_result: Result<(), Box<dyn Error>> = Ok(());
        let remaining = Self::split_chunk(chunk, |boundary| cb_result = on_boundary(boundary));

        cb_result.map(|_| remaining)
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
