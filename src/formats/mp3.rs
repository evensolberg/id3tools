//! Contains the functionality to process MP3 files.
use crate::default_values::DefaultValues;
use crate::shared;
use id3::{
    frame::{Comment, Picture, PictureType},
    Tag, Version,
};

use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Performs the actual processing of MP4 files.
pub fn process_mp3(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    // Reat the tag - bomb out if it doesn't work.
    let mut tag = Tag::read_from_path(&filename)?;

    log::trace!("Tag = {:?}", tag);
    for frame in tag.frames() {
        log::debug!("{} = {}", frame.id(), frame.content());
    }

    // Print new tags
    for (key, value) in new_tags {
        if !(config.detail_off.unwrap_or(false)) {
            log::info!("{} :: New {} = {}", &filename, key, value);
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "APIC-F" => match add_picture(&mut tag, value, PictureType::CoverFront) {
                Ok(_) => (),
                Err(err) => {
                    if config.stop_on_error.unwrap_or(false) {
                        return Err(format!(
                            "Unable to set front cover for {}. Error: {}",
                            filename,
                            err.to_string()
                        )
                        .into());
                    } else {
                        log::error!(
                            "Unable to set front cover for {}. Error: {}",
                            filename,
                            err.to_string()
                        );
                    }
                }
            },
            "APIC-B" => match add_picture(&mut tag, value, PictureType::CoverBack) {
                Ok(_) => (),
                Err(err) => {
                    if config.stop_on_error.unwrap_or(false) {
                        return Err(format!(
                            "Unable to set back cover for {}. Error: {}",
                            filename,
                            err.to_string()
                        )
                        .into());
                    } else {
                        log::error!(
                            "Unable to set back cover for {}. Error: {}",
                            filename,
                            err.to_string()
                        );
                    }
                }
            },
            "COMM" => match set_comment(&mut tag, value) {
                Ok(_) => (),
                Err(err) => {
                    if config.stop_on_error.unwrap_or(false) {
                        return Err(format!(
                            "Unable to set comment for {}. Error: {}",
                            filename,
                            err.to_string()
                        )
                        .into());
                    } else {
                        log::error!(
                            "Unable to set comment for {}. Error: {}",
                            filename,
                            err.to_string()
                        );
                    }
                }
            },
            "TPOS" => {
                let num;
                match u32::from_str_radix(value, 32) {
                    Ok(n) => num = n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set disc number to {}. Error: {}",
                                value,
                                err.to_string()
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set disc number to {}. Setting to 1 and continuing. Error: {}",
                                value,
                                err.to_string()
                            );
                            num = 1
                        }
                    }
                }
                tag.set_disc(num);
            }
            "TPOS-T" => {
                let num;
                match u32::from_str_radix(value, 32) {
                    Ok(n) => num = n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set total discs to {}. Error: {}",
                                value,
                                err.to_string()
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set total discs to {}. Setting to 1 and continuing. Error: {}",
                                value,
                                err.to_string()
                            );
                            num = 1
                        }
                    }
                }
                tag.set_total_discs(num);
            }
            "TRCK" => {
                let num;
                match u32::from_str_radix(value, 32) {
                    Ok(n) => num = n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set track number to {}. Error: {}",
                                value,
                                err.to_string()
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set track number to {}. Setting to 1 and continuing. Error: {}",
                                value,
                                err.to_string()
                            );
                            num = 1
                        }
                    }
                }
                tag.set_track(num);
            }
            "TRCK-T" => {
                let num;
                match u32::from_str_radix(value, 32) {
                    Ok(n) => num = n,
                    Err(err) => {
                        if config.stop_on_error.unwrap_or(false) {
                            return Err(format!(
                                "Unable to set total tracks to {}. Error: {}",
                                value,
                                err.to_string()
                            )
                            .into());
                        } else {
                            log::error!(
                                "Unable to set total tracks to {}. Setting to 1 and continuing. Error: {}",
                                value,
                                err.to_string()
                            );
                            num = 1
                        }
                    }
                }
                tag.set_total_tracks(num)
            }
            _ => tag.set_text(key, value),
        }
    }

    // Process tags

    if !config.dry_run.unwrap_or(true) {
        log::info!("Writing: {}.", filename);
        tag.write_to_path(filename, Version::Id3v24)?;
    } else {
        log::debug!("Not writing {}", filename);
    }

    // return safely
    Ok(())
}

/// Adds front or back covers
fn add_picture(
    tags: &mut Tag,
    value: &str,
    picture_type: PictureType,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Removing existing picture.");
    tags.remove_picture_by_type(picture_type);
    log::debug!("Reading image file {}", value);

    let description = if picture_type == PictureType::CoverFront {
        "Front Cover".to_string()
    } else {
        "Back Cover".to_string()
    };

    // Read the file and check the mime type
    let mime_type = shared::get_mime_type(value)?;
    log::debug!("Image format: {}", mime_type);

    log::debug!("Reading image file {}", value);
    let data = fs::read(&value)?;

    log::debug!("Setting picture to {}", value);
    tags.add_picture(Picture {
        mime_type,
        picture_type,
        description,
        data,
    });

    // Return safely
    Ok(())
}

/// Sets the comments field
fn set_comment(tags: &mut id3::Tag, value: &str) -> Result<(), Box<dyn Error>> {
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
    tags.add_comment(Comment {
        lang: "XXX".to_string(),
        description: "Comment".to_string(),
        text: value.to_string(),
    });
    // return safely
    Ok(())
}
