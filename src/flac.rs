use metaflac::Tag;
use std::error::Error;

pub fn process_flac(filename: &str) -> Result<(), Box<dyn Error>> {
    let tag = Tag::read_from_path(&filename).unwrap();

    log::debug!("Filename: {}", filename);
    for block in tag.blocks() {
        log::trace!("{:?}", block);
    }

    if let Some(id3) = tag.vorbis_comments() {
        log::debug!("vendor_string = {}", &id3.vendor_string);
        for (key, values) in &id3.comments {
            for value in values {
                log::debug!("{} = {}", key, value);
            }
        }
    }

    log::debug!("Picture count: {}", tag.pictures().count());

    // Return safely
    Ok(())
}
