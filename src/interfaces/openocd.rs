use super::board_interface::BoardInterface;
use crate::errors::TockloaderError;

pub struct OpenOCDInterface {}

impl BoardInterface for OpenOCDInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
