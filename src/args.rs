// Process the CLI arguments and find out which flags to set
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::default_values::DefaultValues;

/// Collect the various options submitted into a HashMap for later use
pub fn parse_tags(
    default_values: &DefaultValues,
    args: &clap::ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_tags = HashMap::new();

    if args.is_present("album-artist") {
        new_tags.insert(
            "ALBUMARTIST".to_string(),
            args.value_of("album-artist").unwrap().to_string(),
        );
    }
    if args.is_present("album-title") {
        new_tags.insert(
            "ALBUM".to_string(),
            args.value_of("album-title").unwrap().to_string(),
        );
    }
    if args.is_present("album-genre") {
        new_tags.insert(
            "GENRE".to_string(),
            args.value_of("album-genre").unwrap().to_string(),
        );
    } else {
        if let Some(genre) = &default_values.genre {
            new_tags.insert("GENRE".to_string(), genre.to_string());
        }
    }
    if args.is_present("album-date") {
        new_tags.insert(
            "DATE".to_string(),
            args.value_of("album-date").unwrap().to_string(),
        );
    }
    if args.is_present("disc-number") {
        new_tags.insert(
            "DISCNUMBER".to_string(),
            args.value_of("disc-number").unwrap().to_string(),
        );
    }
    if args.is_present("disc-total") {
        new_tags.insert(
            "DISCTOTAL".to_string(),
            args.value_of("disc-total").unwrap().to_string(),
        );
    }
    if args.is_present("track-artist") {
        new_tags.insert(
            "ARTIST".to_string(),
            args.value_of("track-artist").unwrap().to_string(),
        );
    }
    if args.is_present("track-title") {
        new_tags.insert(
            "ARTIST".to_string(),
            args.value_of("track-title").unwrap().to_string(),
        );
    }
    if args.is_present("track-number") {
        new_tags.insert(
            "TRACKNUMBER".to_string(),
            args.value_of("track-number").unwrap().to_string(),
        );
    }
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
            return Err(format!(
                "Config file picture_front: file {} not found.",
                &picture_front
            )
            .into());
        } else {
            new_tags.insert("PICTUREFRONT".to_string(), picture_front.to_string());
        }
    } else {
        if let Some(picture_front) = &default_values.picture_front {
            if !Path::new(&picture_front).exists() {
                return Err(format!("--picture-front file {} not found.", &picture_front).into());
            } else {
                new_tags.insert("PICTUREFRONT".to_string(), picture_front.to_string());
            }
        }
    }
    if args.is_present("picture-back") {
        let picture_back = args.value_of("picture-back").unwrap();
        if !Path::new(&picture_back).exists() {
            return Err(format!("--picture-back file {} not found.", &picture_back).into());
        } else {
            new_tags.insert("PICTUREBACK".to_string(), picture_back.to_string());
        }
    } else {
        if let Some(picture_back) = &default_values.picture_back {
            if !Path::new(&picture_back).exists() {
                return Err(format!(
                    "Config file picture_back: file {} not found.",
                    &picture_back
                )
                .into());
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
pub fn stop_on_error(default_values: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if let Some(cfg) = default_values.stop_on_error {
        return_value = cfg;
    }

    if args.is_present("stop-on-error") {
        return_value = true;
    }

    if return_value {
        log::debug!("args::stop_on_error - Stop on error flag set. Will stop if errors occur.");
    } else {
        log::debug!("args::stop_on_error - Stop on error flag not set. Will attempt to continue in case of errors.");
    }

    // return the value
    return_value
}

/// Check if the print-summary flag has been set, either in the config file
/// or via the CLI.
pub fn print_summary(default_values: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if let Some(cfg) = default_values.print_summary {
        return_value = cfg;
    }

    if args.is_present("print-summary") {
        return_value = true;
    }

    if return_value {
        log::debug!("args::print_summary - Print summary flag set. Will output summary when all processing is done.");
    } else {
        log::debug!("args::print_summary - Print summary not set. Will not output summary when all processing is done.");
    }

    // return the value
    return_value
}

/// Check if the quiet flag has been set, either in the config file
/// or via the CLI.
pub fn quiet(default_values: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if let Some(cfg) = default_values.quiet {
        return_value = cfg;
    }

    if args.is_present("quiet") {
        return_value = true;
    }

    if return_value {
        log::debug!(
            "args::quiet - Quiet flag set. Will suppress output except warnings or errors."
        );
    } else {
        log::debug!(
            "args::quiet - Quiet flag not set. Will output details as files are processed."
        );
    }

    // return the value
    return_value
}
