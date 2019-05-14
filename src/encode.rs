use encoding::all::WINDOWS_1251;
use encoding::{EncoderTrap, Encoding};
use std::str::from_utf8;

pub fn encode_replace(string: &str) -> Result<String, ()> {
    match WINDOWS_1251.encode(string, EncoderTrap::Replace) {
        Ok(bytes) => match from_utf8(&bytes) {
            Ok(data) => Ok(String::from(data)),
            Err(_) => Err(()),
        },
        Err(_) => Err(()),
    }
}
