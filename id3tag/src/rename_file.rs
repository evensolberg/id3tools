use std::{collections::HashMap, error::Error, path::Path};

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
) -> Result<String, Box<dyn Error>> {
    // Check if there is a rename pattern
    let mut new_filename = if let Some(nfn) = &config.rename_file {
        nfn.to_string()
    } else {
        return Err("No filename pattern presented. Unable to continue.".into());
    };

    // These tags (may) need to be padded with leading zeros.
    let pad_tags = vec![
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
        // Make sure to pad disc and track numbers with leading zeros
        let mut fixed_value = value.clone();
        fixed_value = fixed_value.trim().to_string();

        if pad_tags.contains(&key.as_str()) {
            fixed_value = format!("{:0>2}", value.trim());
        }

        // Do the actual filename replacement
        if fixed_value.is_empty() {
            log::warn!("Tag '{key}' is empty.");
            fixed_value = "unknown".to_string();
        }
        new_filename = new_filename.replace(key, &fixed_value);
    }

    // Fix a few things we know will give us trouble later.
    new_filename = new_filename.replace('/', "-");
    new_filename = new_filename.replace(':', " -");
    new_filename = new_filename.replace('.', "");

    // Remove leading or trailing spaces
    new_filename = new_filename.trim().to_string();

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

    // Check if a file with the new filename already exists - make the filename unique if it does.
    if Path::new(&new_path).exists() {
        let unique_val = common::get_unique_value();

        log::warn!("{new_filename} already exists. Appending unique identifier.");
        new_filename = format!("{new_filename} ({unique_val:0>4})");
        new_path =
            parent.join(Path::new(&new_filename).with_extension(common::get_extension(filename)));
    }

    // Perform the actual rename and check the outcome
    if config.dry_run.unwrap_or(true) {
        log::debug!("dr: {filename} --> {}", new_path.display());
    } else {
        // Get parent dir
        let rn_res = std::fs::rename(filename, &new_path);
        match rn_res {
            Ok(_) => log::debug!("{filename} --> {}", new_path.to_string_lossy()),
            Err(err) => {
                if config.stop_on_error.unwrap_or(true) {
                    return Err(format!(
                        "Unable to rename {filename} to {}. Error message: {err}",
                        new_path.to_string_lossy()
                    )
                    .into());
                }
                log::warn!(
                    "Unable to rename {filename} to {}. Error message: {err}",
                    new_path.to_string_lossy()
                );
            }
        }
    }

    // return safely
    let result = new_path.to_string_lossy().into_owned();
    Ok(result)
}

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
//
mod tests {
    use super::*;

    #[test]
    fn test_rename_file() {
        let mut config = DefaultValues::new();
        config.rename_file = Some("%aa - %at".to_string());
        config.dry_run = Some(true);

        let mut tags = HashMap::new();
        tags.insert("%aa".to_string(), "AlbumArtist".to_string());
        tags.insert("%at".to_string(), "AlbumTitle".to_string());

        assert_eq!(
            rename_file("../testdata/sample.flac", &tags, &config).unwrap(),
            "../testdata/AlbumArtist - AlbumTitle.flac"
        );
    }
}
