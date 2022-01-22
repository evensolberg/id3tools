//! Contains the functionality to process APE files.

use crate::default_values::DefaultValues;
// use crate shared; // for add_pictures
use ape::{self, Item};
use std::{collections::HashMap, error::Error, fs::File};

/// Performs the actual processing of APE files.
pub fn process_ape(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    let mut tags = ape::read_from_path(&filename)?;
    for item in tags.iter() {
        log::debug!("Old {} = {:?}", item.key, item.value);
    }

    // Set new tags
    for (key, value) in new_tags {
        if !(config.detail_off.unwrap_or(false)) {
            if config.dry_run.unwrap_or(false) {
                log::info!("{} :: New {} = {}", &filename, key, value);
            }
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "PICTUREFRONT" | "PICTUREBACK" => {
                log::warn!("Setting covers on APE files is currently not supported.");
            } // PICTUREBACK
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
                                "Unable to set {} to {}. Error message: {}",
                                key, value, err
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set {} to {}. Error message: {}",
                                key,
                                value,
                                err
                            );
                        }
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
        ape::write_to(&tags, &mut file)?;
        log::info!("{}  ✓", filename);
    }

    if config.rename_file.is_some() {
        rename_ape(filename, config, tags)?;
    }

    // return safely
    Ok(())
}

// /// Set the front or back cover (for now)
// fn add_picture(
//     tags: &mut ape::Tag,
//     key: &str,
//     value: &str,
//     config: &DefaultValues,
// ) -> Result<(), Box<dyn Error>> {
//     log::debug!("Removing existing picture if it exists.");
//     // If it exists
//     tags.remove_item("cover");
//     // Read the file and check the mime type
//     let mime_fmt = shared::mime_type(&value)?;
//     log::debug!("MIME type: {}", mime_fmt);
//     log::debug!("Reading image file {}", value);
//     let data = std::fs::read(value)?;
//     log::debug!("Attempting to set picture.");
//     let item = Item::from_binary(value, data);
//     match item {
//         Ok(item) => {
//             log::debug!("Item created: {:?}", item);
//             tags.set_item(item);
//         }
//         Err(err) => {
//             if config.stop_on_error.unwrap_or(true) {
//                 return Err(format!(
//                     "Unable to set {} to {}. Error message: {}",
//                     key,
//                     value,
//                     err
//                 )
//                 .into());
//             } else {
//                 log::error!(
//                     "Unable to set {} to {}. Error message: {}",
//                     key,
//                     value,
//                     err
//                 );
//             }
//         }
//     }
//     // Return safely
//     Ok(())
// }

/// Renames the APE file based on the tags
fn rename_ape(
    _filename: &str,
    _config: &DefaultValues,
    _tags: ape::Tag,
) -> Result<(), Box<dyn Error>> {
    log::warn!(
        "Rename is currently not supported for APE files because the metadata is not standardized."
    );

    // Return safely
    Ok(())
}
