mod cli;
use cli::make_cli;

fn main() {
    let matches = make_cli().get_matches();

    if matches.get_flag("debug") {
        println!("Debug mode enabled");
    }

    match matches.subcommand() {
        Some(("listen", sub_matches)) => {
            println!("Got the listen subcommand");
            let default_adr = "NONE".to_string();
            let adr = sub_matches
                .get_one::<String>("app-address")
                .unwrap_or(&default_adr);
            println!("With App Address {adr}");
        }
        // If only the "--debug" flag is set, then this branch is executed
        // Or, more likely at this stage, a subcommand hasn't been implemented yet.
        _ => {
            println!("Could not run the provided subcommand.");
            _ = make_cli().print_help();
        }
    }
}
