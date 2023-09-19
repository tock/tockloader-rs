use async_trait::async_trait;

use crate::{
    bootloader::attribute::Attribute, errors::TockloaderError,
    interfaces::traits::BootloaderInterface, interfaces::JLinkInterface,
};

#[async_trait]
impl BootloaderInterface for JLinkInterface {
    async fn enter_bootloader(&mut self) -> Result<bool, TockloaderError> {
        todo!()
    }

    async fn ping(&mut self) -> Result<bool, TockloaderError> {
        todo!()
    }

    async fn sync(&mut self) -> Result<(), TockloaderError> {
        todo!()
    }

    async fn get_attribute(&mut self) -> Result<Attribute, TockloaderError> {
        todo!()
    }
}
