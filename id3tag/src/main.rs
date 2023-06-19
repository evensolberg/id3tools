//! A simple application to set metadata in APE, FLAC, MP3 and MP4 files.
//! This application is primarily meant to be used for bulk updates.
//! While updating individual tags (such as track names) is supported, it's not
//! easy to do this using this application.
//!
//! In the future this application will endeavour to support reading tags from CSV files,
//! moving and renaming files based on tags, etc.
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use clap::ArgMatches;
use std::error::Error;
use std::time::Instant;

// Local modules
mod default_values;
mod formats;
mod rename_file;

use crate::default_values::DefaultValues;
use common::file_rename_pattern_not_ok;
use human_duration::human_duration;
use rayon::prelude::*;
use thousands::Separable;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
#[allow(clippy::cast_precision_loss)] // for  `let el = elapsed as f64 / 1000.0;`
fn run() -> Result<(), Box<dyn Error>> {
    // Start timing the execution
    let now = Instant::now();

    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli = common::build_cli(env!("CARGO_PKG_VERSION")).get_matches();

    let binding = String::new();
    let pattern = cli.get_one::<String>("rename-file").unwrap_or(&binding);
    if cli.contains_id("rename-file") && file_rename_pattern_not_ok(pattern) {
        return Err(
            format!("File rename pattern {pattern} likely won't create unique files.").into(),
        );
    }

    // Build the config -- read the CLI arguments and the config file if one is provided.
    let config = DefaultValues::build_config(&cli)?;

    // Configure logging
    let logging_config_filename = get_logging_config_filename(&cli, &config);
    common::build_logger(&logging_config_filename)?;

    log::debug!("config = {config:?}");

    // let show_detail_info = !cli_args.is_present("detail-off");
    if config.dry_run.unwrap_or(true) {
        log::info!("Dry-run starting.");
    }

    // Initialize counters for total files, skipped and processed.
    // let counts = Arc::new(Mutex::new(shared::Counts::default()));

    // create a list of the files to gather
    for file in cli.get_many::<String>("files").unwrap_or_default() {
        log::trace!("file: {file:?}");
    }

    let mut filenames = Vec::<&str>::new();
    let mut file_count = 0;

    for filename in cli.get_many::<String>("files").unwrap_or_default() {
        filenames.push(filename);
        file_count += 1;
    }

    // Process things - uses single threaded mode if we can't figure it out. Better safe than sorry.
    let res_vec: Vec<bool> = if config.single_thread.unwrap_or(true) {
        filenames
            .iter()
            .map(|&filename| process_file(filename, &cli, &config))
            .collect()
    } else {
        filenames
            .par_iter()
            .map(|&filename| process_file(filename, &cli, &config))
            .collect()
    };

    log::trace!("res_vec = {res_vec:?}");

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

        log::info!("   ");
        log::info!(
            "Files examined:              {:>5}",
            file_count.separate_with_commas()
        );
        log::info!(
            "   Processed:                {:>5}",
            processed.separate_with_commas()
        );
        log::info!(
            "   Skipped due to errors:    {:>5}",
            skipped.separate_with_commas()
        );
        let elapsed = now.elapsed();
        log::debug!("elapsed = {elapsed:?}");
        log::info!("Time elapsed:{:>21}", human_duration(&elapsed));
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
fn process_file(filename: &str, cli_args: &ArgMatches, config: &DefaultValues) -> bool {
    log::debug!("----------- NEW FILE ---------");
    log::debug!("process_file::filename = {filename}");

    let file_type = common::get_file_type(filename).unwrap_or(common::FileTypes::Unknown);

    let res = formats::process_file(file_type, filename, config, cli_args).unwrap_or(false);

    log::debug!("process_file result = {res}");

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

    if cli_args.contains_id("log-config-file") {
        cli_args
            .get_one::<String>("log-config-file")
            .unwrap_or(&default)
            .to_string()
    } else if config.log_config_file.is_some() {
        config.log_config_file.as_ref().unwrap_or(&default).clone()
    } else {
        default
    }
}
