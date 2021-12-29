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

/// Process MP3 file - set tags and covers
pub fn process_mp3(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    let mut tag = Tag::read_from_path(&filename)?;
    log::trace!("Tag = {:?}", tag);
    for frame in tag.frames() {
        log::debug!("{} = {}", frame.id(), frame.content());
    }

    // Print new tags
    for (key, value) in new_tags {
        if !(config.detail_off.unwrap()) {
            log::info!("{} :: New {} = {}", &filename, key, value);
        } else {
            log::debug!("{} :: New {} = {}", &filename, key, value);
        }

        // Process the tags
        match key.as_ref() {
            "APIC-F" => add_picture(&mut tag, value, PictureType::CoverFront)?,
            "APIC-B" => add_picture(&mut tag, value, PictureType::CoverBack)?,
            "COMM" => set_comment(&mut tag, value)?,
            "TPOS" => tag.set_disc(u32::from_str_radix(value, 32)?),
            "TPOS-T" => tag.set_total_discs(u32::from_str_radix(value, 32)?),
            "TRCK" => tag.set_track(u32::from_str_radix(value, 32)?),
            "TRCK-T" => tag.set_total_tracks(u32::from_str_radix(value, 32)?),
            _ => tag.set_text(key, value),
        }
    }

    // Process tags

    if !config.dry_run.unwrap() {
        log::info!("Writing: {}.", filename);
        tag.write_to_path(filename, Version::Id3v24)?;
    } else {
        log::debug!("Not writing {}", filename);
    }

    // return safely
    Ok(())
}

fn add_picture(
    tags: &mut id3::Tag,
    value: &str,
    cover_type: PictureType,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Removing existing picture.");
    tags.remove_picture_by_type(cover_type);
    log::debug!("Reading image file {}", value);

    let desc = if cover_type == PictureType::CoverFront {
        "Front Cover".to_string()
    } else {
        "Back Cover".to_string()
    };

    // Read the file and check the mime type
    let mime_fmt = shared::mime_type(value)?;
    log::debug!("Image format: {}", mime_fmt);
    log::debug!("Setting picture to {}", value);
    tags.add_picture(Picture {
        mime_type: mime_fmt,
        picture_type: cover_type,
        description: desc,
        data: fs::read(&value)?,
    });

    // Return safely
    Ok(())
}

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
