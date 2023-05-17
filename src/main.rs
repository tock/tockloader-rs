mod cli;
mod serial_interface;
use cli::make_cli;
use serial_interface::open_first_available_port;

use crate::serial_interface::{open_port, run_terminal};

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {
    let matches = make_cli().get_matches();

    if matches.get_flag("debug") {
        println!("Debug mode enabled");
    }

    match matches.subcommand() {
        Some(("listen", sub_matches)) => {
            let baud_rate = *sub_matches.get_one::<u32>("baud-rate").unwrap();
            let stream = match sub_matches.get_one::<String>("port") {
                Some(port) => open_port(port.to_string(), baud_rate)?,
                None => open_first_available_port(baud_rate)?,
            };

            run_terminal(stream).await;
        }
        // If only the "--debug" flag is set, then this branch is executed
        // Or, more likely at this stage, a subcommand hasn't been implemented yet.
        _ => {
            println!("Could not run the provided subcommand.");
            _ = make_cli().print_help();
        }
    }

    Ok(())
}
