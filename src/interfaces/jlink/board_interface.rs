use crate::errors::TockloaderError;
use crate::interfaces::traits::BoardInterface;
use crate::interfaces::JLinkInterface;

impl BoardInterface for JLinkInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
