//! Struct(s) and functions used across several other modules.

use std::ffi::OsStr;
use std::path::Path;
use std::time::UNIX_EPOCH;
use std::{error::Error, time::SystemTime};

use infer::{MatcherType, Type};

use crate::file_types::FileTypes;

/// Find the MIME type (ie. `image/[bmp|gif|jpeg|png|tiff`) based on the file extension. Not perfect, but it'll do for now.
///
/// # Errors
///
/// - If we can't infer the file type from path, we give an error
pub fn get_mime_type(filename: &str) -> Result<String, Box<dyn Error>> {
    // Read the file and check the mime type
    let Some(file_type) = infer::get_from_path(filename)? else {
        return Err("File type not supported".into());
    };

    Ok(file_type.mime_type().to_string())
}

/// Get the extension part of the filename and return it as a string
#[must_use]
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Get the file type from the Extension
///
/// # Errors
///
/// - `infer::get_from_path()` fails
pub fn get_file_type(filename: &str) -> Result<FileTypes, Box<dyn Error>> {
    // return the file type
    let file_type = infer::get_from_path(filename)?;
    log::debug!("File type = {:?}", file_type);
    let Some(file_type) = file_type else {
        return Err("File type not supported".into());
    };

    let ft;

    if file_type.matcher_type() == MatcherType::Audio
        || file_type.matcher_type() == MatcherType::Video
    {
        ft = audio_file_type(file_type);
        log::debug!("File type is {ft}");
    } else {
        log::debug!("File type is not a recognized audio format. Trying MP4 variants.");
        let mp4vec = vec!["mp4a", "mp4b"];
        let ext = file_type.extension().to_lowercase();
        if mp4vec.contains(&ext.as_str()) {
            ft = FileTypes::MP4;
        } else {
            return Err("File type not supported".into());
        }
    }

    // return safely
    Ok(ft)
}

/// Get the audio file type based on the the mime type from the Infer crate.
///
/// # Arguments
///
/// `ft: Type` - The file type returned from the Infer crate
///
/// # Returns
///
/// `FileTypes` - The audio file type
fn audio_file_type(ft: Type) -> FileTypes {
    match ft.mime_type() {
        "audio/x-ape" => FileTypes::Ape,
        "audio/x-dsf" => FileTypes::Dsf,
        "audio/x-flac" => FileTypes::Flac,
        "audio/mpeg" => FileTypes::MP3,
        "audio/m4a" | "video/mp4" => FileTypes::MP4,
        _ => FileTypes::Unknown,
    }
}

/// Checks that the new filename pattern results in a unique file.
/// Not perfect since the track title can occur multiple times on the same album.
/// TODO: Make this better. Include a check for the disc number and track title combo, for example.
///
/// # Errors
///
/// - Return an error if the pattern provided is unlikely to return unique file names
#[must_use]
pub fn file_rename_pattern_not_ok(pattern: &str) -> bool {
    !pattern.contains("%tn")
        && !pattern.contains("%track-number")
        && !pattern.contains("%tt")
        && !pattern.contains("%track-title")
        && !pattern.contains("%tts")
        && !pattern.contains("%track-title-sort")
}

/// Roman to Decimal conversion
#[must_use]
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

    NUMERALS
        .iter()
        .find(|num| roman.to_uppercase().starts_with(num.symbol))
        .map_or(0, |num| {
            num.value + roman_to_decimal(&roman[num.symbol.len()..])
        })
}

/// Determines if a value (typically track or disc number) needs to be split into two values.
/// This is determined if the provided value contains "/" or "of"
#[must_use]
pub fn need_split(value: &str) -> bool {
    value.contains('/') || value.contains("of")
}

/// Splits a value (typically track or disc number) into two values at a "/" or "of".
///
/// # Errors
///
/// - Returns an error if the split pattern can't be found.
pub fn split_val(value: &str) -> Result<(u16, u16), Box<dyn Error>> {
    let split_str: Vec<&str>;
    if value.contains("of") {
        split_str = value.split("of").collect();
    } else if value.contains('/') {
        split_str = value.split('/').collect();
    } else {
        return Err("Split pattern not found.".into());
    }

    let count = split_str[0].trim().parse::<u16>().unwrap_or(1);
    let total = split_str[1].trim().parse::<u16>().unwrap_or(1);

    // return the values (i.e., 1 of 2)
    Ok((count, total))
}

