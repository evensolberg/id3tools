//! Contains the struct and functions to maintain the confiuration state.

// Read default values from config file
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

//~ spec:startcode
/// The default values for the flags and options.
/// TODO: Write Deserialize trait for this struct
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
    pub disc_number_total: Option<u16>,

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
    pub track_number_total: Option<u16>,

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

    /// A list of front cover candidates
    pub picture_front_candidates: Option<Vec<String>>,

    /// A list of back cover candidates
    pub picture_back_candidates: Option<Vec<String>>,

    /// A list of search folders for cover candidates
    pub picture_search_folders: Option<Vec<String>>,

    /// Picture max size (in pixels - height and width)
    pub picture_max_size: Option<u32>,

    /// New filename pattern for rename
    pub rename_file: Option<String>,
}
//~ spec:endcode

impl DefaultValues {
    /// Initializes a new, empty set of `DefaultValues`. All values are set to `None` or empty vectors except the search folder which includes "." and "..".
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a config based on CLI arguments
    pub fn build_config(cli_args: &clap::ArgMatches) -> Result<Self, Box<dyn Error>> {
        let mut config = Self::new();

        let psf_list: Vec<String> = vec![String::from("."), String::from("..")];
        config.picture_search_folders = Some(psf_list);

        // Read the config file
        if cli_args.is_present("config-file") {
            let config_filename = shellexpand::tilde(
                cli_args
                    .value_of("config-file")
                    .unwrap_or("~/.config/id3tag/config.toml"),
            )
            .to_string();
            log::debug!("build_config::config_filename = {}", config_filename);
            config = Self::load_config(&config_filename)?;
        } else {
            log::debug!("No config file specified.");
        }

        // Collate config file flags and CLI flags and output the right config
        config.check_for_file_rename(cli_args)?;
        config.check_for_stop_on_error(cli_args);
        config.check_for_print_summary(cli_args);
        config.check_for_detail_off(cli_args);
        config.check_for_dry_run(cli_args);
        config.check_for_single_thread(cli_args);
        config.add_picture_search_folders(cli_args);
        config.check_for_picture_max_size(cli_args);
        config.check_for_picture_front_candidates(cli_args);
        config.check_for_picture_back_candidates(cli_args);
        log::debug!("Working config: {:?}", &config);

        Ok(config)
    }

