//! A simple application to set metadata in APE, FLAC, MP3 and MP4 files.
//! This application is primarily meant to be used for bulk updates.
//! While updating individual tags (such as track names) is supported, it's not
//! easy to do this using this application.
//!
//! In the future this application will endeavour to support reading tags from CSV files,
//! moving and renaming files based on tags, etc.
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::error::Error;
use std::time::Instant;

// Logging
// use log::LevelFilter;

// Local modules
mod default_values;
mod formats;
mod rename_file;

use common::thousand_separated;
use rayon::prelude::*;

use crate::default_values::DefaultValues;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Start timing the execution
    let now = Instant::now();

    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = common::build_cli(env!("CARGO_PKG_VERSION")).get_matches();

    // Build the config -- read the CLI arguments and the config file if one is provided.
    let config = DefaultValues::build_config(&cli_args)?;
    log::debug!("config = {:?}", config);

    // Configure logging
    let logging_config_filename = get_logging_config_filename(&cli_args, &config);
    common::build_logger(&logging_config_filename)?;

    // let show_detail_info = !cli_args.is_present("detail-off");
    if config.dry_run.unwrap_or(true) {
        log::info!("Dry-run starting.");
    }

    // Initialize counters for total files, skipped and processed.
    // let counts = Arc::new(Mutex::new(shared::Counts::default()));

    // create a list of the files to gather
    for file in cli_args.values_of("files").unwrap_or_default() {
        log::debug!("file: {:?}", file);
    }

    if cli_args.is_present("tags") {
        for value in cli_args.values_of("tags").unwrap_or_default() {
            log::debug!("tag = {:?}", value);
        }
    }

    let mut filenames = Vec::<&str>::new();
    let mut file_count = 0;

    for filename in cli_args.values_of("files").unwrap_or_default() {
        filenames.push(filename);
        file_count += 1;
    }

    // Process things - uses single threaded mode if we can't figure it out. Better safe than sorry.
    let res_vec: Vec<bool> = if config.single_thread.unwrap_or(true) {
        filenames
            .iter()
            .map(|&filename| process_file(filename, &cli_args, &config))
            .collect()
    } else {
        filenames
            .par_iter()
            .map(|&filename| process_file(filename, &cli_args, &config))
            .collect()
    };

    log::debug!("res_vec = {:?}", res_vec);

    // Print summary information
    if config.print_summary.unwrap_or(false) {
        let mut processed = 0;
        let mut skipped = 0;

        for res_ok in res_vec {
            if res_ok {
                processed += 1;
            } else {
                skipped += 1;
            }
        }

        log::info!(
            "Files examined:              {:>5}",
            thousand_separated(file_count)
        );
        log::info!(
            "   Processed:                {:>5}",
            thousand_separated(processed)
        );
        log::info!(
            "   Skipped due to errors:    {:>5}",
            thousand_separated(skipped)
        );
        log::info!(
            "Time elapsed:            {:>9} ms",
            thousand_separated(now.elapsed().as_millis() as usize)
        );
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked. This in turn calls the `run` function.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}

/// Processes the file based on the filename
fn process_file(
    filename: &str,
    cli_args: &clap::ArgMatches,
    config: &default_values::DefaultValues,
) -> bool {
    let file_type = common::get_file_type(filename).unwrap_or(common::FileTypes::Unknown);

    let res = formats::process_file(file_type, filename, config, cli_args).unwrap_or(false);

    log::debug!("process_file result = {}", res);

    // return safely
    res
}

/// Gets the file name for the logging config.
///
/// The function will first check if the `log-config-file` flag has been set.<br>
///   - If a file name has been provided along with the flag, this will be used.<br>
///   - If no file name was supplied along with the flag, the default is used.<br>
///
/// If no flag is set, we will check the program's configuration.<br>
///   - If something has been provided in the config file<br>
///       - If the config file entry for some reason can't be read, we use the default.<br>
///   - If nothing has been provided, we use the default.<br>
///
///
fn get_logging_config_filename(
    cli_args: &clap::ArgMatches,
    config: &default_values::DefaultValues,
) -> String {
    let default = "~/.config/id3tag/logs.yaml".to_string();

    if cli_args.is_present("log-config-file") {
        cli_args
            .value_of("log-config-file")
            .unwrap_or(&default)
            .to_string()
    } else if config.log_config_file.is_some() {
        config.log_config_file.as_ref().unwrap_or(&default).clone()
    } else {
        default
    }
}
