//! Exports data from an ID3 tag to file.

mod build_cli;
use std::error::Error;

use build_cli::build_cli;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = build_cli().get_matches();

    // Set up the logging.
    let blank = String::new();
    let logging_config_filename = cli_args
        .get_one::<String>("log-config-file")
        .unwrap_or(&blank);
    common::build_logger(logging_config_filename)?;

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(()) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
