use crate::errors::TockloaderError;
use crate::interfaces::traits::BoardChannel;
use crate::interfaces::JLinkChannel;

impl BoardChannel for JLinkChannel {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
