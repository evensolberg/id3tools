use clap::{App, Arg}; // Command line
use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

// Local modules
mod args;
mod default_values;
mod flac;
use crate::default_values::*;

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
                .multiple(true)
                .required(true),
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
                .multiple(false)
                .help("Stop on error. If this flag isn't set, the application will attempt to continue in case of error.")
                .takes_value(false),
        )
        .arg( // Dry-run
            Arg::with_name("dry-run")
                .short("r")
                .long("dry-run")
                .help("Iterate through the files and produce output without actually processing anything.")
                .multiple(false)
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::with_name("print-summary")
                .short("p")
                .long("print-summary")
                .multiple(false)
                .help("Print summary after all files are processed.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::with_name("detail-off")
                .short("o")
                .long("detail-off")
                .help("Don't display detailed information about each file processed.")
                .multiple(false)
                .takes_value(false)
        )
        .arg( // Config file
            Arg::with_name("config-file")
                .short("c")
                .long("config-file")
                .help("The name of the config file to be read.")
                .long_help("The name of the config file to be read. Note that this is specified WITHOUT the '=', eg. -c myconfig.toml")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
                .default_value("~/.id3tag-config.toml")
                .display_order(1)
        )
        //////////////////////////////////////////////
        // Options
        .arg( // Album artist
            Arg::with_name("album-artist")
                .long("album-artist")
                .visible_alias("aa")
                .help("The name of the album artist. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Album title
            Arg::with_name("album-title")
                .long("album-title")
                .visible_alias("at")
                .help("The title of the album. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Album genre
            Arg::with_name("album-genre")
                .long("album-genre")
                .visible_alias("ag")
                .help("The genre of the album. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Album date
            Arg::with_name("album-date")
                .long("album-date")
                .visible_alias("ad")
                .help("The release date for the album.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Album date
            Arg::with_name("album-composer")
                .long("album-composer")
                .visible_alias("ac")
                .help("The composer(s) for the album. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Disc number
            Arg::with_name("disc-number")
                .long("disc-number")
                .visible_alias("dn")
                .help("The disc number.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Disc total
            Arg::with_name("disc-total")
                .long("disc-total")
                .visible_alias("dt")
                .help("The total number of discs for the album.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Track artist
            Arg::with_name("track-artist")
                .long("track-artist")
                .visible_alias("ta")
                .help("The name of the track artist. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Track title
            Arg::with_name("track-title")
                .long("track-title")
                .visible_alias("tt")
                .help("The title of the track. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Track number
            Arg::with_name("track-number")
                .long("track-number")
                .visible_alias("tn")
                .help("The track number.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Track total
            Arg::with_name("track-total")
                .long("track-total")
                .visible_alias("to")
                .help("The total number of track for the album.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Front cover picture
            Arg::with_name("picture-front")
                .long("picture-front")
                .visible_alias("pf")
                .help("The front cover picture file name.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .arg( // Back cover picture
            Arg::with_name("picture-back")
                .long("picture-back")
                .visible_alias("pb")
                .help("The back cover picture.")
                .takes_value(true)
                .multiple(false)
                .require_equals(false)
        )
        .get_matches();

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

    // Configure behaviour values
    let stop_on_error = args::stop_on_error(&config, &cli_args);
    let print_summary = args::print_summary(&config, &cli_args);
    let quiet = args::quiet(&config, &cli_args);

    config.quiet = Some(quiet);
    config.stop_on_error = Some(stop_on_error);
    config.print_summary = Some(print_summary);
    log::debug!("Working config: {:?}", &config);

    if quiet {
        logbuilder.filter_level(LevelFilter::Off);
    }

    // let show_detail_info = !cli_args.is_present("detail-off");
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

    // Read the new tags from the CLI arguments
    let new_tags = args::parse_tags(&config, &cli_args)?;
    log::debug!("New tags: {:?}", new_tags);

    for filename in file_list {
        match args::get_extension(&filename).as_ref() {
            "flac" => {
                flac::process_flac(&filename)?;
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
    if print_summary {
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
