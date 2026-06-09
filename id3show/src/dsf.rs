//! Show DSF file metadata.
use anyhow::{bail, Context, Result};
use dsf::{self, DsfFile};
use std::path::Path;

/// Performs the actual processing of DSF files.
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<()> {
    log::debug!("Filename: {filename}");
    let path = Path::new(&filename);

    if show_detail {
        match DsfFile::open(path) {
            Ok(dsf_file) => {
                println!("DSF file metadata:\n\n{dsf_file}");
            }
            Err(error) => {
                return Err(error).with_context(|| format!("Unable to read DSF file {filename}"));
            }
        }
    } else if let Some(tag) = DsfFile::open(path)?.id3_tag().clone() {
        log::debug!("Tag: {tag:?}");
        for frame in tag.frames() {
            println!("  {} = {}", frame.id(), frame.content());
        }
    } else {
        bail!("Unable to read DSF file {filename}");
    }

    Ok(())
}
