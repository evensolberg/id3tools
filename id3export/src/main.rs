//! Exports data from an ID3 tag to file.

mod build_cli;
mod tracks;

use std::error::Error;

use crate::tracks::Reader;
use build_cli::build_cli;
use env_logger::{Builder, Target};
use log::LevelFilter;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = build_cli().get_matches();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    match cli_args.get_count("debug") {
        0 => {
            logbuilder.filter_level(LevelFilter::Info);
        }
        1 => {
            logbuilder.filter_level(LevelFilter::Debug);
        }
        _ => {
            logbuilder.filter_level(LevelFilter::Trace);
        }
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    let show_detail = cli_args.get_flag("show-detail");
    let print_summary = cli_args.get_flag("print-summary");

    // Initialize counters for total files, skipped and processed.
    let mut files_processed = 0;
    let mut files_skipped = 0;
    let mut file_count = 0;

    let files = cli_args.get_many::<String>("files").unwrap_or_default();
    log::debug!("Files: {files:?}");

    let mut tracks = Vec::<tracks::Track>::new();

    for filename in files {
        log::debug!("Processing file: {filename}");
        for track in glob::glob(filename)? {
            let track_name = track?.to_string_lossy().to_string();

            if show_detail {
                log::info!("{track_name}");
            }

            log::debug!("Track: {track_name}");
            let mut track_info = tracks::Track::from_path(track_name);

            let res = track_info.read();
            if let Err(err) = res {
                log::error!("Error reading track: {err}");
                files_skipped += 1;
                continue;
            } else {
                files_processed += 1;
                log::debug!("Track info: {track_info:?}");
                tracks.push(track_info);
            }

            file_count += 1;
        }
    }

    log::debug!("Tracks: {tracks:?}");

    if print_summary {
        println!("Files processed: {files_processed}");
        println!("Files skipped: {files_skipped}");
        println!("Total files: {file_count}");
    }

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
