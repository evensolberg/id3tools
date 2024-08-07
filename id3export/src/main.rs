//! Exports data from an ID3 tag to file.

mod build_cli;
mod stats;
mod tracks;

use std::error::Error;

use crate::{stats::calc_avg, tracks::Reader};
use build_cli::build_cli;
use clap::parser::ValueSource;
use env_logger::{Builder, Target};
use log::LevelFilter;
use stats::update_stats;

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
    let write_detail = cli_args.value_source("csv-file") == Some(ValueSource::CommandLine);
    let write_summary = cli_args.value_source("summary-file") == Some(ValueSource::CommandLine);

    // Initialize counters for total files, skipped and processed.
    let mut files_processed = 0;
    let mut files_skipped = 0;
    let mut file_count = 0;
    let mut stats = stats::StatsMap::new();

    let mut filenames = Vec::<&str>::new();
    for filename in cli_args.get_many::<String>("files").unwrap_or_default() {
        filenames.push(filename);
    }
    log::debug!("Files: {filenames:?}");

    let mut tracks = Vec::<tracks::Track>::new();
    for filename in filenames {
        log::debug!("Processing file: {filename}");

        if show_detail {
            log::info!("{filename}");
        }

        log::debug!("Track: {filename}");
        let mut track_info = tracks::Track::from_path(filename.to_owned());

        let res = track_info.read();

        if let Err(err) = res {
            log::error!("Error reading track: {err}");
            files_skipped += 1;
            continue;
        } else {
            files_processed += 1;
            log::debug!("Track info: {track_info:?}");

            update_stats(
                &mut stats,
                &track_info.file_format.unwrap_or_default().to_string(),
                track_info.duration_ms.unwrap_or_default(),
                track_info.file_size.unwrap_or_default(),
            );

            tracks.push(track_info);
        }

        file_count += 1;
    }
    log::trace!("Tracks: {tracks:?}");

    if write_detail {
        let default_name = String::from("details.csv");
        let csv_file = cli_args
            .get_one::<String>("csv-file")
            .unwrap_or(&default_name);
        write_csv(csv_file, tracks)?;
    }

    calc_avg(&mut stats);

    if write_summary {
        let default_name = String::from("summary.csv");
        let summary_file = cli_args
            .get_one::<String>("summary-file")
            .unwrap_or(&default_name);
        stats::export_summary_csv(&stats, summary_file)?;
    }

    if print_summary {
        println!("Total files     : {file_count:5}");
        println!("Files processed : {files_processed:5}");
        println!("Files skipped   : {files_skipped:5}\n");
        stats::print_stats(&stats);
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

fn write_csv(filename: &str, tracks: Vec<tracks::Track>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new().from_path(filename)?;

    for track in tracks {
        log::debug!("Writing track: {track:?}");
        wtr.serialize(track)?;
        // Since most fields can have more than one entry, we need to handle them separately.
    }

    wtr.flush()?;
    Ok(())
}
