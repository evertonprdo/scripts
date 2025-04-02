use core::str;
use std::{fs::File, io::Read, mem};

use crate::Config;

pub struct CsvReader {
    file: File,
    chunk: Vec<u8>,
}
impl CsvReader {
    pub fn build_from(config: Config) -> Result<Self, &'static str> {
        let file = match File::open(config.file_path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open file"),
        };

        let watermark = config.watermark.unwrap_or(1024 * 8); // 8KB

        Ok(CsvReader {
            file,
            chunk: vec![0; watermark],
        })
    }
}

type ByteLine = Vec<Vec<u8>>;
enum BoundaryEvent<'a> {
    NewCell(&'a [u8]),
    NewLine,
}

// Consumes a CSV file at once and receives an on_new_line callback for new lines.
impl CsvReader {
    const LF: u8 = 10;
    const COMMA: u8 = 44;
    const QUOTES: u8 = 34;

    fn parse_cell(line: &[u8]) -> Result<String, &'static str> {
        match str::from_utf8(line) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err("failed when parsing cell"),
        }
    }

    fn read_chunk(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let n = self
            .file
            .read(&mut self.chunk)
            .map_err(|_| "Failed when reading file")?;

        if n == 0 {
            return Ok(None);
        }

        Ok(Some(self.chunk[..n].to_vec()))
    }

    pub fn for_each_line<F>(&mut self, on_new_line: F) -> Result<(), &'static str>
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

    pub fn for_each_raw_line<F>(&mut self, on_new_line: F) -> Result<(), &'static str>
    where
        F: Fn(ByteLine),
    {
        let mut line_holder: ByteLine = Vec::new();
        let mut unp_bytes: Vec<u8> = Vec::new(); // unprocessed_bytes

        while let Some(mut chunk) = self.read_chunk()? {
            if !unp_bytes.is_empty() {
                unp_bytes.append(&mut chunk);
                chunk = mem::take(&mut unp_bytes);
            }

            let remaining = Self::split_chunk(&mut chunk, |bond| match bond {
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
    ) -> Result<&'a [u8], &'static str>
    where
        F: FnMut(BoundaryEvent<'a>) -> Result<(), &'static str>,
    {
        let mut cb_result: Result<(), &'static str> = Ok(());
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
