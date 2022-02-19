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

    match NUMERALS
        .iter()
        .find(|num| roman.to_uppercase().starts_with(num.symbol))
    {
        Some(num) => num.value + roman_to_decimal(&roman[num.symbol.len()..]),
        None => 0, // if string empty, add nothing
    }
}

/// Determines if a value (typically track or disc number) needs to be split into two values.
/// This is determined if the provided value contains "/" or "of"
pub fn need_split(value: &str) -> bool {
    value.contains('/') || value.contains("of")
}

/// Splits a value (typically track or disc number) into two values at a "/" or "of".
pub fn split_val(value: &str) -> Result<(u16, u16), Box<dyn Error>> {
    let split_str: Vec<&str>;
    if value.contains("of") {
        split_str = value.split("of").collect();
    } else if value.contains('/') {
        split_str = value.split('/').collect();
    } else {
        return Err("Split pattern not found.".into());
    }

    log::debug!("split_str = {:?}", split_str);
    let num = split_str[0].trim().parse::<u16>().unwrap_or(1);
    let total = split_str[1].trim().parse::<u16>().unwrap_or(1);

    // return the values
    Ok((num, total))
}

/// Counts the number of files in with the same extension in the same directory as the file specified.
pub fn count_files(filename: &str) -> Result<String, Box<dyn Error>> {
    let ext = get_extension(filename);
    log::debug!("ext = {}", ext);

    // Get just the directory part, excluding the filename
    let dir = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    log::debug!("dir = {}", dir.display());

    if !dir.is_dir() {
        return Err(format!("Unable to get directory name from filename {}.", filename).into());
    }

    // Get the list of (music) files in the directory
    let file_list = std::fs::read_dir(Path::new(dir))?
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| {
            x.path()
                .extension()
                .unwrap_or_else(|| OsStr::new(""))
                .to_str()
                .unwrap_or("")
                == ext
        });

    // return safely with the number of files found
    log::debug!("file_list = {:?}", &file_list);
    let file_count = format!("{:0>2}", file_list.count());
    Ok(file_count)
}

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    /// Returns the mime type based on the file name
    fn test_get_mime_type() {
        assert!(get_mime_type("somefile.bmp").is_ok());
        assert!(get_mime_type("somefile.gif").is_ok());
        assert!(get_mime_type("somefile.jpg").is_ok());
        assert!(get_mime_type("somefile.jpeg").is_ok());
        assert!(get_mime_type("somefile.png").is_ok());
        assert!(get_mime_type("somefile.tif").is_ok());
        assert!(get_mime_type("somefile.tiff").is_ok());
        assert!(get_mime_type("somefile.svg").is_err());

        assert_eq!(
            get_mime_type("somefile.bmp").unwrap(),
            "image/bmp".to_string()
        );
        assert_eq!(
            get_mime_type("somefile.gif").unwrap(),
            "image/gif".to_string()
        );
        assert_eq!(
            get_mime_type("somefile.jpg").unwrap(),
            "image/jpeg".to_string()
        );
        assert_eq!(
            get_mime_type("somefile.jpeg").unwrap(),
            "image/jpeg".to_string()
        );
        assert_eq!(
            get_mime_type("somefile.png").unwrap(),
            "image/png".to_string()
        );
        assert_eq!(
            get_mime_type("somefile.tif").unwrap(),
            "image/tiff".to_string()
        );
        assert_eq!(
            get_mime_type("somefile.tiff").unwrap(),
            "image/tiff".to_string()
        );
    }

    #[assay]
    ///
    fn test_get_extension() {
        assert_eq!(get_extension("somefile.png"), "png".to_string());
        assert_eq!(get_extension("somewhere/somefile.png"), "png".to_string());
        assert_eq!(get_extension("noextension"), "unknown".to_string());
        assert_eq!(get_extension("noextension."), "".to_string());
    }

    #[assay]
    ///
    fn test_file_rename_pattern_validate() {
        assert!(file_rename_pattern_validate("%dn-%tn %tt").is_ok());
        assert!(file_rename_pattern_validate("%track-number %track-title").is_ok());

        assert!(file_rename_pattern_validate("%disc-number").is_err());
    }

    #[assay]
    ///
    fn test_roman_to_decimal() {
        assert_eq!(roman_to_decimal("I"), 1);
        assert_eq!(roman_to_decimal("i"), 1);
        assert_eq!(roman_to_decimal("II"), 2);
        assert_eq!(roman_to_decimal("ii"), 2);
        assert_eq!(roman_to_decimal("III"), 3);
        assert_eq!(roman_to_decimal("iii"), 3);
        assert_eq!(roman_to_decimal("IV"), 4);
        assert_eq!(roman_to_decimal("iv"), 4);
        assert_eq!(roman_to_decimal("V"), 5);
        assert_eq!(roman_to_decimal("v"), 5);
        assert_eq!(roman_to_decimal("VI"), 6);
        assert_eq!(roman_to_decimal("vi"), 6);
        assert_eq!(roman_to_decimal("VII"), 7);
        assert_eq!(roman_to_decimal("vii"), 7);
        assert_eq!(roman_to_decimal("IX"), 9);
        assert_eq!(roman_to_decimal("ix"), 9);
        assert_eq!(roman_to_decimal("X"), 10);
        assert_eq!(roman_to_decimal("x"), 10);
        assert_eq!(roman_to_decimal("XI"), 11);
        assert_eq!(roman_to_decimal("xi"), 11);
        assert_eq!(roman_to_decimal("L"), 50);
        assert_eq!(roman_to_decimal("LI"), 51);
        assert_eq!(roman_to_decimal("XC"), 90);
        assert_eq!(roman_to_decimal("C"), 100);
        assert_eq!(roman_to_decimal("D"), 500);
        assert_eq!(roman_to_decimal("CD"), 400);
        assert_eq!(roman_to_decimal("M"), 1000);
        assert_eq!(roman_to_decimal("CM"), 900);

        assert_ne!(roman_to_decimal("IL"), 49);
        assert_ne!(roman_to_decimal("IC"), 99);
        assert_ne!(roman_to_decimal("ID"), 499);
        assert_ne!(roman_to_decimal("IM"), 999);
    }

    #[assay]
    /// Tests whether the need_split function does what it says on the tin
    fn test_need_split() {
        assert_eq!(need_split("1 of 2"), true);
        assert_eq!(need_split("1of2"), true);
        assert_eq!(need_split("1 / 2"), true);
        assert_eq!(need_split("1/2"), true);

        assert_eq!(need_split("1"), false);
        assert_eq!(need_split("01"), false);
        assert_eq!(need_split("03"), false);
        assert_eq!(need_split("DISC 03"), false);
    }

    #[assay]
    ///
    fn test_split_val() {
        assert!(split_val("1 of 2").is_ok());
        assert!(split_val("1of2").is_ok());
        assert!(split_val("1 / 2").is_ok());
        assert!(split_val("1/2").is_ok());

        assert!(split_val("1-2").is_err());

        assert_eq!(split_val("1 of 2").unwrap(), (1, 2));
        assert_eq!(split_val("1of2").unwrap(), (1, 2));
        assert_eq!(split_val("1 / 2").unwrap(), (1, 2));
        assert_eq!(split_val("1/2").unwrap(), (1, 2));
    }

    #[test]
    ///
    fn test_count_files() {
        assert!(count_files("music/01.ape").is_ok());
        assert!(count_files("music/01 Gavottes BWV 1012.mp3").is_ok());
        assert!(count_files("music/01-13 Surf's Up.flac").is_ok());
        assert!(count_files("music/glb.mp4").is_ok());
        assert!(count_files("music/This Is The Story.m4a").is_ok());
        assert!(count_files("music/cover-small.jpg").is_ok());

        assert_eq!(count_files("music/01.ape").unwrap(), "02".to_string());
        assert_eq!(
            count_files("music/01 Gavottes BWV 1012.mp3").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("music/01-13 Surf's Up.flac").unwrap(),
            "02".to_string()
        );
        assert_eq!(count_files("music/glb.mp4").unwrap(), "01".to_string());
        assert_eq!(
            count_files("music/This Is The Story.m4a").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("music/cover-small.jpg").unwrap(),
            "01".to_string()
        );

        assert_eq!(
            count_files("music/somefile.notfound").unwrap(),
            "00".to_string()
        );
    }
}
