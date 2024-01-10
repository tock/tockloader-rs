use crate::channels::traits::BoardChannel;
use crate::channels::JLinkChannel;
use crate::errors::TockloaderError;

impl BoardChannel for JLinkChannel {
    fn open(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }
}
