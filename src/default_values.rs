// Read default values from config file
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

/// The default values for the flags and options.
#[derive(Debug, Deserialize)]
pub struct DefaultValues {
    /// Flag: Do not output detail about each item processed.
    pub detail_off: Option<bool>,

    /// Flag: Print a summary of how many files were processed, skipped, etc.
    pub print_summary: Option<bool>,

    /// Flag: Only output warnings and errors
    pub quiet: Option<bool>,

    /// Flag: Stop immediately if an error occurs, otherwise continue
    pub stop_on_error: Option<bool>,

    /// Default value for the album's genre
    pub genre: Option<String>,

    /// Default value for the albums front cover
    pub picture_front: Option<String>,

    /// Default value for the album's back cover
    pub picture_back: Option<String>,
}
impl Default for DefaultValues {
    fn default() -> Self {
        Self {
            detail_off: None,
            print_summary: None,
            quiet: None,
            stop_on_error: None,
            genre: None,
            picture_front: None,
            picture_back: None,
        }
    }
}

impl DefaultValues {
    /// Initializes a new, empty set of DefaultValues. All values are set to `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the config from the supplied TOML file.
    pub fn load_config(filename: &str) -> Result<Self, Box<dyn Error>> {
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
                log::warn!("default_values::load_config - Unable to read config from {}. Using defaults. Error message: {}", filename, err.to_string());
                Self::new()
            }
        };
        log::debug!("config DefaultValues = {:?}", config);

        Ok(config)
    } // pub fn load_config
} // impl DefaultValues
