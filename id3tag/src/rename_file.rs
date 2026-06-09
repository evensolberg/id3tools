use anyhow::{bail, Context, Result};
use std::{collections::HashMap, path::Path};

use crate::default_values::DefaultValues;

/// Renames the file provided based on the pattern provided.
///
/// **Parameters:**
///
/// - `filename: &str` -- the name of the file to be renamed
/// - `tags: &HashMap<String, String>` -- The various tag values (ie. Album Artist, Genre, etc.)
/// - `pattern: &str` -- the tag pattern for the new filename. This has been validated to be OK by the CLI.
/// - `config: &DefaultValues` -- The tags that have been set, and any config settings such as dry-run.
///
/// Note that you'll need to populate the tags struct _before_ using this function. This is to avoid having to re-open the file and re-read the data.
///
/// **Returns**
/// - The new file name if successful
/// - An error message if unsuccessful.
///
/// **Example:**
///
/// ```
/// let res = rename_file("somefile.flac", "%dn-%tn %tt", &config)?;
/// ```
///
/// This will rename the file based on disc number (`%dn`), track number (`%tn`) and track title (%tt).
pub fn rename_file(
    filename: &str,
    tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<String> {
    // Check if there is a rename pattern
    let mut new_filename = if let Some(nfn) = &config.rename_file {
        nfn.clone()
    } else {
        bail!("No filename pattern presented. Unable to continue.");
    };

    // Check if any tag used in the pattern is empty. If so, skip the rename.
    for (key, value) in tags {
        if new_filename.contains(key.as_str()) && value.trim().is_empty() {
            log::warn!(
                "Tag '{key}' is used in rename pattern but has no value. Skipping rename for '{filename}'."
            );
            return Ok(filename.to_string());
        }
    }

    // These tags (may) need to be padded with leading zeros.
    let pad_tags: [&str; 8] = [
        "%dn",
        "%dt",
        "%tn",
        "%to",
        "%disc-number",
        "%disc-number-total",
        "%track-number",
        "%track-number-total",
    ];

    // replace any options (eg. %aa, %tg) with the corresponding tag
    for (key, value) in tags {
        let mut fixed_value = value.trim().to_string();

        // Make sure to pad disc and track numbers with leading zeros.
        if pad_tags.contains(&key.as_str()) {
            fixed_value = format!("{fixed_value:0>2}");
        }

        new_filename = new_filename.replace(key, &fixed_value);
    }

    // Fix a few things we know will give us trouble later.
    new_filename = clean_filename(&new_filename);

    // Get the path in front of the filename (eg. "music/01.flac" returns "music/")
    let parent = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Create the new filename
    let mut new_path =
        parent.join(Path::new(&new_filename).with_extension(common::get_extension(filename)));

    // Return if the new filename is the same as the old
    let np = new_path.to_string_lossy().to_string();
    if np == *filename {
        return Ok(np);
    }

    // If the target already exists, append a unique suffix to avoid collisions.
    // Loop to handle the unlikely case where the suffixed name also exists.
    while Path::new(&new_path).exists() {
        let unique_val = common::get_unique_value();

        log::warn!("{new_filename} already exists. Appending unique identifier ({unique_val}).");
        new_filename = format!("{new_filename} ({unique_val:0>4})");
        new_path =
            parent.join(Path::new(&new_filename).with_extension(common::get_extension(filename)));
    }

    let npl = new_path.to_string_lossy();

    // Perform the actual rename and check the outcome
    if config.execution.dry_run.unwrap_or(true) {
        log::debug!("dr: {filename} --> {}", new_path.display());
    } else {
        // Get parent dir
        match std::fs::rename(filename, &new_path) {
            Ok(()) => log::debug!("{filename} --> {npl}"),
            Err(err) => {
                if config.execution.stop_on_error.unwrap_or(true) {
                    return Err(err).with_context(|| format!("Unable to rename {filename} to {npl}"));
                }
                log::warn!("Unable to rename {filename} to {npl}: {err:#}");
            }
        }
    }

    // return safely
    Ok(npl.into_owned())
}

fn clean_filename(filename: &str) -> String {
    let mut new_filename = filename.to_string();
    // Replace characters that have reasonable substitutions
    new_filename = new_filename.replace('/', "-");
    new_filename = new_filename.replace('\\', "-");
    new_filename = new_filename.replace(':', " -");
    new_filename = new_filename.replace('|', "-");
    new_filename = new_filename.replace('\t', " ");
    // Remove characters that have no good substitution
    for ch in ['\0', '?', '*', '"', '<', '>', '\n', '\r'] {
        new_filename = new_filename.replace(ch, "");
    }
    // Collapse multiple spaces into one
    while new_filename.contains("  ") {
        new_filename = new_filename.replace("  ", " ");
    }
    new_filename = new_filename.trim_matches('.').to_string();
    new_filename = new_filename.trim().to_string();
    new_filename
}

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
//
mod tests {
    use super::*;

    #[test]
    fn test_clean_filename() {
        assert_eq!(clean_filename("my/long.file:name"), "my-long.file -name");
        assert_eq!(clean_filename("Dr. Dre"), "Dr. Dre");
        assert_eq!(clean_filename(".hidden."), "hidden");
        assert_eq!(clean_filename("back\\slash"), "back-slash");
        assert_eq!(clean_filename("null\0byte"), "nullbyte");
        assert_eq!(clean_filename("what?"), "what");
        assert_eq!(clean_filename("wild*card"), "wildcard");
        assert_eq!(clean_filename("say\"hello\""), "sayhello");
        assert_eq!(clean_filename("<tag>"), "tag");
        assert_eq!(clean_filename("pipe|line"), "pipe-line");
        assert_eq!(clean_filename("tab\there"), "tab here");
        assert_eq!(clean_filename("new\nline"), "newline");
        assert_eq!(clean_filename("cr\rreturn"), "crreturn");
        assert_eq!(clean_filename("too   many  spaces"), "too many spaces");
    }

    #[test]
    fn test_rename_file() {
        let mut config = DefaultValues::new();
        config.rename_file = Some("%aa - %at".to_string());
        config.execution.dry_run = Some(true);

        let mut tags = HashMap::new();
        tags.insert("%aa".to_string(), "AlbumArtist".to_string());
        tags.insert("%at".to_string(), "AlbumTitle".to_string());

        assert_eq!(
            rename_file("../testdata/sample.flac", &tags, &config).unwrap(),
            "../testdata/AlbumArtist - AlbumTitle.flac"
        );
    }

    #[test]
    fn test_rename_skip_blank_text_tag() {
        let mut config = DefaultValues::new();
        config.rename_file = Some("%aa - %at".to_string());
        config.execution.dry_run = Some(true);

        let mut tags = HashMap::new();
        tags.insert("%aa".to_string(), String::new()); // blank
        tags.insert("%at".to_string(), "AlbumTitle".to_string());

        // Should return original filename since %aa is blank and used in pattern
        assert_eq!(
            rename_file("../testdata/sample.flac", &tags, &config).unwrap(),
            "../testdata/sample.flac"
        );
    }

    #[test]
    fn test_rename_skip_blank_numeric_tag() {
        let mut config = DefaultValues::new();
        config.rename_file = Some("%dn-%tn %tt".to_string());
        config.execution.dry_run = Some(true);

        let mut tags = HashMap::new();
        tags.insert("%dn".to_string(), "1".to_string());
        tags.insert("%tn".to_string(), String::new()); // blank
        tags.insert("%tt".to_string(), "Track Title".to_string());

        // Should return original filename since %tn is blank and used in pattern
        assert_eq!(
            rename_file("../testdata/sample.flac", &tags, &config).unwrap(),
            "../testdata/sample.flac"
        );
    }

    #[test]
    fn test_rename_ok_unused_blank_tag() {
        let mut config = DefaultValues::new();
        config.rename_file = Some("%dn-%tn %tt".to_string());
        config.execution.dry_run = Some(true);

        let mut tags = HashMap::new();
        tags.insert("%dn".to_string(), "1".to_string());
        tags.insert("%tn".to_string(), "3".to_string());
        tags.insert("%tt".to_string(), "Track Title".to_string());
        tags.insert("%aa".to_string(), String::new()); // blank but not in pattern

        // Should proceed with rename since %aa is not used in the pattern
        assert_eq!(
            rename_file("../testdata/sample.flac", &tags, &config).unwrap(),
            "../testdata/01-03 Track Title.flac"
        );
    }
}
