use self::{jlink::JLinkInterface, openocd::OpenOCDInterface, serial::SerialInterface, traits::*};
use crate::errors::{CLIError, TockloaderError};
use clap::ArgMatches;
use enum_dispatch::enum_dispatch;

pub mod jlink;
pub mod openocd;
pub mod serial;
pub mod traits;

#[enum_dispatch(BoardInterface)]
#[enum_dispatch(VirtualTerminal)]
pub enum Interface {
    Serial(SerialInterface),
    OpenOCD(OpenOCDInterface),
    JLink(JLinkInterface),
}

pub fn build_interface(args: &ArgMatches) -> Result<Interface, TockloaderError> {
    if args.get_flag("serial") as u8 + args.get_flag("jlink") as u8 + args.get_flag("openocd") as u8
        > 1
    {
        return Err(TockloaderError::CLIError(CLIError::MultipleInterfaces));
    }

    if args.get_flag("serial") {
        Ok(SerialInterface::new(args)?.into())
    } else if args.get_flag("jlink") {
        Ok(JLinkInterface::new(args)?.into())
    } else {
        Ok(OpenOCDInterface::new(args)?.into())
    }
}
