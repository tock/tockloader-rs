use core::fmt;

use tokio::task::JoinError;

#[derive(Debug)]
pub enum TockloaderError {
    TokioSeriallError(tokio_serial::Error),
    NoPortAvailable,
    PortClosed,
    CLIError(CLIError),
    IOError(std::io::Error),
    JoinError(JoinError),
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
            TockloaderError::PortClosed => {
                f.write_str("Unexpected error: port closed mid-operation.")
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
