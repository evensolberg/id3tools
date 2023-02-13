//! Contains the functionality to process APE files.
//! KEY: <https://wiki.hydrogenaud.io/index.php?title=APE_key>

use crate::default_values::DefaultValues;
// use crate shared; // for add_pictures
use ape::{self, Item};
use std::{collections::HashMap, error::Error, fs::File};

/// Performs the actual processing of APE files.
pub fn process(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    let mut processed_ok = false;

    let mut tags = ape::read_from_path(filename)?;
    for item in tags.iter() {
        log::debug!("Old {} = {:?}", item.key, item.value);
    }

    // Set new tags
    for (key, value) in new_tags {
        if config.detail_off.unwrap_or(false) {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        } else if config.dry_run.unwrap_or(false) {
            log::info!("{} :: New {} = {}", &filename, key, value);
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "PICTUREFRONT" | "PICTUREBACK" => {
                log::warn!("Setting covers in APE files is currently not supported.");
            }

            _ => {
                let item = Item::from_text(key, value.trim());
                match item {
                    Ok(item) => {
                        log::debug!("Item created: {:?}", item);
                        tags.set_item(item);
                    }
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set {key} to {value}. Error message: {err}"
                            )
                            .into());
                        }
                        log::error!("Unable to set {} to {}. Error message: {}", key, value, err);
                    }
                }
            }
        } // match key.as_ref()
    }

    // Try to save
    if config.dry_run.unwrap_or(true) {
        log::debug!("Dry-run. Not saving.");
    } else {
        let mut file = File::open(filename)?;
        let res = ape::write_to(&tags, &mut file);
        if res.is_ok() {
            processed_ok = true;
            log::info!("{}  ✓", filename);
        }
    }

    if config.rename_file.is_some() {
        rename_file(filename, config, &tags)?;
        processed_ok = true;
    }

    // return safely
    Ok(processed_ok)
}

/// Renames the APE file based on the tags
fn rename_file(
    _filename: &str,
    _config: &DefaultValues,
    _tags: &ape::Tag,
) -> Result<(), Box<dyn Error>> {
    log::warn!(
        "Rename is currently not supported for APE files because the metadata is not standardized."
    );

    // Return safely
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[assay(include = ["../testdata/sample.ape"])]
    fn test_rename_file() {
        let blank_defaults = DefaultValues::new();
        let blank_ape = ape::Tag::default();

        // Don't need to do a lot because this function just issues a warning and exits without error.
        assert!(rename_file("../testdata/sample.ape", &blank_defaults, &blank_ape).is_ok());
    }

    #[assay(include = ["../testdata/sample.ape"])]
    fn test_process_ape() {
        let mut new_values = HashMap::<String, String>::new();
        new_values.insert("ALBUMARTIST".to_string(), "New Album Artist".to_string());
        let blank_defaults = DefaultValues::new();

        assert!(process("../testdata/sample.ape", &new_values, &blank_defaults).is_ok());
    }
}
