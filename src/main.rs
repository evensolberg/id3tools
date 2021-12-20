use clap::{App, Arg}; // Command line
use std::path::Path;
use std::{error::Error, ffi::OsStr};

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

// Local modules
mod flac;
use crate::flac::*;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = App::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .long_about(clap::crate_description!())
        .arg(
            Arg::with_name("files")
                .value_name("FILE(S)")
                .help("One or more file(s) to process. Wildcards and multiple files (e.g. 2019*.flac 2020*.mp3) are supported.")
                .takes_value(true)
                .multiple(true),
        )
        .arg( // Hidden debug parameter
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(true)
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .takes_value(false)
                .hidden(true),
        )
        .arg( // Don't print any information
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .multiple(false)
                .help("Don't produce any output except errors while working.")
                .takes_value(false)
        )
        .arg( // Stop on error
            Arg::with_name("stop")
                .short("s")
                .long("stop-on-error")
                .multiple(true)
                .help("Stop on error. If this flag isn't set, the application will attempt to continue in case of error.")
                .takes_value(false)
                .hidden(false),
        )
        .arg( // Dry-run
            Arg::with_name("dry-run")
                .short("r")
                .long("dry-run")
                .multiple(false)
                .help("Iterate through the files and produce output without actually processing anything.")
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::with_name("print-summary")
                .short("p")
                .long("print-summary")
                .multiple(false)
                .help("Print summary detail for each session processed.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::with_name("detail-off")
                .short("o")
                .long("detail-off")
                .multiple(false)
                .help("Don't export detailed information about each file processed.")
                .takes_value(false)
        )
        .get_matches();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Turn off logs from the metaflac module
    logbuilder.filter_module("metaflac::block", LevelFilter::Off);

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    let stop_on_error = cli_args.is_present("stop");
    if stop_on_error {
        log::debug!("Stop on error flag set. Will stop if errors occur.");
    } else {
        log::debug!("Stop on error flag not set. Will attempt to continue in case of errors.");
    }

    let show_detail_info = !cli_args.is_present("detail-off");
    let dry_run = cli_args.is_present("dry-run");
    if dry_run {
        log::info!("Dry-run starting.");
    }

    let mut total_file_count: usize = 0;
    let mut processed_file_count: usize = 0;
    let mut skipped_file_count: usize = 0;

    // create a list of the files to gather
    let file_list = cli_args.values_of("files").unwrap();
    log::debug!("File list: {:?}", file_list);

    for filename in file_list {
        match Path::new(&filename)
            .extension()
            .unwrap_or(OsStr::new("unknown"))
            .to_ascii_lowercase()
            .to_str()
            .unwrap()
        {
            "flac" => {
                process_flac(&filename)?;
                processed_file_count += 1;
            }
            "mp3" => {
                log::debug!("Processing MP3. Cool.");
                processed_file_count += 1;
            }
            _ => {
                log::debug!("Processing unknown or other.");
                skipped_file_count += 1;
            }
        }
        total_file_count += 1;
    }

    // Print summary information
    if cli_args.is_present("print-summary") {
        log::info!("Total files examined:        {:5}", total_file_count);
        log::info!("Files processed:             {:5}", processed_file_count);
        log::info!("Files skipped due to errors: {:5}", skipped_file_count);
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
