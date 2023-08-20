use self::{
    board_interface::BoardInterface, jlink::JLinkInterface, openocd::OpenOCDInterface,
    serial::SerialInterface,
};
use crate::errors::TockloaderError;
use enum_dispatch::enum_dispatch;

pub mod board_interface;
pub mod jlink;
pub mod openocd;
pub mod serial;

#[enum_dispatch(BoardInterface)]
pub enum Interface {
    Serial(SerialInterface),
    OpenOCD(OpenOCDInterface),
    JLink(JLinkInterface),
}