/// Counts the number of files in with the same extension in the same directory as the file specified.
/// TODO: Look into canonicalizing the path so we can be sure to get the parent.
///
/// The count is returned as a formatted `String`.
///
/// # Arguments
/// `filename: &str` - the name of the file to be used as the example. The function will get the extension and look for the number of files with the same extension.
///
/// # Returns
/// `Result<String, Box<dyn Error>>` - a formatted string with the number of files found, or an error if something went wrong.
///
/// # Errors
/// - Returns an error if unable to get the directory name from the fielname.
///
/// # Panics
/// None.
pub fn count_files(filename: &str) -> Result<String, Box<dyn Error>> {
    let ext = get_extension(filename);

    // Get just the directory part, excluding the filename
    let mut dir = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    if dir.as_os_str().is_empty() {
        dir = Path::new(&".");
    }

    if !dir.is_dir() {
        return Err(format!("Unable to get directory name from filename {filename}.").into());
    }

    // Get the list of (music) files in the directory
    let file_list = std::fs::read_dir(Path::new(dir))?
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

    // Format the file count
    let file_count = if count < 100 {
        format!("{count:0>2}")
    } else {
        format!("{count:0>3}")
    };

    // Return safely with the formatted file count
    Ok(file_count)
}

/// Gets the microsecond part of the current duration since `UNIX_EPOCH` and modulate to a 4-digit number.
/// This is used to ensure uniqueness of file names.
/// This can be changed to something else later without impacting the main application.
/// For example, one could switch to a random number generator or something.
#[must_use]
pub fn get_unique_value() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time appears to have gone backwards. Or it's after 03:14:07 UTC on 19 January 2038 (the Epochalypse).")
        .as_micros()
        % 10_000_000
}

/// Pretty-prints integer values;
/// Examples:
///
/// ```
/// assert_eq!(thousand_separated(10000), String::from("10,000"));
/// assert_eq!(thousand_separated(10000000), String::from("10,000,000"));
/// ```
///
/// # Panics
///
/// None.
pub fn thousand_separated<T>(val: T) -> String
where
    T: std::fmt::Display,
{
    let s = val.to_string();
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
        .collect();
    let result: Vec<_> = chunks.join(",").bytes().rev().collect();
    String::from_utf8(result)
        .unwrap_or_default()
        .replace(",.", ".")
}

/// Gets the complete directory path to the file, sans the filename.
///
/// # Arguments
/// `filename: &str` - the name of the file for which we need the full path
///
/// # Returns
/// `Result<std::path::PathBuf, Box<dyn Error>>` - a `PathBuf` containing the full directory path to the file if succcessful.
///
/// # Errors
///
/// `canonicalize` has a problem.
///
/// # Example
/// `get_full_path_directory("/some/path/myfile.txt")` returns "/some/path/"
pub fn directory(filename: &str) -> Result<std::path::PathBuf, Box<dyn Error>> {
    let mut music_file_path = std::fs::canonicalize(filename)?;
    music_file_path = music_file_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    log::debug!("music_file_path = {:?}", music_file_path);

    Ok(music_file_path)
}

/// Converts a `Path` to a `String`
#[must_use]
pub fn path_to_string(p: std::path::PathBuf) -> String {
    p.into_os_string().into_string().unwrap_or_default()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
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

    #[test]
    /// Tests the `get_extension` function to ensure it returns the correct extension.
    fn test_get_extension() {
        assert_eq!(get_extension("somefile.png"), "png".to_string());
        assert_eq!(get_extension("somewhere/somefile.png"), "png".to_string());
        assert_eq!(get_extension("noextension"), "unknown".to_string());
        assert_eq!(get_extension("noextension."), String::new());
    }

    #[test]
    ///
    fn test_file_rename_pattern_validate() {
        assert!(!file_rename_pattern_not_ok("%dn-%tn %tt"));
        assert!(!file_rename_pattern_not_ok("%track-number %track-title"));
        assert!(!file_rename_pattern_not_ok(
            "%track-number %track-title-sort"
        ));
        assert!(!file_rename_pattern_not_ok("%track-title-sort"));
        assert!(file_rename_pattern_not_ok("%disc-number"));
    }

    #[test]
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

    #[test]
    /// Tests whether the `need_split` function does what it says on the tin
    fn test_need_split() {
        assert!(need_split("1 of 2"));
        assert!(need_split("1of2"));
        assert!(need_split("1 / 2"));
        assert!(need_split("1/2"));

        assert!(!need_split("1"));
        assert!(!need_split("01"));
        assert!(!need_split("03"));
        assert!(!need_split("DISC 03"));
    }

    #[test]
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
            "03".to_string()
        );

        assert_eq!(
            count_files("../music/somefile.notfound").unwrap(),
            "00".to_string()
        );
    }

    /// Test the unique value generator
    #[test]
    fn test_get_unique_value() {
        assert!(get_unique_value() < 10_000_000);
    }

    #[test]
    ///
    fn test_thousand_separated() {
        assert_eq!(thousand_separated(10), "10".to_string());
        assert_eq!(thousand_separated(1000), "1,000".to_string());
        assert_eq!(thousand_separated(1_000_000), "1,000,000".to_string());
        assert_eq!(thousand_separated(1000.01), "1,000.01".to_string());
    }
}
