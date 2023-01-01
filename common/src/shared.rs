//! Struct(s) and functions used across several other modules.
// use env_logger::{Builder, Target};

use std::ffi::OsStr;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{error::Error, time::SystemTime};

use infer::MatcherType;

use crate::file_types::FileTypes;

/// Find the MIME type (ie. `image/[bmp|gif|jpeg|png|tiff`) based on the file extension. Not perfect, but it'll do for now.
pub fn get_mime_type(filename: &str) -> Result<String, Box<dyn Error>> {
    // Read the file and check the mime type
    let file_type = if let Some(file_type) = infer::get_from_path(filename)? {
        file_type
    } else {
        return Err("File type not supported".into());
    };
    log::debug!("File type: {:?}", file_type);

    let mime_fmt = file_type.mime_type();
    log::debug!("MIME type: {}", mime_fmt);

    // Return safely
    Ok(mime_fmt.to_string())
}

/// Get the extension part of the filename and return it as a string
#[must_use] pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

// Get the file type from the Extension
pub fn get_file_type(filename: &str) -> Result<FileTypes, Box<dyn Error>> {
    // return the file type
    let file_type = infer::get_from_path(filename)?;
    log::debug!("File type = {:?}", file_type);
    let file_type = if let Some(file_type) = file_type {
        file_type
    } else {
        return Err("File type not supported".into());
    };

    let ft;

    if file_type.matcher_type() == MatcherType::Audio
        || file_type.matcher_type() == MatcherType::Video
    {
        log::debug!("File type is audio or video.");
        ft = match file_type.mime_type() {
            "audio/x-ape" => FileTypes::Ape,
            "audio/x-dsf" => FileTypes::Dsf,
            "audio/x-flac" => FileTypes::Flac,
            "audio/mpeg" => FileTypes::MP3,
            "audio/m4a" => FileTypes::MP4,
            "video/mp4" => FileTypes::MP4,
            _ => FileTypes::Unknown,
        };
        log::debug!("File type is {}", ft);
    } else {
        log::debug!("File type is not a recognized audio format. Trying MP4 variants.");
        let mp4vec = vec!["mp4a", "mp4b"];
        let ext = file_type.extension().to_lowercase();
        log::debug!("Extension: {}", ext);
        if mp4vec.contains(&ext.as_str()) {
            ft = FileTypes::MP4;
        } else {
            return Err("File type not supported".into());
        }
    }

    // return safely
    Ok(ft)
}

/// Checks that the new filename pattern results in a unique file.
/// Not perfect since the track title can occur multiple times on the same album.
/// TODO: Make this better. Include a check for the disc number and track title combo, for example.
pub fn file_rename_pattern_validate(pattern: &str) -> Result<(), String> {
    if !pattern.contains("%tn")
        && !pattern.contains("%tt")
        && !pattern.contains("%track-number")
        && !pattern.contains("%track-title")
        && !pattern.contains("%tts")
        && !pattern.contains("%track-title-sort")
    {
        Err(format!("Pattern \"{pattern}\" would likely not yield unique file names. Pattern should contain track number and/or track name variants."))
    } else {
        Ok(())
    }
}

/// Roman to Decimal conversion
#[must_use] pub fn roman_to_decimal(roman: &str) -> u16 {
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
#[must_use] pub fn need_split(value: &str) -> bool {
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
    let mut dir = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    log::debug!(
        "dir = {}, dir length = {}",
        dir.display(),
        dir.as_os_str().len()
    );

    if dir.as_os_str().is_empty() {
        dir = Path::new(&".");
    }

    if !dir.is_dir() {
        return Err(format!("Unable to get directory name from filename {filename}.").into());
    }

    // Get the list of (music) files in the directory
    let file_list = std::fs::read_dir(Path::new(dir))?
        .into_iter()
        .map(std::result::Result::unwrap)
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
    let count = file_list.count();
    log::debug!("count = {}", count);

    let file_count = if count < 100 {
        format!("{count:0>2}")
    } else {
        format!("{count:0>3}")
    };
    Ok(file_count)
}

/// Gets the microsecond part of the current duration since `UNIX_EPOCH` and modulate to a 4-digit number.
/// This is used to ensure uniqueness of file names.
/// This can be changed to something else later without impacting the main application.
/// For example, one could switch to a random number generator or something.
#[must_use] pub fn get_unique_value() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards. Presumably, you have bigger things to worry about.")
        .as_micros()
        % 10_000_000
}

