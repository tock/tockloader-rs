use self::{jlink::JLinkChannel, openocd::OpenOCDChannel, serial::SerialChannel, traits::*};
use crate::errors::TockloaderError;
use enum_dispatch::enum_dispatch;

pub mod jlink;
pub mod openocd;
pub mod serial;
pub mod traits;

#[enum_dispatch(BoardChannel)]
// To add other traits, just chaing the enum_dispatch directive:
// #[enum_dispatch(AnotherTrait)]
pub enum Channel {
    Serial(SerialChannel),
    OpenOCD(OpenOCDChannel),
    JLink(JLinkChannel),
}
