//! Show DSF file metadata.
use dsf::{self, DsfFile};
use std::{error::Error, path::Path};

/// Performs the actual processing of DSF files.
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);
    let path = Path::new(&filename);

    if show_detail {
        match DsfFile::open(path) {
            Ok(dsf_file) => {
                log::info!("DSF file metadata:\n\n{}", dsf_file);
            }
            Err(error) => {
                return Err(
                    format!("Unable to read DSF file {filename}. Error: {error}").into(),
                );
            }
        }
    } else if let Some(tag) = DsfFile::open(path)?.id3_tag().clone() {
        log::debug!("Tag: {:?}", tag);
        for frame in tag.frames() {
            log::info!("  {} = {}", frame.id(), frame.content());
        }
    } else {
        return Err(format!("Unable to read DSF file {filename}").into());
    }

    Ok(())
}