/// Pretty-prints integer values;
/// Examples:
///
/// ```
/// assert_eq!(thousand_separated(10000), "10,000".to_string());
/// assert_eq!(thousand_separated(10000000), "10,000,000".to_string());
/// ```
pub fn thousand_separated<T>(val: T) -> String
where
    T: std::fmt::Display,
{
    let s = val.to_string();
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    let result: Vec<_> = chunks.join(",").bytes().rev().collect();
    String::from_utf8(result).unwrap()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay(include = ["../testdata/sample.mp3", "../testdata/sample.flac", "../testdata/sample.ape", "../testdata/sample.dsf", "../testdata/DSOTM_Cover.jpeg", "../testdata/sample.mp4", "../testdata/sample.m4a"])]
    /// Returns the mime type based on the file name
    fn test_get_mime_type() {
        assert!(get_mime_type("../testdata/sample.mp3").is_ok());
        assert!(get_mime_type("../testdata/sample.flac").is_ok());
        assert!(get_mime_type("../testdata/sample.ape").is_ok());
        assert!(get_mime_type("../testdata/sample.dsf").is_ok());
        assert!(get_mime_type("../testdata/DSOTM_Cover.jpeg").is_ok());
        assert!(get_mime_type("../testdata/sample.m4a").is_ok());
        assert!(get_mime_type("somefile.svg").is_err());

        assert_eq!(
            get_mime_type("../testdata/sample.mp3").unwrap(),
            "audio/mpeg".to_string()
        );
        assert_eq!(
            get_mime_type("../testdata/sample.flac").unwrap(),
            "audio/x-flac".to_string()
        );
        assert_eq!(
            get_mime_type("../testdata/sample.ape").unwrap(),
            "audio/x-ape".to_string()
        );
        assert_eq!(
            get_mime_type("../testdata/sample.dsf").unwrap(),
            "audio/x-dsf".to_string()
        );
        assert_eq!(
            get_mime_type("../testdata/DSOTM_Cover.jpeg").unwrap(),
            "image/jpeg".to_string()
        );
        assert_eq!(
            get_mime_type("../testdata/sample.m4a").unwrap(),
            "audio/m4a".to_string()
        );
    }

    #[assay]
    /// Tests the `get_extension` function to ensure it returns the correct extension.
    fn test_get_extension() {
        assert_eq!(get_extension("somefile.png"), "png".to_string());
        assert_eq!(get_extension("somewhere/somefile.png"), "png".to_string());
        assert_eq!(get_extension("noextension"), "unknown".to_string());
        assert_eq!(get_extension("noextension."), String::new());
    }

    #[assay]
    ///
    fn test_file_rename_pattern_validate() {
        assert!(file_rename_pattern_validate("%dn-%tn %tt").is_ok());
        assert!(file_rename_pattern_validate("%track-number %track-title").is_ok());
        assert!(file_rename_pattern_validate("%track-number %track-title-sort").is_ok());
        assert!(file_rename_pattern_validate("%track-title-sort").is_ok());

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
        assert!(split_val("1-2-3").is_err());
        assert!(split_val("1 av 2").is_err());

        assert_eq!(split_val("1 of 2").unwrap(), (1, 2));
        assert_eq!(split_val("1of2").unwrap(), (1, 2));
        assert_eq!(split_val("1 / 2").unwrap(), (1, 2));
        assert_eq!(split_val("1/2").unwrap(), (1, 2));
    }

    #[test]
    ///
    fn test_count_files() {
        if Path::new("../testdata/DOSTM_Cover-reesize.jpg").exists() {
            let _res = std::fs::remove_file(Path::new("../testdata/DOSTM_Cover-reesize.jpg"));
        }
        assert!(count_files("../testdata/sample.ape").is_ok());
        assert!(count_files("../testdata/sample.mp3").is_ok());
        assert!(count_files("../testdata/sample.flac").is_ok());
        assert!(count_files("../testdata/sample.mp4").is_ok());
        assert!(count_files("../testdata/sample.m4a").is_ok());
        assert!(count_files("../testdata/DSOTM_Cover.jpeg").is_ok());

        assert_eq!(
            count_files("../testdata/sample.ape").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("../testdata/sample.mp3").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("../testdata/sample.flac").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("../testdata/sample.mp4").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("../testdata/sample.m4a").unwrap(),
            "01".to_string()
        );
        assert_eq!(
            count_files("../testdata/DSOTM_Cover.jpeg").unwrap(),
            "02".to_string()
        );

        assert_eq!(
            count_files("../music/somefile.notfound").unwrap(),
            "00".to_string()
        );
    }

    /// Test the unique value generator
    #[assay]
    fn test_get_unique_value() {
        assert!(get_unique_value() < 10_000_000);
    }

    #[assay]
    ///
    fn test_thousand_separated() {
        assert_eq!(thousand_separated(10), "10".to_string());
        assert_eq!(thousand_separated(1000), "1,000".to_string());
        assert_eq!(thousand_separated(1_000_000), "1,000,000".to_string());
    }
}
