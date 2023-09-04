use crate::errors::TockloaderError;
use crate::interfaces::traits::BoardInterface;
use crate::interfaces::SerialInterface;

impl BoardInterface for SerialInterface {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
