use async_trait::async_trait;

use crate::{
    bootloader::attribute::Attribute, errors::TockloaderError,
    interfaces::traits::BootloaderInterface, interfaces::OpenOCDInterface,
};

#[async_trait]
impl BootloaderInterface for OpenOCDInterface {
    async fn enter_bootloader(&mut self) -> Result<bool, TockloaderError> {
        todo!()
    }

    async fn ping(&mut self) -> Result<bool, TockloaderError> {
        todo!()
    }

    async fn sync(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }

    async fn get_attribute(&mut self, _index: u8) -> Result<Attribute, TockloaderError> {
        todo!()
    }
}
