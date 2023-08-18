use super::board_interface::BoardInterface;
use crate::errors::TockloaderError;

pub struct JLinkInterface {}

impl BoardInterface for JLinkInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
