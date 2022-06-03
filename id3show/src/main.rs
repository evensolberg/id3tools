use std::{error::Error, time::Instant};

// Logging
mod build_cli;
use build_cli::build_cli;

mod ape;
mod dsf;
mod flac;
mod mp3;
mod mp4;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Start timing the execution
    let now = Instant::now();

    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = build_cli().get_matches();

    // Set up the logging.
    let logging_config_filename = cli_args
        .value_of("log-config-file")
        .unwrap_or("~/.config/id3tag/id3show-logs.yaml");
    common::build_log(logging_config_filename)?;

    // Initialize counters for total files, skipped and processed.
    let mut files_processed = 0;
    let mut files_skipped = 0;

    let mut filenames = Vec::<&str>::new();
    let mut file_count = 0;

    for filename in cli_args.values_of("files").unwrap() {
        filenames.push(&*filename);
        file_count += 1;
    }

    let show_detail = cli_args.is_present("show-detail");
    let print_summary = cli_args.is_present("print-summary");

    for filename in filenames {
        log::info!("{}", filename);
        let proc_res = match common::get_file_type(filename)? {
            common::FileTypes::Ape => ape::show_metadata(filename, show_detail),
            common::FileTypes::Dsf => dsf::show_metadata(filename, show_detail),
            common::FileTypes::Flac => flac::show_metadata(filename, show_detail),
            common::FileTypes::MP3 => mp3::show_metadata(filename, show_detail),
            common::FileTypes::MP4 => mp4::show_metadata(filename, show_detail),
            common::FileTypes::Unknown => {
                log::info!("  Unknown file type. Skipping.");
                Ok(())
            }
        };

        match proc_res {
            Ok(_) => files_processed += 1,
            Err(err) => {
                log::error!("  Unable to process. Error: {}", err);
                files_skipped += 1;
            }
        }
    }

    // Print summary
    if print_summary {
        log::info!(
            "\nTotal number of files: {}\n  Processed: {}\n  Skipped: {}",
            file_count,
            files_processed,
            files_skipped
        );
        log::info!("Total time: {} ms", now.elapsed().as_millis());
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
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
