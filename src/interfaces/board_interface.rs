use crate::errors::TockloaderError;

pub trait BoardInterface {
    fn open(&mut self) -> Result<(), TockloaderError>;
}
