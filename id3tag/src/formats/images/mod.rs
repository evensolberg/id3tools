//! Image processing module. Contains functions for finding and reading cover images.

use image::{self, imageops::FilterType, io::Reader as ImageReader, ImageFormat::Jpeg};
use std::error::Error;
use std::io::Cursor;

// Homegrown stuff
mod covertype;
mod ops;
mod paths;
mod tests;

use crate::default_values::DefaultValues;
use common::path_to_string;
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
pub fn get_cover_filenames(
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
    let front_cover_path = if cfg.picture_front_candidates.is_some() {
        find_cover(CoverType::Front, music_file, cfg)?
    } else {
        None
    };

    let back_cover_path = if cfg.picture_back_candidates.is_some() {
        find_cover(CoverType::Back, music_file, cfg)?
    } else {
        None
    };

    log::debug!("process_images::front_cover_path = {front_cover_path:?}, back_cover_path = {back_cover_path:?}");

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
    let cover_candidates = gather_cover_candidates(cover_type, cfg);

    let cover_path = find_first_image(music_file, &cover_candidates)?;
    if cover_path.is_some() {
        return Ok(Some(path_to_string(cover_path.unwrap_or_default())));
    }

    Ok(None)
} // fn find_cover()

/// Reads the image file and resizes it if needed. Returns the resized image as a vector of bytes.
/// Set `max_size` to 0 to disable resizing.
///
/// # Arguments
///
/// `cover_file: &str` - the name of the cover file to read.
/// `max_size: u32` - the maximum size of the image, in pixels. If the image is larger than this, it will be resized. Set to 0 to disable resizing.
///
/// # Returns
///
/// `Result<Vec<u8>, Box<dyn Error>>` - a vector of bytes with the image data.
///
/// # Errors
///
/// Returns an error if the image cannot be read or if the aspect ratio is not within the expected range.
/// The expected aspect ratio is within 1.5:1 and 1:1.5 (eg. 300x200, 200x300, 300x300, 200x200)
pub fn read_cover(cover_file: &str, max_size: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = ImageReader::open(cover_file)?.decode()?;

    if !aspect_ratio_ok(img.width(), img.height()) {
        return Err(format!("Image {cover_file} is outside the expected ratio.").into());
    }

    let mut img_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    if image_too_small(&img, max_size) {
        return Err(
            format!("Image {cover_file} is too small. (Less than 1/2 the cover size.)").into(),
        );
    }

    if image_too_large(&img, max_size) {
        let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);
        img_resized
            .write_to(&mut img_buffer, Jpeg)
            .unwrap_or_default();
    } else {
        img.write_to(&mut img_buffer, Jpeg).unwrap_or_default();
    };

    Ok(img_buffer.into_inner())
}

/// Checks if the image is too small to be used as a cover.
///
/// # Arguments
///
/// `img: &image::DynamicImage` - the image to check.
/// `max_size: u32` - the maximum size of the image, in pixels. If the image is smaller than half of this on both sides, it is considered too small.
///
/// # Returns
///
/// `bool` - true if the image is too small, false otherwise.
fn image_too_small(img: &image::DynamicImage, max_size: u32) -> bool {
    let min_size = max_size / 2;

    (img.width() < min_size && img.height() < min_size) && max_size > 0
}

/// Checks if the image is too large to be used as a cover.
///
/// # Arguments
///
/// `img: &image::DynamicImage` - the image to check.
/// `max_size: u32` - the maximum size of the image, in pixels. If the image is larger than this om either size, it is considered too large and needs to be resized down to this size.
///
/// # Returns
///
/// `bool` - true if the image is too large, false otherwise.
fn image_too_large(img: &image::DynamicImage, max_size: u32) -> bool {
    (img.width() > max_size || img.height() > max_size) && max_size > 0
}
