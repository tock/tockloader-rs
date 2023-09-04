use crate::errors::TockloaderError;
use crate::interfaces::traits::BytesReader;
use crate::interfaces::SerialInterface;

impl BytesReader for SerialInterface {
    fn read_range(&self, _start: usize, _len: usize) -> Result<Vec<u8>, TockloaderError> {
        todo!()
    }
}
