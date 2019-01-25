use encoding::all::WINDOWS_1251;
use encoding::{EncoderTrap, Encoding};
use samp_sdk::amx::{AmxError, AmxResult};

//Just a modification to encode function to replace the chararcter on error instead of giving up entire thing
pub fn encode_replace(string: &str) -> AmxResult<Vec<u8>> {
    WINDOWS_1251
        .encode(string, EncoderTrap::Replace)
        .map_err(|_| AmxError::Format)
}
