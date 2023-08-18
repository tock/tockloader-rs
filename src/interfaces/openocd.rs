use crate::errors::TockloaderError;
use super::board_interface::BoardInterface;

pub struct OpenOCDInterface {}

impl BoardInterface for OpenOCDInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
