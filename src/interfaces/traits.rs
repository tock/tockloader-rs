use crate::errors::TockloaderError;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoardInterface {
    fn open(&mut self) -> Result<(), TockloaderError>;
}

#[enum_dispatch]
pub trait BytesReader {
    fn read_range(&self, start: usize, len: usize) -> Result<Vec<u8>, TockloaderError>;
}
