//! Contains the functionality to process MP4 files.
//!
use crate::default_values::DefaultValues;
use crate::shared;
use mp4ameta::{Data, Fourcc, ImgFmt, Tag};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Performs the actual processing of MP4 files.
pub fn process_mp4(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    // Read existing tags
    let mut tag = Tag::read_from_path(filename)?;
    log::trace!("Tag: {:?}", tag);
    for (data_ident, data) in tag.data() {
        log::debug!("{:?} = {:?}", data_ident, data);

        if data.is_image() {
            log::trace!("{:?} = {:?}", data, data.image_data());
        }
    }

    // Print new tags
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
            "aART" => tag.set_album_artist(value),
            "soaa" => tag.set_data(Fourcc(*b"soaa"), Data::Utf8(value.into())), // Album artist stort
            "©alb" => tag.set_album(value),
            "soal" => tag.set_data(Fourcc(*b"soal"), Data::Utf8(value.into())), // Album title sort
            "©ART" => tag.set_artist(value),
            "soar" => tag.set_data(Fourcc(*b"soar"), Data::Utf8(value.into())), // Track artist stort
            "©nam" => tag.set_title(value),
            "sonm" => tag.set_data(Fourcc(*b"sonm"), Data::Utf8(value.into())), // Track title sort
            "©gen" => tag.set_genre(value),
            "©wrt" => tag.set_composer(value),
            "soco" => tag.set_data(Fourcc(*b"soco"), Data::Utf8(value.into())), // Composer sort
            "©day" => tag.set_year(value),
            "©cmt" => tag.set_comment(value),
            "covr-f" => set_picture(&mut tag, value)?,
            "covr-b" => log::warn!("Setting back cover on MP4 files is currently not implemented."),
            "disk" => tag.set_disc_number(u16::from_str_radix(value, 16)?),
            "disk-t" => tag.set_total_discs(u16::from_str_radix(value, 16)?),
            "trkn" => tag.set_track_number(u16::from_str_radix(value, 16)?),
            "trkn-t" => tag.set_total_tracks(u16::from_str_radix(value, 16)?),
            _ => {
                // tag.set_data(Fourcc(key.as_bytes().try_into()?), Data::Utf8(value.into()));
                return Err(format!("Unknown key: {}", key).into());
            }
        }
    }

    // Process tags

    if config.dry_run.unwrap_or(true) {
        log::debug!("Not writing {}", filename);
    } else {
        tag.write_to_path(filename)?;
        log::info!("{}  ✓", filename);
    }

    // return safely
    Ok(())
}

/// Sets the front or back cover
fn set_picture(tags: &mut Tag, value: &str) -> Result<(), Box<dyn Error>> {
    log::debug!("Checking image file type.");
    let ext = shared::get_extension(value);
    let fmt = match ext.as_ref() {
        "jpg" | "jpeg" => ImgFmt::Jpeg,
        "png" => ImgFmt::Png,
        "bmp" => ImgFmt::Bmp,
        _ => return Err("Unsupported image file format. Must be one of BMP, JPEG or PNG.".into()),
    };
    log::debug!("Reading image file {}", value);
    let data = fs::read(&value)?;

    log::debug!("Setting picture to {}", value);
    tags.set_artwork(mp4ameta::Img { fmt, data });

    // Return safely
    Ok(())
}
