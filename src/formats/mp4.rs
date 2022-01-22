//! Contains the functionality to process MP4 files.
//!
use crate::default_values::DefaultValues;
use crate::{rename_file, shared};
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
            "aART" => tag.set_album_artist(value.trim()),
            "soaa" => tag.set_data(Fourcc(*b"soaa"), Data::Utf8(value.trim().into())), // Album artist stort
            "©alb" => tag.set_album(value.trim()),
            "soal" => tag.set_data(Fourcc(*b"soal"), Data::Utf8(value.trim().into())), // Album title sort
            "©ART" => tag.set_artist(value.trim()),
            "soar" => tag.set_data(Fourcc(*b"soar"), Data::Utf8(value.trim().into())), // Track artist stort
            "©nam" => tag.set_title(value.trim()),
            "sonm" => tag.set_data(Fourcc(*b"sonm"), Data::Utf8(value.trim().into())), // Track title sort
            "©gen" => tag.set_genre(value.trim()),
            "©wrt" => tag.set_composer(value.trim()),
            "soco" => tag.set_data(Fourcc(*b"soco"), Data::Utf8(value.trim().into())), // Composer sort
            "©day" => tag.set_year(value.trim()),
            "©cmt" => tag.set_comment(value.trim()),
            "covr-f" => set_picture(&mut tag, value.trim())?,
            "covr-b" => log::warn!("Setting back cover on MP4 files is currently not implemented."),
            "disk" => tag.set_disc_number(value.parse::<u16>().unwrap_or(1)),
            "disk-t" => tag.set_total_discs(value.parse::<u16>().unwrap_or(1)),
            "trkn" => tag.set_track_number(value.parse::<u16>().unwrap_or(1)),
            "trkn-t" => tag.set_total_tracks(value.parse::<u16>().unwrap_or(1)),
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

    // Rename file
    if config.rename_file.is_some() {
        rename_mp4(filename, config, tag)?;
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

/// Renames the MP4 file based on the pattern provided
fn rename_mp4(
    filename: &str,
    config: &DefaultValues,
    tag: mp4ameta::Tag,
) -> Result<(), Box<dyn Error>> {
    let tags_map = get_mp4_tags(&tag)?;
    log::debug!("tags_map = {:?}", tags_map);

    let mut pattern = "".to_string();
    if let Some(p) = &config.rename_file {
        pattern = p.clone();
    }

    let rename_result = rename_file::rename_file(filename, &tags_map, config);
    match rename_result {
        Ok(new_filename) => log::info!("{} --> {}", filename, new_filename),
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!(
                    "Unable to rename {} with tags \"{}\". Error: {}",
                    filename, pattern, err
                )
                .into());
            } else {
                log::warn!(
                    "Unable to rename {} with tags \"{}\". Error: {} Continuing.",
                    filename,
                    pattern,
                    err
                );
            }
        }
    }

    // Return safely
    Ok(())
}

/// Reads the tags from the MP4 tag "the hard way"
fn get_mp4_tags(tags: &mp4ameta::Tag) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut res = HashMap::<String, String>::new();

    let mut data = tags.album_artist().unwrap_or("").to_string();
    res.insert("%album-artist".to_string(), data.clone());
    res.insert("%aa".to_string(), data);

    data = tags // Album Artist Sort
        .data_of(&Fourcc(*b"soaa"))
        .next()
        .unwrap_or(&Data::Utf8("".to_owned()))
        .string()
        .unwrap_or("")
        .to_string();
    res.insert("%album-artist-sort".to_string(), data.clone());
    res.insert("%aas".to_string(), data);

    data = tags.album().unwrap_or("").to_string();
    res.insert("%album-title".to_string(), data.clone());
    res.insert("%at".to_string(), data);

    data = tags // Album Title Sort
        .data_of(&Fourcc(*b"soal"))
        .next()
        .unwrap_or(&Data::Utf8("".to_owned()))
        .string()
        .unwrap_or("")
        .to_string();
    res.insert("%album-title-sort".to_string(), data.clone());
    res.insert("%ats".to_string(), data);

    data = format!("{:0>2}", tags.disc_number().unwrap_or(0));
    res.insert("%disc-number".to_string(), data.clone());
    res.insert("%dn".to_string(), data);

    data = format!("{:0>2}", tags.total_discs().unwrap_or(0));
    res.insert("%disc-number-total".to_string(), data.clone());
    res.insert("%dt".to_string(), data);

    data = tags.artist().unwrap_or("").to_string();
    res.insert("%track-artist".to_string(), data.clone());
    res.insert("%ta".to_string(), data);

    data = tags // Track Artist Sort
        .data_of(&Fourcc(*b"soar"))
        .next()
        .unwrap_or(&Data::Utf8("".to_owned()))
        .string()
        .unwrap_or("")
        .to_string();
    res.insert("%track-artist-sort".to_string(), data.clone());
    res.insert("%tas".to_string(), data);

    data = tags.title().unwrap_or("").to_string();
    res.insert("%track-title".to_string(), data.clone());
    res.insert("%tt".to_string(), data);

    data = tags // Track Title Sort
        .data_of(&Fourcc(*b"sonm"))
        .next()
        .unwrap_or(&Data::Utf8("".to_owned()))
        .string()
        .unwrap_or("")
        .to_string();
    res.insert("%track-title-sort".to_string(), data.clone());
    res.insert("%tts".to_string(), data);

    data = format!("{:0>2}", tags.track_number().unwrap_or(0));
    res.insert("%track-number".to_string(), data.clone());
    res.insert("%tn".to_string(), data);

    data = format!("{:0>2}", tags.total_tracks().unwrap_or(0));
    res.insert("%track-number-total".to_string(), data.clone());
    res.insert("%to".to_string(), data);

    data = tags.genre().unwrap_or("").to_string();
    res.insert("%track-genre".to_string(), data.clone());
    res.insert("%tg".to_string(), data);

    data = tags.composer().unwrap_or("").to_string();
    res.insert("%track-composer".to_string(), data.clone());
    res.insert("%tc".to_string(), data);

    data = tags // Track Composer Sort
        .data_of(&Fourcc(*b"soco"))
        .next()
        .unwrap_or(&Data::Utf8("".to_owned()))
        .string()
        .unwrap_or("")
        .to_string();
    res.insert("%track-composer-sort".to_string(), data.clone());
    res.insert("%tcs".to_string(), data);

    data = tags.year().unwrap_or("").to_string();
    res.insert("%track-date".to_string(), data.clone());
    res.insert("%td".to_string(), data);

    // Return safely
    Ok(res)
}
