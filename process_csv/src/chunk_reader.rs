use core::str;
use std::{fs::File, io::Read, mem};

use crate::Config;

pub struct ChunkReader {
    file: File,
    chunk: Vec<u8>,
}
impl ChunkReader {
    pub fn build_from(config: Config) -> Result<Self, &'static str> {
        let file = match File::open(config.file_path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open file"),
        };

        let watermark = config.watermark.unwrap_or(1024 * 8); // 8KB

        Ok(ChunkReader {
            file,
            chunk: vec![0; watermark],
        })
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
}

// Consumes a CSV file at once and receives an on_new_line callback for new lines.
impl ChunkReader {
    const LF: u8 = 10;
    const COMMA: u8 = 44;
    const QUOTES: u8 = 34;

    pub fn run<F>(&mut self, on_new_line: F) -> Result<(), &'static str>
    where
        F: Fn(Vec<String>),
    {
        let mut result: Vec<String> = Vec::new(); // Vector to hold each line until the callback is called
        let mut unprocessed_bytes: Vec<u8> = Vec::new(); // Vector to hold the remaining unprocessed bytes of the chunk

        // Keep going until the entire file is read.
        while let Some(mut chunk) = self.read_chunk()? {
            // chunk: [3, 4, 5],          unp_bytes: [0, 1, 2]
            // chunk: [0, 1, 2, 3, 4, 5], unp_bytes: []
            if !unprocessed_bytes.is_empty() {
                unprocessed_bytes.append(&mut chunk);
                mem::swap(&mut unprocessed_bytes, &mut chunk);
            }

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
                    result.push(Self::parse_cell(&chunk[j..i])?);
                    j = i + 1;
                }
                if *byte == Self::LF {
                    let capacity = result.len();
                    on_new_line(mem::replace(&mut result, Vec::with_capacity(capacity)));
                }
            }

            if j > 0 {
                unprocessed_bytes.extend(chunk.drain(j..));
            } else {
                unprocessed_bytes = chunk;
            }
        }

        // Process the remaining unprocessed bytes (last cell).
        result.push(Self::parse_cell(&unprocessed_bytes)?);
        on_new_line(result);

        return Ok(());
    }

    fn parse_cell(line: &[u8]) -> Result<String, &'static str> {
        match str::from_utf8(line) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err("failed when parsing cell"),
        }
    }
}

enum Boundary<'a> {
    Cell(&'a [u8]),
    NewLine,
}

impl ChunkReader {
    pub fn run_2<F>(&mut self, on_new_line: F) -> Result<(), &'static str>
    where
        F: Fn(Vec<String>),
    {
        let mut result: Vec<String> = Vec::new();
        let mut unprocessed_bytes: Vec<u8> = Vec::new();

        while let Some(mut chunk) = self.read_chunk()? {
            if !unprocessed_bytes.is_empty() {
                unprocessed_bytes.append(&mut chunk);
                mem::swap(&mut unprocessed_bytes, &mut chunk);
            }

            let ump_b = Self::process_chunk(&mut chunk, |bond| match bond {
                Boundary::Cell(c) => {
                    result.push(Self::parse_cell(c).unwrap());
                }
                Boundary::NewLine => {
                    let capacity = result.capacity();
                    on_new_line(mem::replace(&mut result, Vec::with_capacity(capacity)));
                }
            });

            if !ump_b.is_empty() {
                unprocessed_bytes.extend_from_slice(ump_b);
            }
        }

        result.push(Self::parse_cell(&unprocessed_bytes)?);
        on_new_line(result);

        return Ok(());
    }

    fn process_chunk<'a, F>(chunk: &'a mut [u8], mut on_boundary: F) -> &'a [u8]
    where
        F: FnMut(Boundary<'a>),
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
                on_boundary(Boundary::Cell(&chunk[j..i]));
                j = i + 1;
            }
            if *byte == Self::LF {
                on_boundary(Boundary::NewLine);
            }
        }

        &chunk[j..]
    }
}
