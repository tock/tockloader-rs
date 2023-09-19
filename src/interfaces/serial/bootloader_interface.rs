use super::binary_codec::BinaryCodec;
use crate::{
    bootloader::{attribute::Attribute, codes::*},
    errors::TockloaderError,
    interfaces::traits::BootloaderInterface,
    interfaces::SerialInterface,
    timeout,
};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt, TryFutureExt};
use std::time::Duration;
use tokio_serial::SerialPort;
use tokio_util::codec::Decoder;

#[async_trait]
impl BootloaderInterface for SerialInterface {
    async fn enter_bootloader(&mut self) -> Result<bool, TockloaderError> {
        // These methods are taken from the python version of tockloader
        // bootlaoder_serial.py:518 [_toggle_bootloader_entry_DTR_RTS()]

        if self.stream.is_none() {
            return Err(TockloaderError::StreamClosed);
        }

        // Method 0: We are already in bootloader mode
        if self.bootloader_open().await {
            return Ok(true);
        }

        // Method 1: Change baud rate to 1200 and change back.
        // TODO: When does this work?
        self.stream
            .as_mut()
            .unwrap()
            .set_baud_rate(1200)
            .map_err(TockloaderError::TokioSeriallError)?;
        tokio::time::sleep(Duration::from_millis(1000)).await;

        if self.bootloader_open().await {
            return Ok(true);
        }

        self.stream
            .as_mut()
            .unwrap()
            .set_baud_rate(self.baud_rate)
            .map_err(TockloaderError::TokioSeriallError)?;
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Method 2: DTR & RTS
        // > Use the DTR and RTS lines on UART to reset the chip and assert the
        // > bootloader select pin to enter bootloader mode so that the chip will
        // > start in bootloader mode.
        // - tocklaoder

        // > Reset the SAM4L
        self.stream
            .as_mut()
            .unwrap()
            .write_data_terminal_ready(true)
            .map_err(TockloaderError::TokioSeriallError)?;

        // > Set RTS to make the SAM4L go into bootloader mode
        self.stream
            .as_mut()
            .unwrap()
            .write_request_to_send(true)
            .map_err(TockloaderError::TokioSeriallError)?;

        tokio::time::sleep(Duration::from_millis(100)).await;

        // > Let the SAM4L startup
        self.stream
            .as_mut()
            .unwrap()
            .write_data_terminal_ready(false)
            .map_err(TockloaderError::TokioSeriallError)?;

        // > make sure the bootloader enters bootloader mode
        tokio::time::sleep(Duration::from_millis(500)).await;

        self.stream
            .as_mut()
            .unwrap()
            .write_request_to_send(false)
            .map_err(TockloaderError::TokioSeriallError)?;

        if self.bootloader_open().await {
            return Ok(true);
        }

        Ok(false)
    }

    async fn ping(&mut self) -> Result<bool, TockloaderError> {
        let mut channel = BinaryCodec.framed(self.stream.as_mut().unwrap());

        channel.send([ESCAPE_CHAR, 0x1]).await?;

        if let Ok(response) = timeout!(channel.next()).await {
            if let Some(decoder_result) = response {
                let response = decoder_result?;
                if response == [ESCAPE_CHAR, RESPONSE_PONG] {
                    return Ok(true);
                }
            }

            Ok(false)
        } else {
            // Timeout
            Ok(false)
        }
    }

    async fn sync(&mut self) -> Result<(), TockloaderError> {
        let mut channel = BinaryCodec.framed(self.stream.as_mut().unwrap());

        channel.send([0x00, ESCAPE_CHAR, COMMAND_RESET]).await?;
        Ok(())
    }

    async fn get_attribute(&mut self, index: u8) -> Result<Attribute, TockloaderError> {
        self.sync().await?;

        let mut channel = BinaryCodec.framed(self.stream.as_mut().unwrap());

        channel
            .send([index, ESCAPE_CHAR, COMMAND_GET_ATTRIBUTE])
            .await?;
        if let Some(decoder_result) = timeout!(channel.next()).await? {
            return Attribute::parse_raw(decoder_result?);
        }

        // TODO: Is this the right error to give?
        Err(TockloaderError::BootloaderNotOpen)
    }
}
