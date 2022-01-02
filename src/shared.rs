//! Struct(s) and functions used across several other modules.
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

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
pub fn build_log(cli_args: &clap::ArgMatches) -> Result<Builder, Box<dyn Error>> {
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

    Ok(logbuilder)
}
