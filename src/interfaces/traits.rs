use crate::{bootloader::attribute::Attribute, errors::TockloaderError};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait BoardInterface {
    fn open(&mut self) -> Result<(), TockloaderError>;
}

#[async_trait]
#[enum_dispatch]
pub trait VirtualTerminal {
    async fn run_terminal(&mut self) -> Result<(), TockloaderError>;
}

#[async_trait]
#[enum_dispatch]
pub trait BootloaderInterface {
    /// Attempts to enter the bootloader. Does not work on all boards.
    ///
    /// ## Returns
    /// * Ok(true), if the board could be switched to bootloader mode.
    /// * Ok(false), otherwise
    /// * Err([TockloaderError])
    async fn enter_bootloader(&mut self) -> Result<bool, TockloaderError>;

    /// Send a ping to the bootloader. This method is used to determine the
    /// status of the bootloader.
    ///
    /// ## Returns
    /// * Ok(true), if a PONG is received
    /// * Ok(false), otherwise
    /// * Err([TockloaderError])
    async fn ping(&mut self) -> Result<bool, TockloaderError>;

    async fn bootloader_open(&mut self) -> bool {
        match self.ping().await {
            Ok(true) => true,
            Ok(false) | Err(_) => false,
        }
    }

    /// Send a sync message. TODO: Why? When?
    async fn sync(&mut self) -> Result<(), TockloaderError>;

    /// TODO! Description here, what exactly is an attribute?
    async fn get_attribute(&mut self) -> Result<Attribute, TockloaderError>;
}
