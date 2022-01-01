// Read default values from config file
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use crate::args;

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
        config.quiet = Some(args::quiet(&config, cli_args));
        config.stop_on_error = Some(args::stop_on_error(&config, cli_args));
        config.print_summary = Some(args::print_summary(&config, cli_args));
        config.detail_off = Some(args::detail_off(&config, cli_args));
        config.dry_run = Some(args::dry_run(&config, cli_args));
        log::debug!("Working config: {:?}", &config);

        Ok(config)
    }
} // impl DefaultValues
