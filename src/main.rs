mod bootloader;
mod cli;
mod errors;
mod interfaces;
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
            dbg!(interface.enter_bootloader().await?);
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
