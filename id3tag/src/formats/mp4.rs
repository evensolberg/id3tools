//! Contains the functionality to process MP4 files.
//!
use crate::default_values::DefaultValues;
use crate::formats::images;
use crate::rename_file;
use mp4ameta::{Data, Fourcc, ImgFmt, Tag};
use std::collections::HashMap;
use std::error::Error;

/// Performs the actual processing of MP4 files.
pub fn process(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    let mut processed_ok = false;

    // Read existing tags
    let mut tag = Tag::read_from_path(filename)?;
    log::trace!("Tag: {tag:?}");
    for (data_ident, data) in tag.data() {
        log::debug!("{data_ident:?} = {data:?}");

        if data.is_image() {
            log::trace!("{data:?} = {:?}", data.image_data());
        }
    }

    // Process tags
    for (key, value) in new_tags {
        // Let the user know what we're processing
        if !(config.detail_off.unwrap_or(false)) {
            log::debug!("{filename} :: New {key} = {value}");
        } else if config.dry_run.unwrap_or(false) {
            log::info!("{filename} :: New {key} = {value}");
        } else {
            log::debug!("{filename} :: New {key} = {value}");
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
                return Err(format!("Unknown key: {key}").into());
            }
        }
    }

    // Write to file
    if config.dry_run.unwrap_or(true) {
        processed_ok = true;
        log::debug!("Not writing {filename}");
    } else {
        match tag.write_to_path(filename) {
            Ok(_) => processed_ok = true,
            Err(err) => {
                if config.stop_on_error.unwrap_or(true) {
                    return Err(format!("Unable to save tags to {filename}. Error: {err}").into());
                }
                log::warn!("Unable to save tags to {filename}. Error: {err}");
            }
        }
    }

    // Rename file
    if config.rename_file.is_some() {
        match rename_file(filename, config, &tag) {
            Ok(_) => processed_ok = true,
            Err(err) => {
                if config.stop_on_error.unwrap_or(true) {
                    return Err(format!("Unable to rename {filename}. Error: {err}").into());
                }
                log::warn!("Unable to rename {filename}. Error: {err}");
            }
        }
    }

    // return safely
    Ok(processed_ok)
}

/// Sets the front or back cover
fn set_picture(tags: &mut Tag, filename: &str) -> Result<(), Box<dyn Error>> {
    let fmt = ImgFmt::Jpeg;
    let data = images::read_cover(filename, 0)?;
    tags.set_artwork(mp4ameta::Img { fmt, data });

    // Return safely
    Ok(())
}

/// Renames the MP4 file based on the pattern provided
fn rename_file(
    filename: &str,
    config: &DefaultValues,
    tag: &mp4ameta::Tag,
) -> Result<(), Box<dyn Error>> {
    let tags_map = get_mp4_tags(tag);
    log::debug!("tags_map = {tags_map:?}");

    let mut pattern = String::new();
    if let Some(p) = &config.rename_file {
        pattern = p.clone();
    }

    let rename_result = rename_file::rename_file(filename, &tags_map, config);
    match rename_result {
        Ok(new_filename) => log::info!("{filename} --> {new_filename}"),
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

/// Reads the existing values from the MP4 tags "the hard way"
fn get_mp4_tags(tags: &mp4ameta::Tag) -> HashMap<String, String> {
    let mut res = HashMap::<String, String>::new();

    let mut data = tags.album_artist().unwrap_or("").to_string();
    res.insert("%album-artist".to_string(), data.clone());
    res.insert("%aa".to_string(), data);

    data = tags // Album Artist Sort
        .data_of(&Fourcc(*b"soaa"))
        .next()
        .unwrap_or(&Data::Utf8(String::new()))
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
        .unwrap_or(&Data::Utf8(String::new()))
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
    res.insert("%dnt".to_string(), data.clone());
    res.insert("%dt".to_string(), data);

    data = tags.artist().unwrap_or("").to_string();
    res.insert("%track-artist".to_string(), data.clone());
    res.insert("%ta".to_string(), data);

    data = tags // Track Artist Sort
        .data_of(&Fourcc(*b"soar"))
        .next()
        .unwrap_or(&Data::Utf8(String::new()))
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
        .unwrap_or(&Data::Utf8(String::new()))
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
    res.insert("%to".to_string(), data.clone());
    res.insert("%tnt".to_string(), data);

    data = tags.genre().unwrap_or("").to_string();
    res.insert("%track-genre".to_string(), data.clone());
    res.insert("%tg".to_string(), data);

    data = tags.composer().unwrap_or("").to_string();
    res.insert("%track-composer".to_string(), data.clone());
    res.insert("%tc".to_string(), data);

    data = tags // Track Composer Sort
        .data_of(&Fourcc(*b"soco"))
        .next()
        .unwrap_or(&Data::Utf8(String::new()))
        .string()
        .unwrap_or("")
        .to_string();
    res.insert("%track-composer-sort".to_string(), data.clone());
    res.insert("%tcs".to_string(), data);

    data = tags.year().unwrap_or("").to_string();
    res.insert("%track-date".to_string(), data.clone());
    res.insert("%td".to_string(), data);

    // Return safely
    res
}
