//! Struct(s) and functions used across several other modules.
// use env_logger::{Builder, Target};

use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};

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

    if config.log_config_file.is_some() {
        config_filename = config.log_config_file.as_ref().unwrap_or(&default).clone();
    }

    if cli_args.is_present("log-config-file") {
        config_filename = cli_args
            .value_of("log-config-file")
            .unwrap_or(&default)
            .to_string();
    }

    let path = Path::new(&shellexpand::tilde(&config_filename).to_string()).to_owned();
    if path.exists() {
        // Read the logger config from file
        log4rs::init_file(path, Default::default())?;
    } else {
        // If, for some reason, we can't find the logger config file, create a default logger profile

        // Build a stdout logger.
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{date(%Y-%m-%d %H:%M:%S)} {highlight({level})} {message}{n}",
            )))
            .target(Target::Stdout)
            .build();

        // Logging to log file.
        let logfile = FileAppender::builder()
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            .encoder(Box::new(PatternEncoder::new(
                "{date(%Y-%m-%d %H:%M:%S)} {highlight({level})} {message}{n}",
            )))
            .build("./id3tag.log")
            .unwrap();

        // Log Info level output to file where trace is the default level
        // and the programmatically specified level to stdout.
        let config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Warn)))
                    .build("logfile", Box::new(logfile)),
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Info)))
                    .build("stdout", Box::new(stdout)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Info),
            )
            .unwrap();

        // Use this to change log levels at runtime.
        // This means you can change the default log level to trace
        // if you are trying to debug an issue and need more logs on then turn it off
        // once you are done.
        let _handle = log4rs::init_config(config)?;
    }

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

/// Roman to Decimal conversion
pub fn roman_to_decimal(roman: &str) -> u16 {
    struct RomanNumeral {
        symbol: &'static str,
        value: u16,
    }

    const NUMERALS: [RomanNumeral; 13] = [
        RomanNumeral {
            symbol: "M",
            value: 1000,
        },
        RomanNumeral {
            symbol: "CM",
            value: 900,
        },
        RomanNumeral {
            symbol: "D",
            value: 500,
        },
        RomanNumeral {
            symbol: "CD",
            value: 400,
        },
        RomanNumeral {
            symbol: "C",
            value: 100,
        },
        RomanNumeral {
            symbol: "XC",
            value: 90,
        },
        RomanNumeral {
            symbol: "L",
            value: 50,
        },
        RomanNumeral {
            symbol: "XL",
            value: 40,
        },
        RomanNumeral {
            symbol: "X",
            value: 10,
        },
        RomanNumeral {
            symbol: "IX",
            value: 9,
        },
        RomanNumeral {
            symbol: "V",
            value: 5,
        },
        RomanNumeral {
            symbol: "IV",
            value: 4,
        },
        RomanNumeral {
            symbol: "I",
            value: 1,
        },
    ];

    match NUMERALS.iter().find(|num| roman.starts_with(num.symbol)) {
        Some(num) => num.value + roman_to_decimal(&roman[num.symbol.len()..]),
        None => 0, // if string empty, add nothing
    }
}
