use crate::errors::TockloaderError;
use super::board_interface::BoardInterface;

pub struct JLinkInterface {}

impl BoardInterface for JLinkInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
