//! Contains the functionality to process MP3 files.
use crate::formats::tags::option_to_tag;
use crate::{default_values::DefaultValues, rename_file};
use common::FileTypes;
use id3::frame::{self, ExtendedText};
use id3::TagLike;
use id3::{frame::PictureType, Tag, Version};

use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Performs the actual processing of MP4 files.
pub fn process(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);
    let mut processed_ok = false;

    // Reat the tag - bomb out if it doesn't work.
    let mut tag = Tag::read_from_path(&filename)?;

    log::trace!("Tag = {:?}", tag);
    for frame in tag.frames() {
        log::debug!("{} = {}", frame.id(), frame.content());
    }

    // Print new tags
    for (key, value) in new_tags {
        // Output information about tags getting changed
        if config.detail_off.unwrap_or(false) {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        } else if config.dry_run.unwrap_or(false) {
            log::info!("{} :: New {} = {}", &filename, key, value);
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags into the file. Arguaby we could skip this if it's a
        // dry run, but it's good to do it anyway to ensure that it works.
        match key.as_ref() {
            // Front picture
            "APIC-F" => match add_picture(&mut tag, value.trim(), PictureType::CoverFront) {
                Ok(_) => (),
                Err(err) => {
                    if config.stop_on_error.unwrap_or(false) {
                        return Err(format!(
                            "Unable to set front cover for {}. Error: {}",
                            filename, err
                        )
                        .into());
                    }
                    log::error!("Unable to set front cover for {}. Error: {}", filename, err);
                }
            },

            // Back picture
            "APIC-B" => match add_picture(&mut tag, value.trim(), PictureType::CoverBack) {
                Ok(_) => (),
                Err(err) => {
                    if config.stop_on_error.unwrap_or(false) {
                        return Err(format!(
                            "Unable to set back cover for {}. Error: {}",
                            filename, err
                        )
                        .into());
                    }
                    log::error!("Unable to set back cover for {}. Error: {}", filename, err);
                }
            },

            // Comment
            "COMM" => set_comment(&mut tag, value.trim()),

            // Disc number
            "TPOS" => {
                let num = match value.parse::<u32>() {
                    Ok(n) => n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set disc number to {}. Error: {}",
                                value, err
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set disc number to {}. Setting to 1 and continuing. Error: {}",
                            value,
                            err
                        );
                        1
                    }
                };
                tag.set_disc(num);
            }

            // Disc count
            "TPOS-T" => {
                let num = match value.parse::<u32>() {
                    Ok(n) => n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set total discs to {}. Error: {}",
                                value, err
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set total discs to {}. Setting to 1 and continuing. Error: {}",
                            value,
                            err
                        );
                        1
                    }
                };
                tag.set_total_discs(num);
            }

            // Track number
            "TRCK" => {
                let num = match value.parse::<u32>() {
                    Ok(n) => n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set track number to {}. Error: {}",
                                value, err
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set track number to {}. Setting to 1 and continuing. Error: {}",
                            value,
                            err
                        );
                        1
                    }
                };
                tag.set_track(num);
            }

            // Track count
            "TRCK-T" => {
                let num = match value.parse::<u32>() {
                    Ok(n) => n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set total tracks to {}. Error: {}",
                                value, err
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set total tracks to {}. Setting to 1 and continuing. Error: {}",
                            value,
                            err
                        );
                        1
                    }
                };
                tag.set_total_tracks(num);
            }

            // Everything else
            _ => tag.set_text(key, value.trim()),
        }
    }

    // Write tags to file
    if config.dry_run.unwrap_or(true) {
        log::debug!("Not writing {}", filename);
        processed_ok = true;
    } else if tag.write_to_path(filename, Version::Id3v24).is_ok() {
        processed_ok = true;
        log::info!("{}  ✓", filename);
    }

    // Rename file
    if config.rename_file.is_some() {
        if rename_file(filename, config, &tag).is_ok() {
            processed_ok = true;
        } else {
            processed_ok = false;
        }
    }

    // return safely
    Ok(processed_ok)
}

