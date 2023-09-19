mod bootloader;
mod cli;
mod errors;
mod interfaces;
use std::io::{stdin, Read};

use cli::make_cli;
use errors::TockloaderError;
use interfaces::{build_interface, traits::*};

#[tokio::main]
async fn main() -> Result<(), TockloaderError> {
    let result = run().await;
    if let Err(e) = &result {
        eprintln!("{}", e);
    }

    result
}

async fn run() -> Result<(), TockloaderError> {
    let matches = make_cli().get_matches();

    if matches.get_flag("debug") {
        println!("Debug mode enabled");
    }

    match matches.subcommand() {
        Some(("listen", sub_matches)) => {
            let mut interface = build_interface(sub_matches)?;
            interface.open()?;
            interface.run_terminal().await?;
        }
        Some(("info", sub_matches)) => {
            let mut interface = build_interface(sub_matches)?;
            interface.open()?;
            if !interface.enter_bootloader().await? {
                println!("Couldn't enter bootloader automatically. Please try entering it manually and press any key...");
                // Read a single byte and discard
                let _ = stdin().read(&mut [0u8]).unwrap();
                if !interface.bootloader_open().await {
                    return Err(TockloaderError::BootloaderNotOpen);
                }
            }
            for i in 0..16 {
                let attribute = interface.get_attribute(i).await?;
                println!("{}: {} = {:?}", i, attribute.key, attribute.value);
            }
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
