//! Struct(s) and functions used across several other modules.
// use env_logger::{Builder, Target};

use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::default_values::DefaultValues;

#[derive(Debug, Default, Clone, Copy)]
pub struct Counts {
    pub total_file_count: usize,
    pub processed_file_count: usize,
    pub skipped_file_count: usize,
}

/// Find the MIME type (ie. `image/[bmp|gif|jpeg|png|tiff`) based on the file extension. Not perfect, but it'll do for now.
pub fn get_mime_type(filename: &str) -> Result<String, Box<dyn Error>> {
    let ext = get_extension(filename);
    let fmt_str = match ext.as_ref() {
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "tif" | "tiff" => "image/tiff",
        _ => {
            return Err(
                "Image format not supported. Must be one of BMP, GIF, JPEG, PNG or TIFF.".into(),
            )
        }
    };

    // Return safely
    Ok(fmt_str.to_string())
}

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Creates a log entity for us
pub fn build_log(
    cli_args: &clap::ArgMatches,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    let default = "~/.config/id3tag/logs.yaml".to_string();

    let mut config_filename = default.clone();

    if !config.log_config_file.is_none() {
        config_filename = config.log_config_file.as_ref().unwrap_or(&default).clone();
    }

    if cli_args.is_present("log-config-file") {
        config_filename = cli_args
            .value_of("log-config-file")
            .unwrap_or(&default)
            .to_string();
    }

    let path = &shellexpand::tilde(&config_filename).to_string();
    log4rs::init_file(Path::new(path), Default::default())?;

    Ok(())
}

/// Checks that the new filename pattern results in a unique file
pub fn file_rename_pattern_validate(pattern: &str) -> Result<(), String> {
    if !pattern.contains("%tn")
        && !pattern.contains("%tt")
        && !pattern.contains("%track-number")
        && !pattern.contains("%track-title")
    {
        Err(format!("Pattern \"{}\" would not yield unique file names. Pattern must contain track number and/or track name. Cannot continue.", pattern))
    } else {
        Ok(())
    }
}
