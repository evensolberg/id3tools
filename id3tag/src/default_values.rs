//! Contains the struct and functions to maintain the confiuration state.

// Read default values from config file
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

//~ spec:startcode
/// The default values for the flags and options.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct DefaultValues {
    /// Flag: Do not output detail about each item processed.
    pub detail_off: Option<bool>,

    /// Flag: Print a summary of how many files were processed, skipped, etc.
    pub print_summary: Option<bool>,

    /// Flag: Stop immediately if an error occurs, otherwise continue.
    pub stop_on_error: Option<bool>,

    /// Flag: Don't actually write any changes.
    pub dry_run: Option<bool>,

    /// Flag: Single-threaded execution
    pub single_thread: Option<bool>,

    /// The name of the logging configuration file
    pub log_config_file: Option<String>,

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

    /// Figure out the disc number
    pub disc_count: Option<bool>,

    /// The total number of discs that comprise the album, usually 1.
    pub disc_total: Option<u16>,

    /// Default value for the track's artist.
    pub track_artist: Option<String>,

    /// Set the track artist to be the same as the album artist
    pub track_album_artist: Option<String>,

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

    /// Count the number of tracks
    pub track_count: Option<bool>,

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

    /// New filename pattern for rename
    pub rename_file: Option<String>,
}
//~ spec:endcode

impl DefaultValues {
    /// Initializes a new, empty set of `DefaultValues`. All values are set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a config based on CLI arguments
    pub fn build_config(cli_args: &clap::ArgMatches) -> Result<Self, Box<dyn Error>> {
        let mut config = DefaultValues::new();

        // Read the config file
        if cli_args.is_present("config-file") {
            let config_filename = shellexpand::tilde(
                cli_args
                    .value_of("config-file")
                    .unwrap_or("~/.config/id3tag/config.toml"),
            )
            .to_string();
            log::debug!("Config filename: {}", config_filename);
            config = DefaultValues::load_config(&config_filename)?;
            log::debug!("Loaded config: {:?}", &config);
        }

        // Collate config file flags and CLI flags and output the right config
        config.check_for_file_rename(cli_args)?;
        config.check_for_stop_on_error(cli_args);
        config.check_for_print_summary(cli_args);
        config.check_for_detail_off(cli_args);
        config.check_for_dry_run(cli_args);
        config.check_for_single_thread(cli_args);
        log::debug!("Working config: {:?}", &config);

        Ok(config)
    }

    /// Loads the config from the supplied TOML file.
    fn load_config(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut config_toml = String::new();

        let mut file = File::open(&filename)
            .map_err(|err| format!("Config file {} not found. Error: {}", filename, err))?;

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

    // Housekeeping functions to check which flags have been set, either on the CLI or in the config file.

    /// Checks the loaded config if there is a `file_rename` present, and validates it.
    /// Also checks the CLI for a rename-file and overrides any previous config entries if it is present.
    /// Returns OK if everything went well. Returns an error if the `file_rename` is invalid.
    fn check_for_file_rename(&mut self, args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
        // Check if anything came from the config file and validate it
        if let Some(rnp) = &self.rename_file {
            common::file_rename_pattern_validate(rnp)?;
        }

        // Even if we have something from the config file, CLI takes presedence
        if args.is_present("rename-file") {
            self.rename_file = Some(args.value_of("rename-file").unwrap_or_default().to_string());
        };

        // Return safely
        Ok(())
    }

    /// Check if the stop-on-error flag has been set, either in the config file
    /// or via the CLI.
    fn check_for_stop_on_error(&mut self, args: &clap::ArgMatches) {
        if args.is_present("stop-on-error") {
            self.stop_on_error = Some(true);
        } else if self.stop_on_error.is_none() {
            self.stop_on_error = Some(false);
        }
    }

    /// Check if the print-summary flag has been set, either in the config file
    /// or via the CLI.
    fn check_for_print_summary(&mut self, args: &clap::ArgMatches) {
        if args.is_present("print-summary") {
            self.print_summary = Some(true);
        } else if self.print_summary.is_none() {
            self.print_summary = Some(false);
        }
    }

    /// Check if the detail-off flag has been set, either in the config file
    /// or via the CLI.
    fn check_for_detail_off(&mut self, args: &clap::ArgMatches) {
        if args.is_present("detail-off") {
            self.detail_off = Some(true);
        } else if self.detail_off.is_none() {
            self.detail_off = Some(false);
        }
    }

    /// Check if the detail-off flag has been set, either in the config file
    /// or via the CLI.
    fn check_for_dry_run(&mut self, args: &clap::ArgMatches) {
        if args.is_present("dry-run") {
            self.dry_run = Some(true);
        } else if self.dry_run.is_none() {
            self.dry_run = Some(false);
        }
    }

    /// Check if the single-thread flag has been set, either in the config file
    /// or via the CLI.
    fn check_for_single_thread(&mut self, args: &clap::ArgMatches) {
        if args.is_present("single-thread") {
            self.single_thread = Some(true);
        } else if self.single_thread.is_none() {
            self.single_thread = Some(false);
        }
    }
} // impl DefaultValues

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    ///
    fn test_default_values() {
        // Create a blank config
        let mut def_val = DefaultValues::new();

        // Check that some values are "None"
        assert!(def_val.detail_off.is_none());
        assert!(def_val.log_config_file.is_none());
        assert!(def_val.album_artist.is_none());
        assert!(def_val.track_count.is_none());

        // Assign a few values
        def_val.disc_number = Some(1);
        def_val.disc_count = Some(true);

        // Check that the values got assigned OK.
        assert_eq!(def_val.disc_number.unwrap(), 1);
        assert_eq!(def_val.disc_count.unwrap(), true);

        // Try to load a config file
        let dfv2 = DefaultValues::load_config("id3tag-config.toml");
        assert!(dfv2.is_ok());

        // Make sure we can unwrap the loaded config file
        let dfv2u = dfv2.unwrap();
        assert_eq!(dfv2u.detail_off.unwrap(), false);
        assert_eq!(dfv2u.print_summary.unwrap(), true);
        assert_eq!(dfv2u.stop_on_error.unwrap(), false);
        assert_eq!(dfv2u.track_genre.unwrap(), "Metal".to_string());
        assert_eq!(dfv2u.picture_front.unwrap(), "cover-small.jpg".to_string());

        // Loading a non-existent config file should give an error.
        let dfv3 = DefaultValues::load_config("missing-file.toml");
        assert!(dfv3.is_err());
    }
}
