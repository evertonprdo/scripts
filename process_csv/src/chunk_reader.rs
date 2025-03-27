use std::{fs::File, io::Read};

pub struct ChunkReader {
    file: File,
    chunk: Vec<u8>,
}
impl ChunkReader {
    pub fn from(path: &str, watermark: Option<usize>) -> Self {
        let file = File::open(path).unwrap();
        let watermark = watermark.unwrap_or(1024 * 8); // 8KB

        ChunkReader {
            file,
            chunk: vec![0; watermark],
        }
    }

    fn read_chunk(&mut self) -> Option<Vec<u8>> {
        let n = match self.file.read(&mut self.chunk) {
            Ok(n) => {
                if n == 0 {
                    return None;
                }
                n
            }
            Err(_) => return None,
        };

        Some(self.chunk[..n].to_vec())
    }
}

// Consumes a CSV File at once and receives a on_new_line callback for on new lines
impl ChunkReader {
    pub fn run<F>(&mut self, on_new_line: F)
    where
        F: Fn(Vec<String>),
    {
        let mut result: Vec<String> = Vec::new();
        let mut unprocessed_bytes: Vec<u8> = Vec::new();

        while let Some(chunk) = self.read_chunk() {
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

                if !flag && *b == 44 || *b == 10 {
                    result.push(String::from_utf8(chunk[j..i].to_vec()).unwrap());
                    j = i + 1;
                }

                if !flag && *b == 10 {
                    on_new_line(result.clone());
                    result.clear();
                }

                i += 1;
            }

            if j > 0 {
                unprocessed_bytes = [unprocessed_bytes.clone(), chunk[j..].to_vec()].concat();
            } else {
                unprocessed_bytes = chunk;
            }
        }

        result.push(String::from_utf8(unprocessed_bytes.to_vec()).unwrap());
        on_new_line(result);
    }
}
