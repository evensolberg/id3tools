//! Contains the functionality to process FLAC files.

use crate::default_values::DefaultValues;
use crate::formats::images::read_cover;
use crate::formats::tags;
use crate::formats::FileTypes;
use crate::rename_file;
use metaflac::block::PictureType::{CoverBack, CoverFront};
use metaflac::Tag;
use std::collections::HashMap;
use std::error::Error;
/// Splits the incoming value into two the (disc/track) number and count.
/// Inserts the split values into their respective spots in the `HashSet`.
///
/// # Arguments
///
/// - `$cfg:ident` - the name of the `DefaultValues` config being used.
/// - `$tag:ident` - the name of the `HashSet` being used.
/// - `$nt:ident` - the name of the field holding the total number of discs/tracks (`number_total`)
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
    m_file: &str,
    nt: &mut HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    let mut tags = Tag::read_from_path(m_file)?;
    let mut processed_ok = false;
    let mut cfg = config.clone();
    let max_size = cfg.picture_max_size.unwrap_or(500);

    // Read old tags
    if let Some(id3) = tags.vorbis_comments() {
        for (k, values) in &id3.comments {
            for v in values {
                log::debug!("Old {} = {}", k, v.trim());

                // If TRACKNUMBER or DISCNUMBER is in the x/y format, we need to fix it.
                if k == "TRACKNUMBER" && common::need_split(v) {
                    split!(cfg, nt, track_number_total, track_number, "TRACK", v);
                }

                if k == "DISCNUMBER" && common::need_split(v) {
                    split!(cfg, nt, disc_number_total, disc_number, "DISC", v);
                } // DISCNUMBER
            } // for value in values
        } // for (key, value)
    } // if let

    // Set new tags
    for (k, v) in nt {
        if !(cfg.detail_off.unwrap_or(false)) {
            log::debug!("{} :: New {} = {}", &m_file, k, v);
        } else if cfg.dry_run.unwrap_or(false) {
            log::info!("{} :: New {} = {}", &m_file, k, v.trim());
        } else {
            log::debug!("{} :: New {} = {}", &m_file, k, v);
        }

        // Process the tags
        match k.as_ref() {
            // Pictures need special treatment
            "PICTUREFRONT" | "PICTUREBACK" => {
                let cover_type = if k == "PICTUREFRONT" {
                    CoverFront
                } else {
                    CoverBack
                };

                match set_picture(&mut tags, v.trim(), cover_type, max_size) {
                    Ok(_) => log::trace!("Picture set."),
                    Err(err) => {
                        if cfg.stop_on_error.unwrap_or(true) {
                            return Err(format!(
                                "Unable to set {cover_type:?} to {v}. Error message: {err}"
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set {cover_type:?} to {v}. Continuing. Error message: {err}"
                        );
                    }
                } // match
            }

            _ => tags.set_vorbis(k.clone(), vec![v.clone().trim()]),
        } // match key.as_ref()
    }

    // Try to save
    if cfg.dry_run.unwrap_or(true) {
        log::debug!("Dry-run. Not saving.");
        processed_ok = true;
    } else if tags.save().is_ok() {
        processed_ok = true;
        log::info!("{}   ✓", m_file);
    } else {
        if cfg.stop_on_error.unwrap_or(true) {
            return Err(format!("Unable to save {m_file}").into());
        }
        log::warn!("Unable to save {}", m_file);
    }

    // Rename file
    if cfg.rename_file.is_some() {
        rename_file(m_file, &cfg, &tags)?;
    }

    // Return safely
    Ok(processed_ok)
}

/// Set the front or back cover (for now)
fn set_picture(
    tags: &mut metaflac::Tag,
    img_file: &str,
    cover_type: metaflac::block::PictureType,
    max_size: u32,
) -> Result<(), Box<dyn Error>> {
    tags.remove_picture_type(cover_type);
    let img = read_cover(img_file, max_size)?;

    tags.add_picture("image/jpeg", cover_type, img.into_inner());

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
