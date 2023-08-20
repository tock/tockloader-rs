use crate::errors::TockloaderError;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoardInterface {
    fn open(&mut self) -> Result<(), TockloaderError>;
}