    /// Loads the config from the supplied TOML file.
    pub fn load_config(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut config_toml = String::new();
        log::debug!("load_config::Loading {filename}");

        let mut file = File::open(filename)
            .map_err(|err| format!("Config file {filename} not found. Error: {err}"))?;

        log::debug!("load_config::file = {file:?}");

        let bytes = file.read_to_string(&mut config_toml)?;
        if bytes > 0 {
            log::debug!("Bytes read: {bytes}");
        } else {
            return Err(format!("Unable to read the contents of {filename}").into());
        }
        log::debug!("load_config::config_toml = {config_toml}");

        let mut config = match toml::from_str(&config_toml) {
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

        log::debug!("load_config::config = {config:?}");

        // Check if the picture_search_folders contain "." and "..". Add them if not.
        let mut psf = config.picture_search_folders.clone().unwrap_or_default();
        if !psf.contains(&'.'.to_string()) {
            psf.push('.'.to_string());
        }
        if !psf.contains(&"..".to_string()) {
            psf.push("..".to_string());
        }
        config.picture_search_folders = Some(psf);

        log::debug!("load_config::returning Ok({config:?})");

        Ok(config)
    }

    // Housekeeping functions to check which flags have been set, either on the CLI or in the config file.

    /// Checks the loaded config if there is a `file_rename` present, and validates it.
    /// Also checks the CLI for a rename-file and overrides any previous config entries if it is present.
    /// Returns OK if everything went well. Returns an error if the `file_rename` is invalid.
    fn check_for_file_rename(&mut self, args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
        // Check if anything came from the config file and validate it
        if let Some(pattern) = &self.rename_file {
            common::validate_file_rename_pattern(pattern)?;
        }

        // Even if we have something from the config file, CLI takes presedence
        if args.is_present("rename-file") {
            let pattern = args.value_of("rename-file").unwrap_or_default();
            common::validate_file_rename_pattern(pattern)?;
            self.rename_file = Some(pattern.to_string());
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

    /// Add any picture search folders from the CLI to the config.
    /// Note that '.' and '..' are always added to the list.
    fn add_picture_search_folders(&mut self, args: &clap::ArgMatches) {
        let mut candidate_list: Vec<String> = Vec::new();
        if let Some(folders) = args.values_of("picture-search-folder") {
            for folder in folders {
                candidate_list.push(folder.to_string());
            }
            candidate_list.push(".".to_string());
            candidate_list.push("..".to_string());
            self.picture_search_folders = Some(candidate_list);
        }
    }

    /// Set the maximum picture size from the CLI to the config.
    fn check_for_picture_max_size(&mut self, args: &clap::ArgMatches) {
        if let Some(size) = args.value_of("picture-max-size") {
            let pms: u32 = size.parse::<u32>().unwrap_or(0);
            self.picture_max_size = Some(pms);
            log::debug!("picture-max-size = {:?}", pms);
        }
    }

    /// Add the front cover candidates from the CLI to the config. If the list is empty, add "front.jpg", "cover.jpg", and "folder.jpg".
    fn check_for_picture_front_candidates(&mut self, args: &clap::ArgMatches) {
        let mut candidate_list: Vec<String> = Vec::new();
        if let Some(candidates) = args.values_of("picture-front-candidate") {
            for candidate in candidates {
                candidate_list.push(candidate.to_string());
            }
        }
        if !candidate_list.is_empty() && self.picture_front_candidates.is_none() {
            self.picture_front_candidates = Some(candidate_list);
        }
    }

    /// Add the back cover candidates from the CLI to the config. If the list is empty, add "back.jpg".
    fn check_for_picture_back_candidates(&mut self, args: &clap::ArgMatches) {
        let mut candidate_list: Vec<String> = Vec::new();
        if let Some(candidates) = args.values_of("picture-back-candidate") {
            for candidate in candidates {
                candidate_list.push(candidate.to_string());
            }
        }
        if !candidate_list.is_empty() && self.picture_back_candidates.is_none() {
            self.picture_back_candidates = Some(candidate_list);
        }
    }

    // Misc convenience functions

    /// Gathers the list of folder candidates into a vector. Uses "." and ".." if nothing is found.
    /// While this may seem redundant, it's safer since it always returns something.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the picture folder candidates from the config, or "." & ".." if the original list is empty.
    ///
    /// # Errors
    ///
    /// None.
    ///
    /// # Panics
    ///
    /// None.
    ///
    /// # Examples
    ///
    /// See tests.
    ///
    pub fn search_folders(&self) -> Vec<String> {
        if let Some(f) = &self.picture_search_folders {
            if !f.is_empty() {
                return self
                    .picture_search_folders
                    .as_ref()
                    .unwrap_or(&vec![".".to_string(), "..".to_string()])
                    .clone();
            }
        }
        vec![".".to_string(), "..".to_string()]
    }

    /// Get the list of front cover candidates
    pub fn picture_front_candidates(&self) -> Vec<String> {
        self.picture_front_candidates
            .as_ref()
            .unwrap_or(&vec![
                "front.jpg".to_string(),
                "cover.jpg".to_string(),
                "folder.jpg".to_string(),
            ])
            .clone()
    }

    /// Get the list of back cover candidates
    pub fn picture_back_candidates(&self) -> Vec<String> {
        self.picture_back_candidates
            .as_ref()
            .unwrap_or(&vec!["back.jpg".to_string()])
            .clone()
    }
} // impl DefaultValues

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    ///
    fn test_new_default_values() {
        // Create a blank config
        let mut dfv = DefaultValues::new();

        // Check that some values are "None"
        assert!(dfv.detail_off.is_none());
        assert!(dfv.log_config_file.is_none());
        assert!(dfv.album_artist.is_none());
        assert!(dfv.track_count.is_none());

        // Assign a few values
        dfv.disc_number = Some(1);
        dfv.disc_count = Some(true);

        // Check that the values got assigned OK.
        assert_eq!(dfv.disc_number.unwrap(), 1);
        assert_eq!(dfv.disc_count.unwrap(), true);
    }

    #[test]
    fn test_load_config() {
        // Try to load a config file
        let dfv = DefaultValues::load_config("../testdata/id3tag-config.toml");
        println!("dfv = {dfv:?}");
        assert!(dfv.is_ok());

        // Make sure we can unwrap the loaded config file
        let dfvu = dfv.unwrap();
        println!("dfvu = {dfvu:?}");

        assert_eq!(dfvu.detail_off.unwrap(), false);
        assert_eq!(dfvu.print_summary.unwrap(), true);
        assert_eq!(dfvu.stop_on_error.unwrap(), false);
        assert_eq!(dfvu.dry_run.unwrap(), true);
        assert_eq!(dfvu.single_thread.unwrap(), false);
        assert_eq!(dfvu.log_config_file.unwrap(), "log4rs.yaml".to_string());

        assert_eq!(
            dfvu.album_artist.unwrap(),
            "Ludwig van Beethoven".to_string()
        );
        assert_eq!(
            dfvu.album_artist_sort.unwrap(),
            "Beethoven, Ludwig van".to_string()
        );
        assert_eq!(dfvu.album_title.unwrap(), "Piano Sonata No. 5".to_string());
        assert_eq!(
            dfvu.album_title_sort.unwrap(),
            "Piano Sonata No. 5".to_string()
        );

        assert_eq!(dfvu.disc_number.unwrap(), 1);
        assert_eq!(dfvu.disc_count.unwrap(), true);
        assert_eq!(dfvu.disc_number_total.unwrap(), 2);

        assert_eq!(
            dfvu.track_artist.unwrap(),
            "Ludwig van Beethoven".to_string()
        );
        assert_eq!(
            dfvu.track_artist_sort.unwrap(),
            "Beethoven, Ludwig van".to_string()
        );
        assert_eq!(
            dfvu.track_title.unwrap(),
            "Piano Sonata No. 5 - II. Adagio".to_string()
        );
        assert_eq!(
            dfvu.track_title_sort.unwrap(),
            "Piano Sonata No. 5 - II. Adagio".to_string()
        );
        assert_eq!(dfvu.track_number.unwrap(), 2);
        assert_eq!(dfvu.track_count.unwrap(), true);
        assert_eq!(dfvu.track_number_total.unwrap(), 5);

        assert_eq!(dfvu.track_genre.unwrap(), "Classical".to_string());
        assert_eq!(dfvu.track_genre_number.unwrap(), 33);

        assert_eq!(
            dfvu.track_composer.unwrap(),
            "Ludwig van Beethoven".to_string()
        );
        assert_eq!(
            dfvu.track_composer_sort.unwrap(),
            "Beethoven, Ludwig van".to_string()
        );

        assert_eq!(dfvu.track_date.unwrap(), "1843".to_string());
        assert_eq!(
            dfvu.track_comments.unwrap(),
            "I have no idea if this is correct".to_string()
        );

        assert_eq!(dfvu.picture_front.unwrap(), "cover-resized.jpg".to_string());
        assert_eq!(dfvu.picture_back.unwrap(), "back-resized.jpg".to_string());

        assert_eq!(dfvu.picture_search_folders.unwrap().len(), 4);
        assert_eq!(dfvu.picture_front_candidates.unwrap().len(), 6);
        assert_eq!(dfvu.picture_back_candidates.unwrap().len(), 4);
        assert_eq!(dfvu.picture_max_size.unwrap(), 500);

        assert_eq!(dfvu.rename_file.unwrap(), "%dn-%tn - %ta - %tt".to_string());

        // Loading a non-existent config file should give an error.
        let dfv2 = DefaultValues::load_config("missing-file.toml");
        assert!(dfv2.is_err());
    }

    #[test]
    ///
    fn test_search_folders() {
        let mut cfg = DefaultValues::new();

        // Default is none.
        assert_eq!(
            cfg.search_folders(),
            vec![".".to_string(), "..".to_string()]
        );

        cfg.picture_search_folders = Some(vec![
            "Artwork".to_string(),
            "Scans".to_string(),
            "Covers".to_string(),
        ]);
        assert_eq!(
            cfg.search_folders(),
            vec![
                "Artwork".to_string(),
                "Scans".to_string(),
                "Covers".to_string()
            ]
        );
    }
}
