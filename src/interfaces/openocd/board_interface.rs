use crate::errors::TockloaderError;
use crate::interfaces::traits::BoardChannel;
use crate::interfaces::OpenOCDChannel;

impl BoardChannel for OpenOCDChannel {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
