//! Contains the functionality to process APE files.
//! KEY: <https://wiki.hydrogenaud.io/index.php?title=APE_key>

use crate::default_values::DefaultValues;
use crate::formats::images::read_cover;
use anyhow::{Context, Result};
use ape::{self, Item, ItemType};
use std::{collections::HashMap, fs::File};

/// Performs the actual processing of APE files.
pub fn process(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool> {
    let mut processed_ok = false;
    let mut tags = ape::read_from_path(filename)?;

    // Set new tags
    for (key, value) in new_tags {
        if config.execution.detail_off.unwrap_or(false) {
            log::debug!("{filename} :: New {key} = {value}");
        } else if config.execution.dry_run.unwrap_or(false) {
            log::info!("{filename} :: New {key} = {value}");
        } else {
            log::debug!("{filename} :: New {key} = {value}");
        }

        // Process the tags
        match key.as_ref() {
            "PICTUREFRONT" | "PICTUREBACK" => {
                let ape_key = if key == "PICTUREFRONT" {
                    "Cover Art (Front)"
                } else {
                    "Cover Art (Back)"
                };

                let max_size = config.pictures.picture_max_size.unwrap_or(0);
                match set_picture(&mut tags, value.trim(), ape_key, max_size) {
                    Ok(()) => log::debug!("{ape_key} set for {filename}."),
                    Err(err) => {
                        if config.execution.stop_on_error.unwrap_or(true) {
                            return Err(err)
                                .context(format!("Unable to set {ape_key} to {value}"));
                        }
                        log::error!("Unable to set {ape_key} to {value}. Continuing: {err:#}");
                    }
                }
            }

            _ => {
                let item = Item::new(
                    key.as_str(),
                    ItemType::Text,
                    value.trim().as_bytes().to_vec(),
                );
                match item {
                    Ok(item) => {
                        tags.set_item(item);
                    }
                    Err(err) => {
                        if config.execution.stop_on_error.unwrap_or(true) {
                            return Err(err).context(format!("Unable to set {key} to {value}"));
                        }
                        log::error!("Unable to set {key} to {value}: {err:#}");
                    }
                }
            }
        } // match key.as_ref()
    }

    // Try to save
    if !config.execution.dry_run.unwrap_or(true) {
        let mut file = File::options().read(true).write(true).open(filename)?;
        match ape::write_to(&tags, &mut file) {
            Ok(()) => {
                processed_ok = true;
                log::info!("{filename}  ✓");
            }
            Err(e) => {
                log::error!("{filename}: Failed to write APE tags: {e}");
            }
        }
    }

    if config.rename_file.is_some() {
        rename_file(filename, config, &tags)?;
        processed_ok = true;
    }

    // return safely
    Ok(processed_ok)
}

/// Sets the front or back cover art in an APE tag.
/// APE cover art convention: key is "Cover Art (Front)" or "Cover Art (Back)",
/// value is a binary item with format: `description\0` + raw image bytes.
fn set_picture(tags: &mut ape::Tag, img_file: &str, ape_key: &str, max_size: u32) -> Result<()> {
    // Remove existing cover art with this key
    let _ = tags.remove_items(ape_key);

    let (img, mime_type) = read_cover(img_file, max_size)?;
    log::debug!(
        "set_picture::Image {img_file} read. Length = {}, mime = {mime_type}",
        img.len()
    );

    // APE binary cover format: "description\0" prefix followed by raw image bytes
    let mut binary_data = Vec::new();
    binary_data.extend_from_slice(b"\0"); // empty description + null terminator
    binary_data.extend_from_slice(&img);

    let item = Item::new(ape_key, ItemType::Binary, binary_data)?;
    tags.set_item(item);

    Ok(())
}

/// Renames the APE file based on the tags
// Allow the uneccesary Ok(()) for now for consistency with other functions and possible changes later.
#[allow(clippy::unnecessary_wraps)]
fn rename_file(_filename: &str, _config: &DefaultValues, _tags: &ape::Tag) -> Result<()> {
    log::warn!(
        "Rename is currently not supported for APE files because the metadata is not standardized."
    );

    // Return safely
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_file() {
        // Skip if testdata is not available (e.g. in CI without LFS files)
        if !std::path::Path::new("../testdata/sample.ape").exists() {
            return;
        }
        let blank_defaults = DefaultValues::new();
        let blank_ape = ape::Tag::default();

        // Don't need to do a lot because this function just issues a warning and exits without error.
        assert!(rename_file("../testdata/sample.ape", &blank_defaults, &blank_ape).is_ok());
    }

    #[test]
    fn test_process_ape() {
        // Skip if testdata is not available (e.g. in CI without LFS files)
        if !std::path::Path::new("../testdata/sample.ape").exists() {
            return;
        }
        let mut new_values = HashMap::<String, String>::new();
        new_values.insert("ALBUMARTIST".to_string(), "New Album Artist".to_string());
        let blank_defaults = DefaultValues::new();

        assert!(process("../testdata/sample.ape", &new_values, &blank_defaults).is_ok());
    }
}
