use clap::ArgMatches;

use crate::errors::TockloaderError;

pub mod board_interface;
pub mod bootloader_interface;
pub mod virtual_terminal;

pub struct OpenOCDInterface {}

impl OpenOCDInterface {
    pub fn new(_args: &ArgMatches) -> Result<Self, TockloaderError> {
        todo!()
    }
}
