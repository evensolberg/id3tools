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
    let mut tags = Tag::read_from_path(&filename).unwrap();

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
        if !(config.detail_off.unwrap()) {
            log::info!("{} :: New {} = {}", &filename, key, value);
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "PICTUREFRONT" => {
                log::debug!("Setting front cover.");
                add_picture(&mut tags, value, CoverFront)?;
            }
            "PICTUREBACK" => {
                log::debug!("Setting back cover.");
                add_picture(&mut tags, value, CoverBack)?;
            }
            _ => tags.set_vorbis(key.clone(), vec![value.clone()]),
        }
    }

    // Try to save
    if !(config.dry_run.unwrap()) {
        log::debug!("Attempting to save file {}", filename);
        tags.save()?;
    } else {
        log::debug!("Dry-run. Not saving.")
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
    log::debug!("Reading image file {}", value);

    // Read the file and check the mime type
    let mime_fmt = shared::mime_type(value)?;
    log::debug!("Image format: {}", mime_fmt);
    log::debug!("Setting picture to {}", value);
    tags.add_picture(mime_fmt, cover_type, fs::read(&value)?);

    // Return safely
    Ok(())
}
