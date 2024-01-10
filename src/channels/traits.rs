use crate::errors::TockloaderError;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoardChannel {
    fn open(&mut self) -> Result<(), TockloaderError>;
}

// Other traits go in here
