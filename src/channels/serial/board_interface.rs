use crate::channels::traits::BoardChannel;
use crate::channels::SerialChannel;
use crate::errors::TockloaderError;

impl BoardChannel for SerialChannel {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
