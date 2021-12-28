//! Contains the functionality to process MP3 files.
use crate::default_values::DefaultValues;
use id3::Tag;

use std::collections::HashMap;
use std::error::Error;

/// Process MP3 file - set tags and covers
pub fn process_mp3(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    let mut tag = Tag::read_from_path(&filename)?;
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
    }

    if !config.dry_run.unwrap() {
        log::info!("Writing: {}.", filename);
        //tag.write_to_path("music.mp3", Version::Id3v24)?;
    } else {
        log::debug!("Not writing {}", filename);
    }

    // return safely
    Ok(())
}
