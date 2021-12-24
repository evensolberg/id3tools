// Process the CLI arguments and find out which flags to set
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::default_values::DefaultValues;
use clap::ArgMatches;

/// Collect the various options submitted into a HashMap for later use.
/// Also checks the default values loaded from a config file.
pub fn parse_tags(
    defaults: &DefaultValues,
    args: &ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_tags = HashMap::new();

    // Album artist
    if args.is_present("album-artist") {
        new_tags.insert(
            "ALBUMARTIST".to_string(),
            args.value_of("album-artist").unwrap().to_string(),
        );
    }

    // Album title
    if args.is_present("album-title") {
        new_tags.insert(
            "ALBUM".to_string(),
            args.value_of("album-title").unwrap().to_string(),
        );
    }

    // Genre
    if args.is_present("album-genre") {
        new_tags.insert(
            "GENRE".to_string(),
            args.value_of("album-genre").unwrap().to_string(),
        );
    } else {
        if let Some(genre) = &defaults.genre {
            new_tags.insert("GENRE".to_string(), genre.to_string());
        }
    }

    // Composer
    if args.is_present("album-composer") {
        new_tags.insert(
            "COMPOSER".to_string(),
            args.value_of("album-composer").unwrap().to_string(),
        );
    } else {
        if let Some(composer) = &defaults.composer {
            new_tags.insert("COMPOSER".to_string(), composer.to_string());
        }
    }

    // Date
    if args.is_present("album-date") {
        new_tags.insert(
            "DATE".to_string(),
            args.value_of("album-date").unwrap().to_string(),
        );
    }

    // Disc number
    if args.is_present("disc-number") {
        new_tags.insert(
            "DISCNUMBER".to_string(),
            args.value_of("disc-number").unwrap().to_string(),
        );
    }

    // Disc total
    if args.is_present("disc-total") {
        new_tags.insert(
            "DISCTOTAL".to_string(),
            args.value_of("disc-total").unwrap().to_string(),
        );
    }

    // Track artist
    if args.is_present("track-artist") {
        new_tags.insert(
            "ARTIST".to_string(),
            args.value_of("track-artist").unwrap().to_string(),
        );
    }

    // Track title
    if args.is_present("track-title") {
        new_tags.insert(
            "ARTIST".to_string(),
            args.value_of("track-title").unwrap().to_string(),
        );
    }

    // Track number
    if args.is_present("track-number") {
        new_tags.insert(
            "TRACKNUMBER".to_string(),
            args.value_of("track-number").unwrap().to_string(),
        );
    }

    // Track total
    if args.is_present("track-total") {
        new_tags.insert(
            "TRACKTOTAL".to_string(),
            args.value_of("track-total").unwrap().to_string(),
        );
    }

    // Check if picture files exist
    // Check parameter first, then fall back to config file (if something is specified there)
    if args.is_present("picture-front") {
        let picture_front = args.value_of("picture-front").unwrap();
        if !Path::new(&picture_front).exists() {
            if defaults.stop_on_error.unwrap() {
                return Err(format!(
                    "Config file picture_front: file {} not found.",
                    &picture_front
                )
                .into());
            } else {
                log::warn!(
                    "Config file picture_front: file {} not found. Continuing.",
                    &picture_front
                );
            }
        } else {
            new_tags.insert("PICTUREFRONT".to_string(), picture_front.to_string());
        }
    } else {
        if let Some(picture_front) = &defaults.picture_front {
            if !Path::new(&picture_front).exists() {
                if defaults.stop_on_error.unwrap() {
                    return Err(format!(
                        "Config file picture_front: file {} not found.",
                        &picture_front
                    )
                    .into());
                } else {
                    log::warn!(
                        "Config file picture_front: file {} not found. Continuing.",
                        &picture_front
                    );
                }
            } else {
                new_tags.insert("PICTUREFRONT".to_string(), picture_front.to_string());
            }
        }
    }
    if args.is_present("picture-back") {
        let picture_back = args.value_of("picture-back").unwrap();
        if !Path::new(&picture_back).exists() {
            if defaults.stop_on_error.unwrap() {
                return Err(format!(
                    "Config file picture_back: file {} not found.",
                    &picture_back
                )
                .into());
            } else {
                log::warn!(
                    "Config file picture_back: file {} not found. Continuing.",
                    &picture_back
                );
            }
        } else {
            new_tags.insert("PICTUREBACK".to_string(), picture_back.to_string());
        }
    } else {
        if let Some(picture_back) = &defaults.picture_back {
            if !Path::new(&picture_back).exists() {
                if defaults.stop_on_error.unwrap() {
                    return Err(format!(
                        "Config file picture_back: file {} not found.",
                        &picture_back
                    )
                    .into());
                } else {
                    log::warn!(
                        "Config file picture_back: file {} not found. Continuing.",
                        &picture_back
                    );
                }
            } else {
                new_tags.insert("PICTUREBACK".to_string(), picture_back.to_string());
            }
        }
    }

    // Return safely
    Ok(new_tags)
}

// Housekeeping functions to check which flags have been set, either here or in the config file.

/// Check if the stop-on-error flag has been set, either in the config file
/// or via the CLI.
pub fn stop_on_error(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if let Some(cfg) = defaults.stop_on_error {
        return_value = cfg;
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
pub fn print_summary(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if let Some(cfg) = defaults.print_summary {
        return_value = cfg;
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
pub fn quiet(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if let Some(cfg) = defaults.quiet {
        return_value = cfg;
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

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or(OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap()
        .to_string()
}
