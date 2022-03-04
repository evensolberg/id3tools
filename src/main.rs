//! A simple application to set metadata in APE, FLAC, MP3 and MP4 files.
//! This application is primarily meant to be used for bulk updates.
//! While updating individual tags (such as track names) is supported, it's not
//! easy to do this using this application.
//!
//! In the future this application will endeavour to support reading tags from CSV files,
//! moving and renaming files based on tags, etc.

use std::{
    error::Error,
    sync::{Arc, Mutex},
};

// Logging
// use log::LevelFilter;

// Local modules
mod cli;
mod default_values;
mod formats;
mod rename_file;
mod shared;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::default_values::*;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build_cli();

    // Build the config -- read the CLI arguments and the config file if one is provided.
    let config = DefaultValues::build_config(&cli_args)?;
    log::debug!("config = {:?}", config);

    // Configure logging
    let mut _logs = shared::build_log(&cli_args, &config)?;

    // let show_detail_info = !cli_args.is_present("detail-off");
    if config.dry_run.unwrap_or(true) {
        log::info!("Dry-run starting.");
    }

    // Initialize counters for total files, skipped and processed.
    let counts = Arc::new(Mutex::new(shared::Counts::default()));

    // create a list of the files to gather
    for file in cli_args.values_of("files").unwrap() {
        log::debug!("file: {:?}", file);
    }

    if cli_args.is_present("tags") {
        for value in cli_args.values_of("tags").unwrap() {
            log::debug!("tag = {:?}", value);
        }
    }

    let mut filenames = Vec::<&str>::new();
    let mut file_count = 0;

    for filename in cli_args.values_of("files").unwrap() {
        filenames.push(filename.clone());
        file_count += 1;
    }

    // Process things
    let res_vec: Vec<bool> = filenames
        .par_iter()
        .map(|&filename| process_file(filename, &cli_args, &config, &counts).unwrap_or(false))
        .collect();

    let skipped = res_vec.iter().filter(|_| false).count();
    let processed = res_vec.iter().filter(|_| true).count();

    // Print summary information
    if config.print_summary.unwrap_or(false) {
        log::info!("Total files examined:        {:5}", file_count);
        log::info!("Files processed:             {:5}", processed);
        log::info!("Files skipped due to errors: {:5}", skipped);
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace("\"", ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}

/// Processes the file based on the filename
fn process_file(
    filename: &str,
    cli_args: &clap::ArgMatches,
    config: &default_values::DefaultValues,
    counts: &Arc<Mutex<shared::Counts>>,
) -> Result<bool, Box<dyn Error>> {
    let file_type;
    match shared::get_extension(filename).as_ref() {
        "ape" => file_type = formats::FileTypes::Ape,
        "flac" => file_type = formats::FileTypes::Flac, // process flac
        "mp3" => file_type = formats::FileTypes::MP3,
        "m4a" | "m4b" | "mp4" | "mp4a" | "mp4b" => file_type = formats::FileTypes::MP4,
        _ => {
            if config.stop_on_error.unwrap_or(true) {
                return Err("Unknown file type. Unable to proceed.".into());
            } else {
                log::debug!("Unknown file type. Skipping.");
                file_type = formats::FileTypes::Unknown;
            }
        } // Unknown
    }
    let res = match formats::process_file(
        file_type,
        filename,
        config,
        &cli_args,
        &mut counts.lock().unwrap(),
    ) {
        Ok(_) => true,
        Err(_) => false,
    };

    // return safely
    Ok(res)
}
