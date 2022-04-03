//! Contains the functionality to process FLAC files.

use crate::default_values::DefaultValues;
use crate::formats::tags;
use crate::formats::FileTypes;
use crate::rename_file;
use metaflac::block::PictureType::{CoverBack, CoverFront};
use metaflac::Tag;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Performs the actual processing of FLAC files.
///
/// **Parameters:**
///
/// - `filename: &str` -- The name of the file to be processed, eg. "somefile.flac".
/// - `new_tags: &HashMap<String, String>` -- A set of new tags in Key/Value form, eg. _key = ALBUMARTIST_, _value = "The Tragically Hip"_
/// - `config: &DefaultValues` -- A struct containing default values read from a config file and the CLI
///
/// **Returns:**
///
/// `Result<(), Box<dyn Error>>` -- Nothing except `Ok` if things go well, otherwise an error.
///
/// **Example:**
///
/// `flac::process("somefile.flac", &my_tags, &my_config)?;`
pub fn process(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    let mut tags = Tag::read_from_path(&filename)?;
    log::debug!("Filename: {}", filename);

    let mut processed_ok = false;

    // Output existing blocks
    for block in tags.blocks() {
        log::trace!("{:?}", block);
    }

    let mut config = config.clone();

    // Read old tags
    if let Some(id3) = tags.vorbis_comments() {
        log::debug!("vendor_string = {}", &id3.vendor_string);
        for (key, values) in &id3.comments {
            for value in values {
                log::debug!("Old {} = {}", key, value.trim());

                // If TRACKNUMBER or DISCNUMBER is in the x/y format, we need to fix it.
                if key == "TRACKNUMBER" && common::need_split(value) {
                    let track_split = common::split_val(value.trim())?;
                    log::debug!("track_split = {:?}", track_split);
                    if track_split.0 != 0 {
                        config.track_number = Some(track_split.0);
                    }
                    if track_split.1 != 0 {
                        config.track_total = Some(track_split.1);
                    }
                } // TRACKNUMBERid3t --help
                if key == "DISCNUMBER" && common::need_split(value) {
                    let disc_split = common::split_val(value.trim())?;
                    log::debug!("disc_split = {:?}", disc_split);
                    if disc_split.0 != 0 {
                        config.disc_number = Some(disc_split.0);
                    }
                    if disc_split.1 != 0 {
                        config.disc_total = Some(disc_split.1);
                    }
                } // DISCNUMBER
            } // for value in values
        } // for (key, value)
    } // if let

    // Set new tags
    for (key, value) in new_tags {
        if !(config.detail_off.unwrap_or(false)) {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        } else if config.dry_run.unwrap_or(false) {
            log::info!("{} :: New {} = {}", &filename, key, value.trim());
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "PICTUREFRONT" => {
                log::debug!("Setting front cover.");
                match add_picture(&mut tags, value.trim(), CoverFront) {
                    Ok(_) => log::trace!("Picture set."),
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set front cover to {}. Error message: {}",
                                value, err
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set front cover to {}. Continuing. Error message: {}",
                            value,
                            err
                        );
                    }
                } // match
            } // PICTUREFRONT

            "PICTUREBACK" => {
                log::debug!("Setting back cover.");
                match add_picture(&mut tags, value.trim(), CoverBack) {
                    Ok(_) => log::trace!("Picture set."),
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set back cover to {}. Error message: {}",
                                value, err
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set back cover to {}. Error message: {}",
                            value,
                            err
                        );
                    }
                } // match
            } // PICTUREBACK

            _ => tags.set_vorbis(key.clone(), vec![value.clone().trim()]),
        } // match key.as_ref()
    }

    // Try to save
    if config.dry_run.unwrap_or(true) {
        log::debug!("Dry-run. Not saving.");
        processed_ok = true;
    } else if tags.save().is_ok() {
        processed_ok = true;
        log::info!("{}   ✓", filename);
    } else {
        if config.stop_on_error.unwrap_or(true) {
            return Err(format!("Unable to save {}", filename).into());
        }
        log::warn!("Unable to save {}", filename);
    }

    // Rename file
    if config.rename_file.is_some() {
        rename_file(filename, &config, &tags)?;
    }

    // Return safely
    Ok(processed_ok)
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
    let mime_fmt = common::get_mime_type(value)?;
    log::debug!("MIME type: {}", mime_fmt);
    log::debug!("Reading image file {}", value);
    let data = fs::read(value)?;
    log::debug!("Attempting to set picture.");
    tags.add_picture(mime_fmt, cover_type, data);

    // Return safely
    Ok(())
}

/// Renames a FLAC file based on the pattern provided
fn rename_file(
    filename: &str,
    config: &DefaultValues,
    tags: &metaflac::Tag,
) -> Result<(), Box<dyn Error>> {
    let tags_names = tags::option_to_tag(FileTypes::Flac);
    let mut replace_map = HashMap::new();
    let mut pattern = "".to_string();
    if let Some(p) = &config.rename_file {
        pattern = p.clone();
    }

    // get the mappings of %aa --> ALBUMARTIST --> Madonna
    // key = %aa, vorbis_key = ALBUMARTIST, vval = Madonna
    for (key, vorbis_key) in tags_names {
        if let Some(mut vval) = tags.get_vorbis(&vorbis_key) {
            let value = vval.next().unwrap_or_default().to_string();
            log::debug!("key = {}, value = {}", key, value);
            replace_map.insert(key, value);
        }
    }
    log::debug!("replace_map = {:?}", replace_map);

    // Try to rename, and process the result
    let rename_result = rename_file::rename_file(filename, &replace_map, config);
    match rename_result {
        Ok(new_filename) => log::info!("{} --> {}", filename, new_filename),
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!(
                    "Unable to rename {} with tags \"{}\". Error: {}",
                    filename, pattern, err
                )
                .into());
            }
            log::warn!(
                "Unable to rename {} with tags \"{}\". Error: {} Continuing.",
                filename,
                pattern,
                err
            );
        }
    }

    // Return safely
    Ok(())
}