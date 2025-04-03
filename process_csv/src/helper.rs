use std::error::Error;

pub fn cell_to_string(line: Vec<u8>) -> Result<String, Box<dyn Error>> {
    String::from_utf8(line.to_vec()).map_err(|e| e.into())
}
