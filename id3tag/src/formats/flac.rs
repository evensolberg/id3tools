//! Contains the functionality to process FLAC files.

use crate::default_values::DefaultValues;
use crate::formats::tags;
use crate::formats::FileTypes;
use crate::rename_file;
use metaflac::block::PictureType::{CoverBack, CoverFront};
use metaflac::Tag;
use std::collections::HashMap;
use std::error::Error;

use crate::formats::images;

/// Splits the incoming value into two the (disc/track) number and count.
/// Inserts the split values into their respective spots in the `HashSet`.
///
/// # Arguments
///
/// - `$cfg:ident` - the name of the `DefaultValues` config being used.
/// - `$tag:ident` - the name of the `HashSet` being used.
/// - `$nt:ident` - the name of the field holding the total number of discs/tracks (number_total)
/// - `$nn:ident` - the name of the field holding the number of the current disc/track
/// - `$nm:literal` - used to indicate whether we're dealing with a "DISC" or "TRACK". Used when inserting into the `HashSet`.
/// - `$value:ident` - the name of the variable containing the value to be split
///
/// # Examples
///
/// `split!(config, new_tags, track_number_total, track_number, "TRACK", value);`
///
macro_rules! split {
    ($cfg:ident, $tag:ident, $nt:ident, $nn:ident, $nm:literal, $value:ident) => {
        let split = common::split_val($value.trim())?;
        if split.0 != 0 {
            $cfg.$nn = Some(split.0);
            let name = format!("{}NUMBER", $nm);
            $tag.insert(name, split.0.to_string());
        }
        if split.1 != 0 {
            $cfg.$nt = Some(split.1);
            let name = format!("{}TOTAL", $nm);
            $tag.insert(name, split.1.to_string());
        }
    };
}

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
    new_tags: &mut HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    let mut tags = Tag::read_from_path(filename)?;
    log::debug!("Filename: {}", filename);

    let mut processed_ok = false;

    // Output existing blocks
    for block in tags.blocks() {
        log::trace!("{:?}", block);
    }

    let mut cfg = config.clone();

    // Read old tags
    if let Some(id3) = tags.vorbis_comments() {
        for (key, values) in &id3.comments {
            for value in values {
                log::debug!("Old {} = {}", key, value.trim());

                // If TRACKNUMBER or DISCNUMBER is in the x/y format, we need to fix it.
                if key == "TRACKNUMBER" && common::need_split(value) {
                    split!(
                        cfg,
                        new_tags,
                        track_number_total,
                        track_number,
                        "TRACK",
                        value
                    );
                }

                if key == "DISCNUMBER" && common::need_split(value) {
                    split!(cfg, new_tags, disc_number_total, disc_number, "DISC", value);
                } // DISCNUMBER
            } // for value in values
        } // for (key, value)
    } // if let

    // Set new tags
    for (key, value) in new_tags {
        if !(cfg.detail_off.unwrap_or(false)) {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        } else if cfg.dry_run.unwrap_or(false) {
            log::info!("{} :: New {} = {}", &filename, key, value.trim());
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            // Pictures need special treatment
            "PICTUREFRONT" | "PICTUREBACK" => {
                let cover_type = if key == "PICTUREFRONT" {
                    CoverFront
                } else {
                    CoverBack
                };
                log::debug!("Setting {cover_type:?}.");

                match add_picture(&mut tags, value.trim(), cover_type) {
                    Ok(_) => log::trace!("Picture set."),
                    Err(err) => {
                        if cfg.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set {cover_type:?} to {value}. Error message: {err}"
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set {cover_type:?} to {value}. Continuing. Error message: {err}"
                        );
                    }
                } // match
            }

            _ => tags.set_vorbis(key.clone(), vec![value.clone().trim()]),
        } // match key.as_ref()
    }

    // Try to save
    if cfg.dry_run.unwrap_or(true) {
        log::debug!("Dry-run. Not saving.");
        processed_ok = true;
    } else if tags.save().is_ok() {
        processed_ok = true;
        log::info!("{}   ✓", filename);
    } else {
        if cfg.stop_on_error.unwrap_or(true) {
            return Err(format!("Unable to save {filename}").into());
        }
        log::warn!("Unable to save {}", filename);
    }

    // Rename file
    if cfg.rename_file.is_some() {
        rename_file(filename, &cfg, &tags)?;
    }

    // Return safely
    Ok(processed_ok)
}

/// Set the front or back cover (for now)
fn add_picture(
    tags: &mut metaflac::Tag,
    filename: &str,
    cover_type: metaflac::block::PictureType,
) -> Result<(), Box<dyn Error>> {
    tags.remove_picture_type(cover_type);

    // Read the file and check the mime type
    let filename_str = rename_file::filename_resized(filename)?;
    let filename = filename_str.as_str();
    let mime_fmt = common::get_mime_type(filename)?;

    let image_data = images::read_cover(filename, 0)?;

    tags.add_picture(mime_fmt, cover_type, image_data);

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
    let mut pattern = String::new();
    if let Some(p) = &config.rename_file {
        pattern = p.clone();
    }

    // get the mappings of %aa --> ALBUMARTIST --> Madonna
    // key = %aa, vorbis_key = ALBUMARTIST, vval = Madonna
    for (key, vorbis_key) in tags_names {
        if let Some(mut vval) = tags.get_vorbis(&vorbis_key) {
            let value = vval.next().unwrap_or_default().to_string();
            log::debug!("key = {key}, value = {value}");
            replace_map.insert(key, value);
        }
    }
    log::debug!("replace_map = {replace_map:?}");

    // Try to rename, and process the result
    let rename_result = rename_file::rename_file(filename, &replace_map, config);
    match rename_result {
        Ok(new_filename) => log::info!("{} --> {}", filename, new_filename),
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!(
                    "Unable to rename {filename} with tags \"{pattern}\". Error: {err}"
                )
                .into());
            }
            log::warn!(
                "Unable to rename {filename} with tags \"{pattern}\". Error: {err} Continuing."
            );
        }
    }

    // Return safely
    Ok(())
}
