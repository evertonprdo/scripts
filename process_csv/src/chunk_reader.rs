use std::{fs::File, io::Read};

// Unexpected Behavior
pub struct ChunkReader {
    file: File,
    chunk: Vec<u8>,
}
impl ChunkReader {
    pub fn from(path: &str, watermark: Option<usize>) -> Self {
        let file = File::open(path).unwrap();
        let watermark = if let Some(v) = watermark { v } else { 8 }; // 8KB

        ChunkReader {
            file,
            chunk: vec![0; watermark],
        }
    }
}

impl Iterator for ChunkReader {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.file.read(&mut self.chunk) {
            Ok(n) => {
                if n == 0 {
                    return None;
                }
            }
            Err(_) => return None,
        }

        Some(self.chunk.clone())
    }
}

impl ChunkReader {
    pub fn run(&mut self) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        result.push([].to_vec());

        let mut unprocessed_bytes: Vec<u8> = Vec::new();
        let mut k = 0;

        while let Some(chunk) = self.next() {
            let mut chunk = chunk;

            if unprocessed_bytes.len() > 0 {
                chunk = [unprocessed_bytes.clone(), chunk].concat();
                unprocessed_bytes.clear();
            }

            let mut flag = false;
            let mut i = 0;
            let mut j = 0;

            for b in chunk.iter() {
                if *b == 34 {
                    flag = !flag;
                }

                if !flag && *b == 0 {
                    break;
                }

                if !flag && *b == 44 || *b == 10 {
                    result[k].push(String::from_utf8(chunk[j..i].to_vec()).unwrap());
                    j = i + 1;
                }

                if !flag && *b == 10 {
                    result.push([].to_vec());
                    k += 1;
                }

                i += 1;
            }

            if j > 0 {
                unprocessed_bytes = [unprocessed_bytes.clone(), chunk[j..i].to_vec()].concat();
            } else {
                unprocessed_bytes = chunk;
            }
        }

        result[k].push(String::from_utf8(unprocessed_bytes.to_vec()).unwrap());
        result
    }
}
