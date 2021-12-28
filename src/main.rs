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

    if config.quiet.unwrap() {
        logbuilder.filter_level(LevelFilter::Off);
    }

    // let show_detail_info = !cli_args.is_present("detail-off");
    if config.dry_run.unwrap() {
        log::info!("Dry-run starting.");
    }

    let mut total_file_count: usize = 0;
    let mut processed_file_count: usize = 0;
    let mut skipped_file_count: usize = 0;

    // create a list of the files to gather
    let file_list = cli_args.values_of("files").unwrap();
    log::debug!("File list: {:?}", file_list);

    if cli_args.is_present("tags") {
        log::debug!("tags = {:?}", cli_args.values_of("tags"));
        if let Some(tags) = cli_args.values_of("tags") {
            log::debug!("tag = {:?}", tags);
        }
    }

    // Read the new tags from the CLI arguments
    let new_tags = args::parse_options(&config, &cli_args)?;
    log::debug!("New tags: {:?}", new_tags);

    for filename in file_list {
        match args::get_extension(filename).as_ref() {
            "flac" => {
                flac::process_flac(filename, &new_tags, &config)?;
                processed_file_count += 1;
            }
            "mp3" => {
                log::debug!("Processing MP3. Cool.");
                mp3::process_mp3(filename, &new_tags, &config)?;
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
    if config.print_summary.unwrap() {
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
