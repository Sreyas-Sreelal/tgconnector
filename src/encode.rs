use encoding::all::UTF_8;
use encoding::{EncoderTrap, Encoding};
use std::error;
use std::str::from_utf8;

pub fn encode_replace(string: &str) -> Result<String, Box<dyn error::Error>> {
    let bytes = UTF_8.encode(string, EncoderTrap::Replace)?;
    let data = from_utf8(&bytes)?;
    Ok(data.to_string())
}
