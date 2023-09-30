pub mod board_interface;
pub mod virtual_terminal;

use clap::ArgMatches;
use tokio_serial::SerialStream;

use crate::errors::TockloaderError;

pub struct SerialInterface {
    port: String,
    baud_rate: u32,
    stream: Option<SerialStream>,
}

impl SerialInterface {
    pub fn new(args: &ArgMatches) -> Result<Self, TockloaderError> {
        // If the user has specified a port, we want to try to use it.
        // Otherwise, we let tokio-serial enumarate all ports and
        // if multiple ports are present, we let the user decide which.
        let port = if let Some(user_port) = args.get_one::<String>("port") {
            user_port.clone()
        } else {
            let available_ports = tokio_serial::available_ports()?;

            if available_ports.is_empty() {
                return Err(TockloaderError::NoPortAvailable);
            } else if available_ports.len() == 1 {
                clean_port_path(available_ports[0].port_name.clone())
            } else {
                // available_ports.len() > 1
                todo!("Make user choose out of multiple available ports")
            }
        };

        let baud_rate = if let Some(baud_rate) = args.get_one::<u32>("baud-rate") {
            *baud_rate
        } else {
            unreachable!("'--baud-rate' should have a default value.")
        };

        Ok(Self {
            port,
            baud_rate,
            stream: None,
        })
    }
}

// When listing available ports, tokio_serial list unix ports like so:
//     /sys/class/tty/ttyACM0
//     /sys/class/tty/<port>
// For some users, tokio_serial fails to open ports using this path scheme.
// This function replaces it with the normal '/dev/<port>' scheme.
// Windows COM ports should not be affected.
fn clean_port_path(port: String) -> String {
    if port.contains("/sys/class/tty/") {
        port.replace("/sys/class/tty/", "/dev/")
    } else {
        port
    }
}
