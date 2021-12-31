use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

// Local modules
mod args;
mod cli;
mod default_values;
mod flac;
mod mp3;
mod mp4;
mod shared;
use crate::default_values::*;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build_cli();

    // Configure logging
    let mut logbuilder = Builder::new();
    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }
    logbuilder.filter_module("metaflac::block", LevelFilter::Warn);
    logbuilder.target(Target::Stdout).init();

    // Read the config file if asked to
    let mut config = DefaultValues::new();
    if cli_args.is_present("config-file") {
        let config_filename = shellexpand::tilde(
            cli_args
                .value_of("config-file")
                .unwrap_or("~/.id3tag-config.toml"),
        )
        .to_string();
        log::debug!("Config filename: {}", config_filename);
        config = DefaultValues::load_config(&config_filename)?;
        log::debug!("Loaded config: {:?}", &config);
    }

    // Collate config file flags and CLI flags and output the right config
    config.quiet = Some(args::quiet(&config, &cli_args));
    config.stop_on_error = Some(args::stop_on_error(&config, &cli_args));
    config.print_summary = Some(args::print_summary(&config, &cli_args));
    config.detail_off = Some(args::detail_off(&config, &cli_args));
    config.dry_run = Some(args::dry_run(&config, &cli_args));
    log::debug!("Working config: {:?}", &config);

    if config.quiet.unwrap_or(false) {
        logbuilder.filter_level(LevelFilter::Off);
    }

    // let show_detail_info = !cli_args.is_present("detail-off");
    if config.dry_run.unwrap_or(true) {
        log::info!("Dry-run starting.");
    }

    let mut counts = shared::Counts::default();
    log::debug!("counts = {:?}", counts);

    // create a list of the files to gather
    let file_list = cli_args.values_of("files").unwrap_or_default();
    log::debug!("File list: {:?}", file_list);

    if cli_args.is_present("tags") {
        log::debug!("tags = {:?}", cli_args.values_of("tags"));
        if let Some(tags) = cli_args.values_of("tags") {
            log::debug!("tag = {:?}", tags);
        }
    }

    // Read the new tags from the CLI arguments

    for filename in file_list {
        match shared::get_extension(filename).as_ref() {
            "flac" => process_file(
                args::FileType::Flac,
                filename,
                &config,
                &cli_args,
                &mut counts,
            )?, // process flac
            "mp3" => process_file(
                args::FileType::MP3,
                filename,
                &config,
                &cli_args,
                &mut counts,
            )?, // process mp3
            "m4a" | "m4b" | "mp4" | "mp4a" | "mp4b" => process_file(
                args::FileType::MP4,
                filename,
                &config,
                &cli_args,
                &mut counts,
            )?, // process mp4
            _ => {
                if config.stop_on_error.unwrap_or(true) {
                    return Err("Unknown file type. Unable to proceed.".into());
                } else {
                    log::debug!("Unknown file type. Skipping.");
                }
                counts.skipped_file_count += 1;
            } // Unknown
        }
        counts.total_file_count += 1;
    }

    // Print summary information
    if config.print_summary.unwrap_or(false) {
        log::info!("Total files examined:        {:5}", counts.total_file_count);
        log::info!(
            "Files processed:             {:5}",
            counts.processed_file_count
        );
        log::info!(
            "Files skipped due to errors: {:5}",
            counts.skipped_file_count
        );
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

/// Performs the actual file processing
///
/// Parameters:
///
/// - `file_type: args::FileType` -- the type of file to process (`Flac`, `MP3` or `MP4`)
/// - `filename: &str` -- the name of the file
/// - `config: &DefaultValuess` -- The default config values to use (stop on error, etc)
/// - `cli_args: &clap::ArgMatches` -- The config values and options supplied from the CLI
/// - `counts: &mut shared::Counts` -- A struct for various file counters (skipped, processed, total)
///
/// Returns:
///
/// - `Ok()` if everything goes well.
/// - `Box<dyn Error>` if we run into problems
fn process_file(
    file_type: args::FileType,
    filename: &str,
    config: &DefaultValues,
    cli_args: &clap::ArgMatches,
    counts: &mut shared::Counts,
) -> Result<(), Box<dyn Error>> {
    match file_type {
        args::FileType::Flac => log::debug!("Processing FLAC."),
        args::FileType::MP3 => log::debug!("Processing MP3."),
        args::FileType::MP4 => log::debug!("Processing MP4."),
    }

    let new_tags_result = args::parse_options(&filename, file_type, config, cli_args);
    log::debug!("new_tags_result: {:?}", new_tags_result);
    let new_tags;
    match new_tags_result {
        Ok(res) => {
            new_tags = res;
            log::debug!("New tags: {:?}", new_tags);

            log::debug!("Processing file.");
            let proc_res = match file_type {
                args::FileType::Flac => flac::process_flac(filename, &new_tags, config),
                args::FileType::MP3 => mp3::process_mp3(filename, &new_tags, config),
                args::FileType::MP4 => mp4::process_mp4(filename, &new_tags, config),
            };

            match proc_res {
                Ok(_) => counts.processed_file_count += 1,
                Err(err) => {
                    if config.stop_on_error.unwrap_or(true) {
                        return Err(format!(
                            "Unable to process {}. Error: {}",
                            filename,
                            err.to_string()
                        )
                        .into());
                    } else {
                        log::error!("Unable to process {}. Error: {}", filename, err.to_string());
                        counts.skipped_file_count += 1;
                    }
                }
            } // match flag::process_flac
        } // Ok(_)
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!(
                    "Unable to parse tags for {}. Error: {}",
                    filename,
                    err.to_string()
                )
                .into());
            } else {
                log::error!(
                    "Unable to parse tags for {}. Error: {}",
                    filename,
                    err.to_string()
                );
                counts.skipped_file_count += 1;
            }
        } // Err(err)
    } // match new_tags_result

    // return safely
    Ok(())
}
