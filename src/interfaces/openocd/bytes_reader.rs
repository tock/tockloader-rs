use crate::errors::TockloaderError;
use crate::interfaces::traits::BytesReader;
use crate::interfaces::OpenOCDInterface;

impl BytesReader for OpenOCDInterface {
    fn read_range(&self, _start: usize, _len: usize) -> Result<Vec<u8>, TockloaderError> {
        todo!()
    }
}
