use crate::default_values::DefaultValues;
use metaflac::Tag;
use std::collections::HashMap;
use std::error::Error;

pub fn process_flac(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<(), Box<dyn Error>> {
    let mut file_tags = Tag::read_from_path(&filename).unwrap();

    log::debug!("Filename: {}", filename);
    for block in file_tags.blocks() {
        log::trace!("{:?}", block);
    }

    if let Some(id3) = file_tags.vorbis_comments() {
        log::debug!("vendor_string = {}", &id3.vendor_string);
        for (key, values) in &id3.comments {
            for value in values {
                log::debug!("Old {} = {}", key, value);
            }
        }
        for (key, value) in new_tags {
            log::info!("{} :: New {} = {}", &filename, key, value);
            if !(config.dry_run.unwrap()) {
                file_tags.set_vorbis(key.clone(), vec![value.clone()])
            }
        }
    }

    if !(config.dry_run.unwrap()) {
        log::debug!("Attempting to save file {}", filename);
        file_tags.save()?;
    } else {
        log::debug!("Dry-run. Not saving.")
    }

    log::debug!("Picture count: {}", file_tags.pictures().count());

    // Return safely
    Ok(())
}
