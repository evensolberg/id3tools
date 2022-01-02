// Read default values from config file
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

/// The default values for the flags and options.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DefaultValues {
    /// Flag: Do not output detail about each item processed.
    pub detail_off: Option<bool>,

    /// Flag: Print a summary of how many files were processed, skipped, etc.
    pub print_summary: Option<bool>,

    /// Flag: Only output warnings and errors.
    pub quiet: Option<bool>,

    /// Flag: Stop immediately if an error occurs, otherwise continue.
    pub stop_on_error: Option<bool>,

    /// Flag: Don't actually write any changes.
    pub dry_run: Option<bool>,

    // Options //
    /// The default album artist.
    pub album_artist: Option<String>,

    /// The default name on which the album artist is sorted. Example: Artist is "Alicia Keys", but the artist_sort may be "Keys, Alicia".
    pub album_artist_sort: Option<String>,

    /// Album title.
    pub album_title: Option<String>,

    /// Album title sort.
    pub album_title_sort: Option<String>,

    /// Default value for the disc number, usually 1.
    pub disc_number: Option<u16>,

    /// The total number of discs that comprise the album, usually 1.
    pub disc_total: Option<u16>,

    /// Default value for the track's artist.
    pub track_artist: Option<String>,

    /// Default value for the track's artist sort.
    pub track_artist_sort: Option<String>,

    /// Default value for the track's title.
    pub track_title: Option<String>,

    /// Default value for the track's title sort.
    pub track_title_sort: Option<String>,

    /// Default value for the track number, usually not set to a default value.
    pub track_number: Option<u16>,

    /// Default value for the total number of tracks.
    pub track_total: Option<u16>,

    /// Default value for the track's genre.
    pub track_genre: Option<String>,

    /// Default numerical value for the track's genre.
    /// Ref: <https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D>
    pub track_genre_number: Option<u16>,

    /// Default value for the track's composer(s).
    pub track_composer: Option<String>,

    /// Default value for the track's composer(s).
    pub track_composer_sort: Option<String>,

    /// Default value for the track's composer(s).
    pub track_date: Option<String>,

    /// Default value for the track's comments.
    pub track_comments: Option<String>,

    /// Default value for the albums front cover.
    pub picture_front: Option<String>,

    /// Default value for the album's back cover.
    pub picture_back: Option<String>,
}

impl DefaultValues {
    /// Initializes a new, empty set of DefaultValues. All values are set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a config based on CLI arguments
    pub fn build_config(cli_args: &clap::ArgMatches) -> Result<Self, Box<dyn Error>> {
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
        config.quiet = Some(quiet(&config, cli_args));
        config.stop_on_error = Some(stop_on_error(&config, cli_args));
        config.print_summary = Some(print_summary(&config, cli_args));
        config.detail_off = Some(detail_off(&config, cli_args));
        config.dry_run = Some(dry_run(&config, cli_args));
        log::debug!("Working config: {:?}", &config);

        Ok(config)
    }

    /// Loads the config from the supplied TOML file.
    fn load_config(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut config_toml = String::new();

        let mut file = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => {
                return Err(format!("Config file {} not found.", filename).into());
            }
        };

        file.read_to_string(&mut config_toml)?;
        let config = match toml::from_str(&config_toml) {
            Ok(config) => config,
            Err(err) => {
                log::warn!(
                    "Unable to read config from {}. Using defaults. Error message: {}",
                    filename,
                    err.to_string()
                );
                Self::new()
            }
        };
        log::debug!("{:?}", config);

        Ok(config)
    } // pub fn load_config
} // impl DefaultValues

// Housekeeping functions to check which flags have been set, either here or in the config file.

/// Check if the stop-on-error flag has been set, either in the config file
/// or via the CLI.
fn stop_on_error(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.stop_on_error {
            return_value = cfg;
        }
    }

    if args.is_present("stop-on-error") {
        return_value = true;
    }

    if return_value {
        log::debug!("Stop on error flag set. Will stop if errors occur.");
    } else {
        log::debug!("Stop on error flag not set. Will attempt to continue in case of errors.");
    }

    // return the value
    return_value
}

/// Check if the print-summary flag has been set, either in the config file
/// or via the CLI.
fn print_summary(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.print_summary {
            return_value = cfg;
        }
    }

    if args.is_present("print-summary") {
        return_value = true;
    }

    if return_value {
        log::debug!("Print summary flag set. Will output summary when all processing is done.");
    } else {
        log::debug!("Print summary not set. Will not output summary when all processing is done.");
    }

    // return the value
    return_value
}

/// Check if the quiet flag has been set, either in the config file
/// or via the CLI.
fn quiet(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.quiet {
            return_value = cfg;
        }
    }

    if args.is_present("quiet") {
        return_value = true;
    }

    if return_value {
        log::debug!("Quiet flag set. Will suppress output except warnings or errors.");
    } else {
        log::debug!("Quiet flag not set. Will output details as files are processed.");
    }

    // return the value
    return_value
}

/// Check if the detail-off flag has been set, either in the config file
/// or via the CLI.
fn detail_off(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.detail_off {
            return_value = cfg;
        }
    }

    if args.is_present("detail-off") {
        return_value = true;
    }

    if return_value {
        log::debug!("Detail off flag set. Will suppress output except warnings or errors.");
    } else {
        log::debug!("Detail off flag not set. Will output details as files are processed.");
    }

    // return the value
    return_value
}

/// Check if the detail-off flag has been set, either in the config file
/// or via the CLI.
fn dry_run(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.dry_run {
            return_value = cfg;
        }
    }

    if args.is_present("dry-run") {
        return_value = true;
    }

    if return_value {
        log::debug!(
            "Dry run flag set. Will not perform any actual processing, only report output."
        );
    } else {
        log::debug!("Dry run flag not set. Will process files.");
    }

    // return the value
    return_value
}
