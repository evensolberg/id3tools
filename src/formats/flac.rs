//! Various file format parsers. The different types of file formats (ie. APE, FLAC, MP3, MP4)
//! all reside under this crate, so they don't have to be exposed to the main body of code.git t

use crate::default_values::DefaultValues;
use crate::shared;
use metaflac::block::PictureType::{CoverBack, CoverFront};
use metaflac::Tag;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Process a FLAC file with tags and images.
///
/// **Parameters:**
///
/// - `filename: &str` -- The name of the file to be processed, eg. "somefile.flac".
/// - `new_tags: &HashMap<String, String>` -- A set of new tags in Key/Value form, eg. _key = ALBUMARTIST_, _value = "The Tragically Hip"_
/// - `config: &DefaulValues` -- A struct containing default values read from a config file and the CLI
///
/// **Returns:**
///
/// `Result<(), Box<dyn Error>>` -- Nothing except `Ok` if things go well, otherwise an error.
///
/// **Example:**
///
/// `process_flac("somefile.flac", &my_tags, &my_config)?;`
pub fn process_flac(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    let mut tags = Tag::read_from_path(&filename)?;
    log::debug!("Filename: {}", filename);

    // Output existing blocks
    for block in tags.blocks() {
        log::trace!("{:?}", block);
    }

    // Read old tags
    if let Some(id3) = tags.vorbis_comments() {
        log::debug!("vendor_string = {}", &id3.vendor_string);
        for (key, values) in &id3.comments {
            for value in values {
                log::debug!("Old {} = {}", key, value);
            }
        }
    }

    // Set new tags
    for (key, value) in new_tags {
        if !(config.detail_off.unwrap_or(false)) {
            log::info!("{} :: New {} = {}", &filename, key, value);
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "PICTUREFRONT" => {
                log::debug!("Setting front cover.");
                match add_picture(&mut tags, value, CoverFront) {
                    Ok(_) => log::trace!("Picture set."),
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set front cover to {}. Error message: {}",
                                value,
                                err.to_string()
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set front cover to {}. Continuing. Error message: {}",
                                value,
                                err.to_string()
                            );
                        }
                    }
                } // match
            } // PICTUREFRONT
            "PICTUREBACK" => {
                log::debug!("Setting back cover.");
                match add_picture(&mut tags, value, CoverBack) {
                    Ok(_) => log::trace!("Picture set."),
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set back cover to {}. Error message: {}",
                                value,
                                err.to_string()
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set back cover to {}. Error message: {}",
                                value,
                                err.to_string()
                            );
                        }
                    }
                } // match
            } // PICTUREBACK
            _ => tags.set_vorbis(key.clone(), vec![value.clone()]),
        } // match key.as_ref()
    }

    // Try to save
    if !config.dry_run.unwrap_or(true) {
        log::debug!("Attempting to save file {}", filename);
        tags.save()?;
    } else {
        log::debug!("Dry-run. Not saving.");
    }

    log::debug!("Picture count: {}", tags.pictures().count());

    // Return safely
    Ok(())
}

/// Set the front or back cover (for now)
fn add_picture(
    tags: &mut metaflac::Tag,
    value: &str,
    cover_type: metaflac::block::PictureType,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Removing existing picture.");
    tags.remove_picture_type(cover_type);

    // Read the file and check the mime type
    let mime_fmt = shared::mime_type(&value)?;
    log::debug!("MIME type: {}", mime_fmt);
    log::debug!("Reading image file {}", value);
    let data = fs::read(value)?;
    log::debug!("Attempting to set picture.");
    tags.add_picture(mime_fmt, cover_type, data);

    // Return safely
    Ok(())
}
