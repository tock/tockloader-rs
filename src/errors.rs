use core::fmt;

use tokio::task::JoinError;

#[derive(Debug)]
pub enum TockloaderError {
    TokioSeriallError(tokio_serial::Error),
    NoPortAvailable,
    CLIError(CLIError),
    IOError(std::io::Error),
    JoinError(JoinError),
    StreamClosed,
    Timeout,
    BootloaderNotOpen,
    MalformedResponse(String),
}

#[derive(Debug)]
pub enum CLIError {
    MultipleInterfaces,
}

impl fmt::Display for TockloaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TockloaderError::TokioSeriallError(inner) => {
                inner.fmt(f)
            }
            TockloaderError::NoPortAvailable => {
                f.write_str("Tockloader has failed to find any open ports. If your device is plugged in, you can manually specify it using the '--port <path>' argument.")
            },
            TockloaderError::CLIError(inner) => {
                inner.fmt(f)
            }
            TockloaderError::IOError(inner) => {
                inner.fmt(f)
            },
            TockloaderError::JoinError(inner) => {
                inner.fmt(f)
            },
            TockloaderError::StreamClosed => {
                f.write_str("The serial stream unexpectedly closed.")
            },
            TockloaderError::Timeout => {
                f.write_str("The operation timed out. Check if the board is still connected.")
            },
            TockloaderError::BootloaderNotOpen => {
                f.write_str("The bootloader wouldn't respond. Try again.")
            },
            TockloaderError::MalformedResponse(explanation) => {
                f.write_str(format!("Received corrupted or unexpected response. {}", *explanation).as_str())
            },
        }
    }
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLIError::MultipleInterfaces => {
                f.write_str("At most one of the following tranport interfaces may be used: '--serial', '--openocd', '-jlink'")
            },
        }
    }
}

impl From<std::io::Error> for TockloaderError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl std::error::Error for TockloaderError {}
impl std::error::Error for CLIError {}
