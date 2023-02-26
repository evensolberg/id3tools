//! Image processing module. Contains functions for reading, resizing and writing cover images.
//!
//! The methodology for finding a cover image, is as follows:
//!
//! 1. Gather the search paths
//! 2. Check the search folders for the cover requested.
//! 3. If not found, check the candidates
//! 4. If anything is found, look for a "-resized" version of the file found.
//! 5. If not found, exit
//! 6. Check if the file identified needs to be resized, and if so, save it with "-resized" appended to the filename
//! 7. Load the file as the cover

use image::{self, imageops::FilterType};
use std::error::Error;
use std::io::Cursor;

// Homegrown stuff
mod covertype;
mod ops;
mod paths;
mod tests;

use crate::default_values::DefaultValues;
use common::{directory, path_to_string};
use covertype::CoverType;
use paths::{find_first_image, gather_cover_candidates};

pub use ops::aspect_ratio_ok;

/// Catch the image-related CLI parameters and process the image(s).
///
/// # Arguments
///
/// `music_file: &str` - the name of the current music file being processed.
///
/// `cfg: &DefaultValues` - a copy of the program configuration, which includes search paths, cover file names, etc.
///
/// # Returns
///
/// `Result<(Option<String>, Option<String>), Box<dyn Error>>` - an `Option<String>` tuple containing the paths to the front and back covers, or None, if nothing has been found.
// #[allow(clippy::module_name_repetitions)]
pub fn get_cover_filenames(
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
    let front_cover_path = if cfg.picture_front_candidates.is_some() {
        log::debug!("get_cover_filenames:: Gathering front cover candidates.");
        find_cover(CoverType::Front, music_file, cfg)?
    } else {
        None
    };

    let back_cover_path = if cfg.picture_back_candidates.is_some() {
        log::debug!("get_cover_filenames:: Gathering back cover candidates.");

        find_cover(CoverType::Back, music_file, cfg)?
    } else {
        None
    };

    // return safely
    log::debug!("process_images::front_cover_path = {front_cover_path:?}, process_images::back_cover_path = {back_cover_path:?}");
    Ok((front_cover_path, back_cover_path))
}

/// Search for the cover file in the locations provided - alongside the music file or in the search folders.
///
/// # Arguments:
///
/// `cover_type: CoverType` - the type of cover, i.e., `CoverType::Front` or `CoverType::Back`
/// `music_file: &str` - the name of the music file being processed. This is used to find relative paths.
/// `config: &DefaultValues` - the configuration for the program, containing parameters such as search paths, dry run, etc.
///
/// # Returns:
///
/// `Result<String, Box<dyn Error>>` - returns a string with the path to the cover if found, or an empty string if not.
/// Returns an error if something goes wrong
fn find_cover(
    cover_type: CoverType,
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    let music_path = path_to_string(directory(music_file)?);
    log::debug!("find_cover::music_path = {music_path}");

    let candidate_images = gather_cover_candidates(cover_type, cfg);
    log::debug!("find_cover::candidate_images = {candidate_images:?}");

    let cover_path = find_first_image(music_file, &candidate_images)?;
    log::debug!("find_cover::cover_path = {cover_path:?}");

    if cover_path.is_some() {
        return Ok(Some(path_to_string(cover_path.unwrap_or_default())));
    }

    let image_path = find_first_image(music_file, &candidate_images)?;
    log::debug!("find_cover::image_path = {image_path:?}");

    if image_path.is_some() {
        let cp = path_to_string(image_path.unwrap_or_default());
        return Ok(Some(cp));
    }

    // return
    Ok(None)
} // fn find_cover()

/// Reads the image file and resizes it if needed. Returns the resized image as a vector of bytes.
/// Set `max_size` to 0 to disable resizing.
/// Returns a vector of bytes with the image data.
pub fn read_cover(cover_file: &str, max_size: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(cover_file)?;

    if !aspect_ratio_ok(img.width(), img.height()) {
        return Err("Image {cover_file} is outside the expected ratio.".into());
    }

    let mut eib = Cursor::new(Vec::new());
    if (img.width() > max_size || img.height() > max_size) && max_size > 0 {
        log::debug!("Reiszing to {max_size} pixels.");
        let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);
        img_resized
            .write_to(&mut eib, image::ImageOutputFormat::Jpeg(90))
            .unwrap_or_default();
    } else {
        img.write_to(&mut eib, image::ImageOutputFormat::Jpeg(90))
            .unwrap_or_default();
    };

    Ok(eib.into_inner())
}
