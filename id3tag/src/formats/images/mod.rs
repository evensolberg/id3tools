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

use image::imageops::FilterType;
use image::{self, RgbImage};
use std::error::Error;
use std::path::Path;

// Homegrown stuff
mod covertype;
mod ops;
mod paths;
mod tests;

use crate::default_values::DefaultValues;
use common::directory;
use covertype::{cover_filename_from_config, CoverType};
use ops::{aspect_ratio_ok, cover_needs_resizing};
use paths::{complete_path, find_first_image, find_in_folders, gather_cover_paths};

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
#[allow(clippy::module_name_repetitions)]
pub fn process_images(
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
    let front_cover_path = get_cover_filename(CoverType::Front, music_file, cfg)?;
    let back_cover_path = get_cover_filename(CoverType::Back, music_file, cfg)?;

    // return safely
    log::debug!("front_cover_path = {front_cover_path:?}, back_cover_path = {back_cover_path:?}");
    Ok((front_cover_path, back_cover_path))
}

/// Find the cover image for the given type.
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
fn get_cover_filename(
    cover_type: CoverType,
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    let dry_run = cfg.dry_run.unwrap_or(true);
    let max_size = cfg.picture_max_size.unwrap_or(500);
    let mut cover_path_returned: Option<String> = None;

    // Check that we've been given either a front or back cover
    if cover_type != CoverType::Front && cover_type != CoverType::Back {
        log::error!("Incorrect cover type supplied. Should be CoverType::Front or CoverType::Back. cover_type = {cover_type}");
        return Err("Incorrect cover type supplied. Should be CoverType::Front or CoverType::Back. Exiting the function.".into());
    }

    // Look for the cover file next to the music file and in the candidate paths provided.
    // If found, we need to do a bunch of processing.
    if let Some(cover_found_path) = find_cover(cover_type, music_file, cfg)? {
        let cover_file_name = cover_filename_from_config(cover_type, cfg);
        let music_file_path = directory(music_file)?;

        // TODO: Refactor this into a couple of functions
        // If the cover found is the same as the --picture-XXXXX parameter, we need to check the size of the cover
        cover_path_returned = Some(cover_found_path.clone());
        if cover_file_name == cover_found_path {
            if cover_needs_resizing(&cover_found_path, max_size)? {
                let cp_resize = crate::rename_file::resized_filename(&cover_found_path)?;
                cover_path_returned = Some(cp_resize.clone());
                let _res = create_cover(&cover_found_path, &cp_resize, max_size, dry_run)?;
            }
        // If the cover found is different from the --picture-XXXXX parameter, we need to create the cover.
        } else {
            let cover_output_filename = complete_path(&music_file_path, &cover_file_name);

            if dry_run {
                log::debug!("Not creating the resized cover since we're in dry-run mode.");
            } else {
                let _res =
                    create_cover(&cover_found_path, &cover_output_filename, max_size, dry_run)
                        .unwrap_or_default();
            }
        }
        // return the resulting path
    }

    Ok(cover_path_returned)
}

/// Search for the cover file in the locations provided - alongside the music file or in the search folders.
fn find_cover(
    cover_type: CoverType,
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    // Get the file name from the CLI, based on the type we're looking for.
    // If we're looking for a candidate, the corresponding name will be used for output.
    let cover_file_name = cover_filename_from_config(cover_type, cfg);
    let music_path = directory(music_file)?
        .to_str()
        .unwrap_or_default()
        .to_string();

    // Look for the cover file in the music file's directory and in the config's picture_search_folders.
    let cover_path = if cover_file_name.is_empty() {
        None
    } else {
        find_in_folders(&cover_file_name, &music_path, cfg)
    };

    if cover_path.is_some() {
        return Ok(cover_path);
    }

    // If we get here, we didn't find the cover. Let's see if we can create it from candidates
    let candidate_images = if cover_type == CoverType::Front {
        gather_cover_paths(CoverType::FrontCandidate, cfg)?
    } else {
        gather_cover_paths(CoverType::BackCandidate, cfg)?
    };

    if candidate_images.is_empty() {
        return Ok(None);
    }

    let image_path = find_first_image(music_file, &candidate_images)?;
    if image_path.is_some() {
        let cp = image_path
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        return Ok(Some(cp));
    }

    // return
    Ok(None)
} // fn find_cover()

/// Reads a source image file, resizes it, and writes it to a destination file.
/// If the destination file already exists, it is overwritten.
/// If the source file is an image, but the image is not in the expected ratio, an error is returned.
/// If the `save_file` parameter is set to `true`, the destination file is saved to the source file's directory.
/// Otherwise it is not saved but the byte stream is returned so it can be used to set the cover.
///
/// # Arguments
///
/// * `source_file` - The source file to read.
/// * `dest_file` - The destination file to write.
/// * `max_size` - The maximum size in pixels of the image, both width and height.
/// * `save_file` - Whether to save the destination file to the source file's directory. If set to `false`, the byte stream is returned. If set to `true` the destination file is saved to the source file's directory _and_ the byte stream is returned.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - If everything goes well, the byte stream of the resized image is returned.
/// * `Err(Box<dyn Error>)` - If something goes wrong, an error is returned.
///
pub fn create_cover(
    source: &str,
    destination: &str,
    max_size: u32,
    dry_run: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(source)?;

    if !aspect_ratio_ok(img.width(), img.height()) {
        return Err("Image is not in the expected ratio.".into());
    }

    // Resize the image to the max size.
    let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);

    // Save the new image file
    if dry_run {
        log::debug!("Dry run. Not saving image to file {destination}.");
    } else {
        img_resized.save(Path::new(&destination))?;
    }

    let return_vec = img_resized
        .as_rgb8()
        .unwrap_or(&RgbImage::new(1, 1))
        .to_vec();
    Ok(return_vec)
}

/// Reads the image file and resizes it if needed. Returns the resized image as a vector of bytes.
/// Set `max_size` to 0 to disable resizing.
/// Returns a vector of bytes with the image data.
pub fn read_cover(cover_file: &str, max_size: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(cover_file)?;

    let return_vec: Vec<u8> = if (img.width() > max_size || img.height() > max_size) && max_size > 0
    {
        if !aspect_ratio_ok(img.width(), img.height()) {
            return Err("Image is outside the expected ratio.".into());
        }

        let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);

        // Return safely with the saved image data as a vector for later use.
        img_resized
            .as_rgb8()
            .unwrap_or(&RgbImage::new(1, 1))
            .to_vec()
    } else {
        img.as_rgb8().unwrap_or(&RgbImage::new(1, 1)).to_vec()
    };

    Ok(return_vec)
}
