use crate::errors::TockloaderError;
use crate::interfaces::traits::BoardInterface;
use crate::interfaces::OpenOCDInterface;

impl BoardInterface for OpenOCDInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