/// Adds front or back covers
fn add_picture(
    tags: &mut Tag,
    filename: &str,
    picture_type: PictureType,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Removing existing picture.");
    tags.remove_picture_by_type(picture_type);
    log::debug!("Reading image file {}", filename);

    let description = if picture_type == PictureType::CoverFront {
        "Front Cover".to_string()
    } else {
        "Back Cover".to_string()
    };

    // Read the file and check the mime type
    let mime_type = common::get_mime_type(filename)?;
    log::debug!("Image format: {}", mime_type);

    log::debug!("Reading image file {}", filename);
    let image_data = fs::read(&filename)?;

    log::debug!("Setting picture to {}", filename);
    tags.add_frame(frame::Picture {
        mime_type,
        picture_type,
        description,
        data: image_data,
    });

    // Return safely
    Ok(())
}

/// Sets the comments field
fn set_comment(tags: &mut id3::Tag, value: &str) {
    log::debug!("Removing {} existing comment(s):", tags.comments().count());
    for comment in tags.comments() {
        log::debug!(
            "Comment: lang: {}, description: {}, text: {}",
            comment.lang,
            comment.description,
            comment.text
        );
    }
    tags.remove("COMM");
    log::debug!("Setting comment to: {}", value);
    tags.add_frame(ExtendedText {
        description: "Comment".to_string(),
        value: value.to_string(),
    });
}

/// Renames an MP3 file based on the pattern provided
fn rename_file(
    filename: &str,
    config: &DefaultValues,
    tag: &id3::Tag,
) -> Result<(), Box<dyn Error>> {
    let tags_names = option_to_tag(FileTypes::MP3);
    let mut replace_map = HashMap::new();

    let mut pattern = "".to_string();
    if let Some(p) = &config.rename_file {
        pattern = p.clone();
    }

    // get the mappings of %aa --> ALBUMARTIST --> Madonna
    // key = %aa, vorbis_key = ALBUMARTIST, vval = Madonna
    for (key, tag_name) in tags_names {
        // Get the MP3 value based on the tag_name from the HashMap
        if let Some(vval) = tag.get(&tag_name).and_then(|frame| frame.content().text()) {
            if tag_name == "TPOS" || tag_name == "TRCK" {
                let separates: Vec<&str> = vval.split('/').collect();
                let mut count = "01".to_string();
                let mut total = "01".to_string();
                if !separates.is_empty() {
                    count = format!("{:0>2}", separates[0]);
                }
                if separates.len() > 1 {
                    total = format!("{:0>2}", separates[1]);
                }
                log::debug!("{} count = {}, total = {}", tag_name, count, total);
                match tag_name.as_str() {
                    "TPOS" => {
                        replace_map.insert("%dn".to_string(), count.clone());
                        replace_map.insert("%disc-number".to_string(), count);
                        replace_map.insert("%dt".to_string(), total.clone());
                        replace_map.insert("%dnt".to_string(), total.clone());
                        replace_map.insert("%disc-number-total".to_string(), total);
                    }
                    "TRCK" => {
                        replace_map.insert("%tn".to_string(), count.clone());
                        replace_map.insert("%track-number".to_string(), count);
                        replace_map.insert("%to".to_string(), total.clone());
                        replace_map.insert("%tnt".to_string(), total.clone());
                        replace_map.insert("%track-number-total".to_string(), total);
                    }
                    _ => {
                        return Err(format!(
                            "Unknown tag {} encountered when unwrapping disc/track information.",
                            tag_name
                        )
                        .into())
                    }
                }
            } else {
                let value = vval.to_string();
                log::debug!("key = {}, tag_name = {}, value = {}", key, tag_name, value);
                replace_map.insert(key, value);
            }
        }
    }

    log::debug!("replace_map = {:?}", replace_map);

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
