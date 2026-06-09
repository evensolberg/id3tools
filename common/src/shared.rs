//! Struct(s) and functions used across several other modules.

use std::ffi::OsStr;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::{bail, Context, Result};
use infer::{MatcherType, Type};

use crate::file_types::FileTypes;

/// Expand glob patterns in a list of file arguments into actual file paths.
///
/// Each argument is classified as follows:
///
/// - **No glob characters** (`*`, `?`, `[`): passed through as-is so that the
///   caller (clap/downstream) can report a meaningful error if the file does
///   not exist.
/// - **Contains `[` but no `*` or `?`, and a filesystem entry with that exact
///   name exists**: the argument is treated as a *literal* path. Detection uses
///   [`std::fs::symlink_metadata`] (does **not** follow symlinks) so that
///   dangling symlinks — whose names may contain `[` — are correctly recognised
///   as present entries rather than fed to the glob engine. Any OS error other
///   than [`NotFound`][std::io::ErrorKind::NotFound] (e.g. `PermissionDenied`)
///   is also treated conservatively as "exists", preferring a downstream
///   open-error over silently dropping the argument. This handles the common
///   case of real audio filenames like `Song [Live].mp3`.
/// - **Contains `*` or `?`** (with or without `[`), **or contains `[` and no
///   matching entry exists**: the argument is treated as a glob pattern and
///   expanded by [`glob::glob`]. `*`/`?`-containing patterns are always
///   expanded regardless of whether a same-named literal entry exists, so that
///   `*.mp3` always expands to all matching files rather than being shadowed
///   by an unusual literal file called `*.mp3`. Patterns that match nothing
///   produce a `warn!` log and contribute no entries to the result. Invalid
///   patterns fall back to literal passthrough (with a `warn!`).
///
/// # Arguments
///
/// * `args` - An iterator of string references from CLI arguments
///
/// # Returns
///
/// A `Vec<String>` containing all expanded file paths.
pub fn expand_file_args<'a, I>(args: I) -> Vec<String>
where
    I: Iterator<Item = &'a str>,
{
    let mut result = Vec::new();
    for arg in args {
        // Compute the two constituent flags first so they can be reused
        // inside the branch without rescanning the string.
        let has_wildcards = arg.contains('*') || arg.contains('?');
        let has_bracket = arg.contains('[');
        let looks_like_glob = has_wildcards || has_bracket;

        if looks_like_glob {
            // The literal-existence shortcut only applies when the argument
            // contains `[` but NOT `*` or `?`.  Arguments with `*`/`?` are
            // almost certainly intentional glob patterns (e.g. `*.mp3`) and
            // should always be expanded.  The original bug was specifically
            // about `[` in real filenames like `Song [Live].mp3`; treating
            // `*`/`?`-containing args as potential literal paths would prevent
            // glob expansion in the common case where a file by that name
            // (e.g. `*.mp3`) happens to exist on disk.
            //
            // Uses `symlink_metadata` (does NOT follow symlinks) so that
            // dangling symlinks are detected as present rather than being
            // routed into the glob engine where brackets would be
            // misinterpreted.  Any non-`NotFound` error (e.g. `PermissionDenied`)
            // is treated conservatively as "exists" to prefer a downstream
            // open-error over silently dropping the argument.
            let exists_literally = !has_wildcards
                && match std::fs::symlink_metadata(arg) {
                    Ok(_) => {
                        log::debug!(
                            "Argument '{arg}' contains '[' but a filesystem \
                             entry with that exact name exists; treating it \
                             as a literal path."
                        );
                        true
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
                    Err(e) => {
                        log::warn!(
                            "Could not stat '{arg}' ({e}); \
                             treating it as a literal path."
                        );
                        true
                    }
                };

            if exists_literally {
                result.push(arg.to_string());
            } else {
                match glob::glob(arg) {
                    Ok(paths) => {
                        let mut matched = false;
                        for entry in paths {
                            match entry {
                                Ok(path) => {
                                    if let Some(s) = path.to_str() {
                                        result.push(s.to_string());
                                        matched = true;
                                    }
                                }
                                Err(e) => log::warn!("Glob error for pattern '{arg}': {e}"),
                            }
                        }
                        if !matched {
                            log::warn!("No files matched pattern '{arg}'");
                        }
                    }
                    Err(e) => {
                        log::warn!("Invalid glob pattern '{arg}': {e}");
                        result.push(arg.to_string());
                    }
                }
            }
        } else {
            result.push(arg.to_string());
        }
    }
    result
}

/// Find the MIME type (ie. `image/[bmp|gif|jpeg|png|tiff`) based on the file extension. Not perfect, but it'll do for now.
///
/// # Errors
///
/// - If we can't infer the file type from path, we give an error
pub fn get_mime_type(filename: &str) -> Result<String> {
    // Read the file and check the mime type
    let Some(file_type) = infer::get_from_path(filename)? else {
        bail!("File type not supported");
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
pub fn get_file_type(filename: &str) -> Result<FileTypes> {
    // return the file type
    let file_type = infer::get_from_path(filename)?;
    log::debug!("File type = {file_type:?}");
    let Some(file_type) = file_type else {
        bail!("File type not supported");
    };

    let ft;

    if file_type.matcher_type() == MatcherType::Audio
        || file_type.matcher_type() == MatcherType::Video
    {
        ft = audio_file_type(file_type);
        log::debug!("File type is {ft}");
    } else {
        log::debug!("File type is not a recognized audio format. Trying MP4 variants.");
        let mp4vec: [&str; 2] = ["mp4a", "mp4b"];
        let ext = file_type.extension().to_lowercase();
        if mp4vec.contains(&ext.as_str()) {
            ft = FileTypes::M4A;
        } else {
            bail!("File type not supported");
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
        "audio/m4a" | "video/mp4" => FileTypes::M4A,
        _ => FileTypes::Unknown,
    }
}

/// Checks that the new filename pattern results in a unique file.
///
/// Not perfect since the track title can occur multiple times on the same album.
/// TODO: Make this better. Include a check for the disc number and track title combo, for example.
///
/// # Errors
///
/// - Return an error if the pattern provided is unlikely to return unique file names
#[must_use]
pub fn file_rename_pattern_not_ok(pattern: &str) -> bool {
    !pattern.contains("%tn")
        && !pattern.contains("%tt")
        && !pattern.contains("%track-number")
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
pub fn split_val(value: &str) -> Result<(u16, u16)> {
    let split_str: Vec<&str>;
    if value.contains("of") {
        split_str = value.split("of").collect();
    } else if value.contains('/') {
        split_str = value.split('/').collect();
    } else {
        bail!("Split pattern not found.");
    }

    let count = split_str[0]
        .trim()
        .parse::<u16>()
        .with_context(|| format!("Unable to parse count '{}'", split_str[0].trim()))?;
    let total = split_str[1]
        .trim()
        .parse::<u16>()
        .with_context(|| format!("Unable to parse total '{}'", split_str[1].trim()))?;

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
/// `Result<String>` - a formatted string with the number of files found, or an error if something went wrong.
///
/// # Errors
/// - Returns an error if unable to get the directory name from the fielname.
///
/// # Panics
/// None.
pub fn count_files(filename: &str) -> Result<String> {
    let ext = get_extension(filename);

    // Get just the directory part, excluding the filename
    let mut dir = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    if dir.as_os_str().is_empty() {
        dir = Path::new(&".");
    }

    if !dir.is_dir() {
        bail!("Unable to get directory name from filename {filename}.");
    }

    // Get the list of (music) files in the directory
    let file_list = std::fs::read_dir(Path::new(dir))?
        .filter_map(std::result::Result::ok)
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
    log::debug!("count = {count}");

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
///
/// This is used to ensure uniqueness of file names.
/// This can be changed to something else later without impacting the main application.
/// For example, one could switch to a random number generator or something.
///
/// # Panics
///
/// If the time appears to have gone backwards. Or it's after 03:14:07 UTC on 19 January 2038 (the Epochalypse).
/// Either case is highly unlikely. If you need to worry about this, you have bigger problems.
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
/// use common::thousand_separated;
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
/// `Result<std::path::PathBuf>` - a `PathBuf` containing the full directory path to the file if succcessful.
///
/// # Errors
///
/// `canonicalize` has a problem.
///
/// # Example
/// `get_full_path_directory("/some/path/myfile.txt")` returns "/some/path/"
pub fn directory(filename: &str) -> Result<std::path::PathBuf> {
    let mut music_file_path = std::fs::canonicalize(filename)?;
    music_file_path = music_file_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    log::debug!("music_file_path = {}", music_file_path.display());

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

    /// Drop guard that removes a file/symlink on scope exit, including on panic.
    struct TempPathGuard(std::path::PathBuf);
    impl Drop for TempPathGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    #[test]
    /// Returns the mime type based on the file name
    fn test_get_mime_type() {
        // Skip if testdata is not available (e.g. in CI without LFS files)
        if !Path::new("../testdata/sample.mp3").exists() {
            return;
        }
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
        // Skip if testdata is not available (e.g. in CI without LFS files)
        if !Path::new("../testdata/sample.ape").exists() {
            return;
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

    #[test]
    fn test_expand_file_args() {
        // Non-glob args are passed through as-is
        let args = vec!["file1.txt", "file2.txt"];
        let result = expand_file_args(args.into_iter());
        assert_eq!(result, vec!["file1.txt", "file2.txt"]);

        // Skip testdata-dependent assertions if files are not available (e.g. in CI)
        if !Path::new("../testdata/sample.flac").exists() {
            return;
        }

        // Glob patterns that match files should expand
        let args = vec!["../testdata/sample.*"];
        let result = expand_file_args(args.into_iter());
        assert!(result.len() > 1);
        assert!(result.iter().any(|f| f.ends_with(".flac")));
        assert!(result.iter().any(|f| f.ends_with(".mp3")));

        // Glob patterns that match nothing produce no entries
        let args = vec!["../testdata/nonexistent_*.xyz"];
        let result = expand_file_args(args.into_iter());
        assert!(result.is_empty());

        // Mixed glob and non-glob args
        let args = vec!["plain.txt", "../testdata/sample.*"];
        let result = expand_file_args(args.into_iter());
        assert_eq!(result[0], "plain.txt");
        assert!(result.len() > 2);
    }

    /// A literal filename that contains `[` and `]` must not be fed through the
    /// glob engine — the brackets would be misinterpreted as a character class
    /// and the file would be silently dropped.
    #[test]
    fn test_expand_file_args_brackets_in_literal_filename() {
        // Include the process ID in the name to avoid collisions when multiple
        // test processes run concurrently on the same machine.
        let pid = std::process::id();
        let dir = std::env::temp_dir();
        let path = dir.join(format!("Song [Live Version] {pid}.mp3"));
        let _ = std::fs::remove_file(&path);
        std::fs::File::create(&path).expect("create temp file");

        // `expand_file_args` only accepts `&str`, so skip the test if the temp
        // directory path is not valid UTF-8 rather than panicking.
        let Some(path_str_owned) = path.to_str().map(str::to_owned) else {
            let _ = std::fs::remove_file(&path);
            return;
        };
        // Guard removes the file even if the assertion panics.
        let _guard = TempPathGuard(path);

        let result = expand_file_args(std::iter::once(path_str_owned.as_str()));
        assert_eq!(
            result,
            vec![path_str_owned],
            "literal file with brackets in name was silently dropped"
        );
    }

    /// A glob pattern that legitimately uses `[` together with `*` must still
    /// expand correctly when no literal file by that name exists.
    #[test]
    fn test_expand_file_args_bracket_glob_pattern_still_works() {
        // Skip if testdata is not available
        if !Path::new("../testdata/sample.flac").exists() {
            return;
        }

        // Pattern uses both `[` and `*` — there is no literal file with this name
        let args = vec!["../testdata/sample.[fm]*"];
        let result = expand_file_args(args.into_iter());
        // Should match sample.flac, sample.mp3, sample.m4a at minimum
        assert!(
            !result.is_empty(),
            "bracket+wildcard glob pattern expanded nothing"
        );
        assert!(result.iter().any(|f| f.ends_with(".flac")));
        assert!(result.iter().any(|f| f.ends_with(".mp3")));
    }

    /// A bracket-only glob pattern (no `*` or `?`) must still be routed through
    /// the glob engine when no literal file by that name exists.  This guards
    /// against the `arg.contains('[')` arm being accidentally dropped from the
    /// `looks_like_glob` condition in a future refactor.
    #[test]
    fn test_expand_file_args_bracket_only_glob() {
        // Skip if testdata is not available
        if !Path::new("../testdata/sample.flac").exists() {
            return;
        }

        // Pattern has ONLY `[` — no `*` or `?`.  There is no literal file with
        // this name, so it must be expanded by the glob engine.
        let args = vec!["../testdata/sample.[Ff]lac"];
        let result = expand_file_args(args.into_iter());
        assert!(
            !result.is_empty(),
            "bracket-only glob pattern expanded nothing (arg.contains('[') may have been dropped)"
        );
        assert!(result.iter().any(|f| f.ends_with(".flac")));
    }

    /// A `?`-only glob pattern (no `*` or `[`) must be routed through the glob
    /// engine.  This guards against `?` being accidentally dropped from the
    /// `has_wildcards` check in a future refactor — such a drop would cause
    /// `?`-patterns to fall through to literal passthrough silently.
    #[test]
    fn test_expand_file_args_question_mark_glob() {
        // Skip if testdata is not available
        if !Path::new("../testdata/sample.flac").exists() {
            return;
        }

        // Pattern uses ONLY `?` — no `*` or `[`.  No literal file with this
        // name exists, so it must be expanded by the glob engine.
        let args = vec!["../testdata/sample.?lac"];
        let result = expand_file_args(args.into_iter());
        assert!(
            !result.is_empty(),
            "question-mark-only glob pattern expanded nothing (? may have been \
             dropped from has_wildcards)"
        );
        assert!(result.iter().any(|f| f.ends_with(".flac")));
    }

    /// A dangling symlink whose name contains `[` must be passed through as a
    /// literal path, not silently dropped by the glob engine.  `Path::exists()`
    /// follows symlinks and returns `false` for a dangling target, which would
    /// cause the original code to misinterpret the brackets as a character class.
    #[cfg(unix)]
    #[test]
    fn test_expand_file_args_dangling_symlink_with_brackets() {
        use std::os::unix::fs::symlink;

        // Include the process ID in both names to avoid collisions when multiple
        // test processes run concurrently on the same machine.
        let pid = std::process::id();
        let dir = std::env::temp_dir();
        let link_path = dir.join(format!("Song [Demo] {pid}.mp3"));
        // Ensure the target path definitely does not exist so the symlink is
        // truly dangling (a pre-existing target would make it non-dangling and
        // invalidate what the test is asserting).
        let missing_target = dir.join(format!("nonexistent_target_{pid}_xyzzy.mp3"));
        let _ = std::fs::remove_file(&missing_target);

        // Remove any stale link from a previous run, then create a fresh dangling symlink.
        let _ = std::fs::remove_file(&link_path);
        symlink(&missing_target, &link_path).expect("create dangling symlink");
        // `expand_file_args` only accepts `&str`, so skip the test if the temp
        // directory path is not valid UTF-8 rather than panicking.
        let Some(link_str_owned) = link_path.to_str().map(str::to_owned) else {
            let _ = std::fs::remove_file(&link_path);
            return;
        };
        // Guard removes the symlink even if the assertion or expand_file_args panics.
        let _guard = TempPathGuard(link_path);
        let result = expand_file_args(std::iter::once(link_str_owned.as_str()));
        assert_eq!(
            result,
            vec![link_str_owned],
            "dangling symlink with brackets in name was silently dropped"
        );
    }
}
