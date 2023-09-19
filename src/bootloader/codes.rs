#![allow(unused)]
// "This was chosen as it is infrequent in .bin files" - immesys
pub const ESCAPE_CHAR: u8 = 0xFC;

// Commands from this tool to the bootloader.   (tockloader)
// The "X" commands are for external flash.     (tockloader)
pub const COMMAND_PING: u8 = 0x01;
pub const COMMAND_INFO: u8 = 0x03;
pub const COMMAND_ID: u8 = 0x04;
pub const COMMAND_RESET: u8 = 0x05;
pub const COMMAND_ERASE_PAGE: u8 = 0x06;
pub const COMMAND_WRITE_PAGE: u8 = 0x07;
pub const COMMAND_XEBLOCK: u8 = 0x08;
pub const COMMAND_XWPAGE: u8 = 0x09;
pub const COMMAND_CRCRX: u8 = 0x10;
pub const COMMAND_READ_RANGE: u8 = 0x11;
pub const COMMAND_XRRANGE: u8 = 0x12;
pub const COMMAND_SET_ATTRIBUTE: u8 = 0x13;
pub const COMMAND_GET_ATTRIBUTE: u8 = 0x14;
pub const COMMAND_CRC_INTERNAL_FLASH: u8 = 0x15;
pub const COMMAND_CRCEF: u8 = 0x16;
pub const COMMAND_XEPAGE: u8 = 0x17;
pub const COMMAND_XFINIT: u8 = 0x18;
pub const COMMAND_CLKOUT: u8 = 0x19;
pub const COMMAND_WUSER: u8 = 0x20;
pub const COMMAND_CHANGE_BAUD_RATE: u8 = 0x21;
pub const COMMAND_EXIT: u8 = 0x22;
pub const COMMAND_SET_START_ADDRESS: u8 = 0x23;

// Responses from the bootloader.   (tockloader)
pub const RESPONSE_OVERFLOW: u8 = 0x10;
pub const RESPONSE_PONG: u8 = 0x11;
pub const RESPONSE_BADADDR: u8 = 0x12;
pub const RESPONSE_INTERROR: u8 = 0x13;
pub const RESPONSE_BADARGS: u8 = 0x14;
pub const RESPONSE_OK: u8 = 0x15;
pub const RESPONSE_UNKNOWN: u8 = 0x16;
pub const RESPONSE_XFTIMEOUT: u8 = 0x17;
pub const RESPONSE_XFEPE: u8 = 0x18;
pub const RESPONSE_CRCRX: u8 = 0x19;
pub const RESPONSE_READ_RANGE: u8 = 0x20;
pub const RESPONSE_XRRANGE: u8 = 0x21;
pub const RESPONSE_GET_ATTRIBUTE: u8 = 0x22;
pub const RESPONSE_CRC_INTERNAL_FLASH: u8 = 0x23;
pub const RESPONSE_CRCXF: u8 = 0x24;
pub const RESPONSE_INFO: u8 = 0x25;
pub const RESPONSE_CHANGE_BAUD_FAIL: u8 = 0x26;

pub fn escape(source: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::with_capacity(source.len());
    for byte in source {
        result.push(byte);
        // Escape the escape char
        if byte == ESCAPE_CHAR {
            result.push(ESCAPE_CHAR)
        }
    }

    result
}

pub fn deescape(source: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::with_capacity(source.len());

    if !source.is_empty() {
        result.push(source[0]);
    }

    for i in 1..source.len() {
        if source[i] == ESCAPE_CHAR && source[i - 1] == ESCAPE_CHAR {
            // The previous char was already pushed, so we can skip
            // pushing this one
            continue;
        } else {
            result.push(source[i]);
        }
    }

    result
}
