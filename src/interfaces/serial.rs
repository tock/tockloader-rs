use super::board_interface::BoardInterface;
use crate::errors::TockloaderError;

pub struct SerialInterface {}

impl BoardInterface for SerialInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
