use super::codes::*;
use crate::errors::TockloaderError;
use std::str;

/// TODO: What exactly is an attribute?
#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl Attribute {
    /// Intrepret raw bytes, according to the Tock Format (TODO: Source??)
    pub fn parse_raw(bytes: Vec<u8>) -> Result<Attribute, TockloaderError> {
        // First 2 bytes should be:
        // <ESCAPE CHAR> <RESPONSE_GET_ATTRIBUTE>

        if bytes[0] != ESCAPE_CHAR {
            return Err(TockloaderError::MalformedResponse(format!(
                "Expected attribute to start with ESCAPE_CHAR({}), but got {}",
                ESCAPE_CHAR, bytes[0]
            )));
        }

        if bytes[1] != RESPONSE_GET_ATTRIBUTE {
            return Err(TockloaderError::MalformedResponse(format!(
                "Expected attribute to have second byte RESPONSE_GET_ATTRIBUTE({}), but got {}",
                RESPONSE_GET_ATTRIBUTE, bytes[1]
            )));
        }

        // The next 8 bytes will be the name of the attribute (key) with
        // null bytes as padding

        let key_bytes: Vec<u8> = bytes[2..10]
            .iter()
            .copied()
            .filter(|byte| *byte != 0)
            .collect();
        let key = str::from_utf8(&key_bytes)
            .map_err(|_| {
                TockloaderError::MalformedResponse(format!(
                    "Failed to parse UTF-8 from key: {:?}",
                    key_bytes
                ))
            })?
            .to_string();

        // 9-th byte is the length of the value (without padding).
        let len = bytes[10];
        // 10th+ bytes is the value
        let val_bytes: Vec<u8> = bytes[11..(11 + len as usize)]
            .iter()
            .copied()
            .filter(|byte| *byte != 0)
            .collect();

        let value = str::from_utf8(&val_bytes)
            .map_err(|_| {
                TockloaderError::MalformedResponse(format!(
                    "Failed to parse UTF-8 from value: {:?}",
                    val_bytes
                ))
            })?
            .to_string();

        Ok(Attribute { key, value })
    }
}
