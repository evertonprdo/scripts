pub mod helper;
pub mod reader;

use std::env;

pub use helper::CellParser;
pub use reader::CsvReader;
pub use reader::YieldEvent;

const LF: u8 = 10;
const CR: u8 = 13;
const COMMA: u8 = 44;
const QUOTES: u8 = 34;

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
