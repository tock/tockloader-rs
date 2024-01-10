use crate::errors::TockloaderError;
use crate::interfaces::traits::BoardChannel;
use crate::interfaces::SerialChannel;

impl BoardChannel for SerialChannel {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
