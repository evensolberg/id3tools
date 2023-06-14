//! Contains the functionality to process MP3 files.
use crate::formats::images::read_cover;
use crate::formats::tags::option_to_tag;
use crate::{default_values::DefaultValues, rename_file};
use common::FileTypes;
use id3::frame::{self, ExtendedText};
use id3::TagLike;
use id3::{frame::PictureType, Tag, Version};

use std::collections::HashMap;
use std::error::Error;

/// Performs the actual processing of MP4 files.
#[allow(clippy::too_many_lines)]
pub fn process(
    filename: &str,
    nt: &HashMap<String, String>,
    cfg: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    log::debug!("Filename: {filename}");
    let mut processed_ok = false;
    let max_size = cfg.picture_max_size.unwrap_or(500);

    // Reat the tag - bomb out if it doesn't work.
    let mut tag = Tag::read_from_path(filename)?;

    // Print new tags
    for (key, value) in nt {
        // Output information about tags getting changed
        if cfg.dry_run.unwrap_or(false) {
            log::info!("{filename} :: New {key} = {value}");
        }

        // Process the tags into the file. Arguaby we could skip this if it's a
        // dry run, but it's good to do it anyway to ensure that it works.
        match key.as_ref() {
            // Front picture
            "APIC-F" => {
                match set_picture(&mut tag, value.trim(), PictureType::CoverFront, max_size) {
                    Ok(_) => (),
                    Err(err) => {
                        if cfg.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set front cover for {filename}. Error: {err}"
                            )
                            .into());
                        }
                        log::error!("Unable to set front cover for {filename}. Error: {err}");
                    }
                }
            }

            // Back picture
            "APIC-B" => match set_picture(&mut tag, value.trim(), PictureType::CoverBack, max_size)
            {
                Ok(_) => (),
                Err(err) => {
                    if cfg.stop_on_error.unwrap_or(false) {
                        return Err(format!(
                            "Unable to set back cover for {filename}. Error: {err}"
                        )
                        .into());
                    }
                    log::error!("Unable to set back cover for {filename}. Error: {err}");
                }
            },

            // Comment
            "COMM" => set_comment(&mut tag, value.trim()),

            // Disc number
            "TPOS" => {
                let num = match value.parse::<u32>() {
                    Ok(n) => n,
                    Err(err) => {
                        if cfg.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set disc number to {value}. Error: {err}"
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set disc number to {value}. Setting to 1 and continuing. Error: {err}"
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
                        if cfg.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set total discs to {value}. Error: {err}"
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set total discs to {value}. Setting to 1 and continuing. Error: {err}"                        );
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
                        if cfg.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set track number to {value}. Error: {err}"
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set track number to {value}. Setting to 1 and continuing. Error: {err}"
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
                        if cfg.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set total tracks to {value}. Error: {err}",
                            )
                            .into());
                        }
                        log::error!(
                            "Unable to set total tracks to {value}. Setting to 1 and continuing. Error: {err}"
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

    // Write tags to file - unless we're on a dry run.
    if cfg.dry_run.unwrap_or(true) {
        processed_ok = true;
    } else if tag.write_to_path(filename, Version::Id3v24).is_ok() {
        processed_ok = true;
        log::info!("{filename}  ✓");
    }

    // Rename file
    if cfg.rename_file.is_some() {
        processed_ok = rename_file(filename, cfg, &tag).is_ok();
    }

    // return safely
    Ok(processed_ok)
}

/// Adds front or back covers
fn set_picture(
    tags: &mut Tag,
    img_file: &str,
    picture_type: PictureType,
    max_size: u32,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Removing existing picture.");
    tags.remove_picture_by_type(picture_type);

    let description = if picture_type == PictureType::CoverFront {
        "Front Cover".to_string()
    } else {
        "Back Cover".to_string()
    };

    // Read the file and check the mime type
    log::debug!("Reading image file {img_file}");
    let img = read_cover(img_file, max_size)?;
    let mime_type = String::from("image/jpeg");
    log::debug!("Image format: {mime_type}");

    log::debug!("Setting picture to {img_file}");
    tags.add_frame(frame::Picture {
        mime_type,
        picture_type,
        description,
        data: img,
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
    log::debug!("Setting comment to: {value}");
    tags.add_frame(ExtendedText {
        description: String::from("Comment"),
        value: String::from(value),
    });
}

/// Renames an MP3 file based on the pattern provided
fn rename_file(filename: &str, cfg: &DefaultValues, tag: &id3::Tag) -> Result<(), Box<dyn Error>> {
    let tags_names = option_to_tag(FileTypes::MP3);
    let mut replace_map = HashMap::new();

    let mut pattern = String::new();
    if let Some(p) = &cfg.rename_file {
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
                log::debug!("{tag_name} count = {count}, total = {total}");
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
                            "Unknown tag {tag_name} encountered when unwrapping disc/track information."
                        )
                        .into())
                    }
                }
            } else {
                let value = vval.to_string();
                log::debug!("key = {key}, tag_name = {tag_name}, value = {value}");
                replace_map.insert(key, value);
            }
        }
    }

    log::debug!("replace_map = {replace_map:?}");

    let rename_result = rename_file::rename_file(filename, &replace_map, cfg);
    match rename_result {
        Ok(new_filename) => log::info!("{filename} --> {new_filename}"),
        Err(err) => {
            if cfg.stop_on_error.unwrap_or(true) {
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
