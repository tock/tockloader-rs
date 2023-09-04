use self::{jlink::JLinkInterface, openocd::OpenOCDInterface, serial::SerialInterface, traits::*};
use crate::errors::TockloaderError;
use enum_dispatch::enum_dispatch;

pub mod jlink;
pub mod openocd;
pub mod serial;
pub mod traits;

#[enum_dispatch(BoardInterface)]
#[enum_dispatch(BytesReader)]
pub enum Interface {
    Serial(SerialInterface),
    OpenOCD(OpenOCDInterface),
    JLink(JLinkInterface),
}
