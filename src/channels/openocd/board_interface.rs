use crate::channels::traits::BoardChannel;
use crate::channels::OpenOCDChannel;
use crate::errors::TockloaderError;

impl BoardChannel for OpenOCDChannel {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
