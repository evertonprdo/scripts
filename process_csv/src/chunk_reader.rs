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

// Consumes a CSV File at once and receives a on_new_line callback for on new lines
impl ChunkReader {
    const LF: u8 = 10;
    const COMMA: u8 = 44;
    const QUOTES: u8 = 34;

    pub fn run<F>(&mut self, on_new_line: F) -> Result<(), &'static str>
    where
        F: Fn(Vec<String>),
    {
        let mut result: Vec<String> = Vec::new();
        let mut unprocessed_bytes: Vec<u8> = Vec::new();

        while let Some(chunk) = self.read_chunk()? {
            let mut chunk = chunk;

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
