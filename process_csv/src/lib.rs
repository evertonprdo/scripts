pub mod buf_reader;
pub mod chunk_reader;

use std::env;

pub use buf_reader::CsvReader;
pub use chunk_reader::ChunkReader;

pub struct Config {
    file_path: String,
    watermark: Option<usize>,
}
impl Config {
    pub fn build_from(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = args.next().ok_or("Didn't get a file name")?;
        let watermark = env::var("WATERMARK")
            .ok()
            .map(|val| {
                val.parse::<usize>()
                    .map_err(|_| "Failed to parse 'WATERMARK'")
            })
            .transpose()?;

        Ok(Config {
            file_path,
            watermark,
        })
    }
}
