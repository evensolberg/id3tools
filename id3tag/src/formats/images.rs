//! Image processing module. Contains functions for reading, resizing and writing cover images.

use image;
use image::imageops::FilterType;
use std::error::Error;
use std::path::Path;

use crate::default_values::DefaultValues;

#[derive(PartialEq, Default, Copy, Clone)]
enum CoverType {
    #[default]
    Front,
    Back,
}

/// Catch the image-related CLI parameters and process the image(s).
/// Returns OK if the image(s) were processed successfully. This may change to return the path to the resulting image(s).
#[allow(clippy::too_many_lines, clippy::module_name_repetitions)]
pub fn process_images(
    music_file: &str,
    config: &DefaultValues,
) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
    let dry_run = config.dry_run.unwrap_or(true);
    if dry_run {
        log::debug!("process_images dry run.");
    }
    let max_size = config.picture_max_size.unwrap_or(500);

    // Placeholders for there resulting filenames.
    let mut front_cover_path_resulting = None;
    let mut back_cover_path_resulting = None;

    // Check if the front cover exists. If not, see if we can create it.
    let front_cover_path = find_cover(CoverType::Front, music_file, config);
    if let Some(fcp) = front_cover_path {
        log::debug!("Front cover found: {:?}", fcp);
        front_cover_path_resulting = Some(fcp.clone());
        if cover_needs_resizing(&fcp, max_size)? {
            log::debug!("Resizing front cover.");
            let fcp_resize = crate::rename_file::filename_resize(&fcp)?;
            front_cover_path_resulting = Some(fcp_resize.clone());
            let res = create_cover(&fcp, &fcp_resize, max_size, dry_run)?;
            log::debug!("Resized front cover size: {} bytes.", res.len());
        }
    } else {
        log::debug!("Front cover not found.");
    }

    // Check if the back cover exists. If it does, resize if necessary and return the path.
    let back_cover_path = find_cover(CoverType::Back, music_file, config);
    if let Some(bcp) = back_cover_path {
        log::debug!("Back cover found: {:?}", bcp);
        back_cover_path_resulting = Some(bcp.clone());
        if cover_needs_resizing(&bcp, max_size)? {
            log::debug!("Resizing back cover.");
            let bcp_resize = crate::rename_file::filename_resize(&bcp)?;
            back_cover_path_resulting = Some(bcp_resize.clone());
            let res = create_cover(&bcp, &bcp_resize, max_size, dry_run)?;
            log::debug!("Resized back cover size: {} bytes.", res.len());
        }
    } else {
        log::debug!("Back cover not found.");
    }

    // return safely
    Ok((front_cover_path_resulting, back_cover_path_resulting))
} // fn process_images()

/// Search for the cover file in the locations provided.
fn find_cover(cover_type: CoverType, music_file: &str, config: &DefaultValues) -> Option<String> {
    // Get the front or back cover file name.
    log::debug!("find_cover: config = {:?}", config);

    let cover_file_name = match cover_type {
        CoverType::Front => config.picture_front.clone().unwrap_or_default(),
        CoverType::Back => config.picture_back.clone().unwrap_or_default(),
    };
    log::debug!("cover_file_name = {:?}", cover_file_name);

    // Get the path to the music file.
    let music_path = if let Some(mpath) = Path::new(&music_file).parent() {
        mpath.to_str().unwrap_or_default().to_string()
    } else {
        Path::new(".").to_str().unwrap_or_default().to_string()
    };
    log::debug!("music_path = {:?}", music_path);

    // Look for the cover file in the music file's directory and in the config's picture_search_folders.
    let mut cover_path = None;
    if !cover_file_name.is_empty() {
        cover_path = find_in_folders(&cover_file_name, &music_path, config);
    }
    if cover_path.is_some() {
        log::debug!("Found cover file: {:?}", cover_path.as_ref().unwrap());
        return cover_path;
    }
    log::debug!("No cover file found yet.");

    // If we get here, we didn't find the cover. Let's see if we can create it from the candidate images.
    let candidate_images = if cover_type == CoverType::Front {
        config.picture_front_candidates.clone().unwrap_or_default()
    } else {
        config.picture_back_candidates.clone().unwrap_or_default()
    };

    if candidate_images.is_empty() {
        log::debug!("No candidate images found.");
        return None;
    }
    log::debug!(
        "Cover not found. Searching for candidate images: {:?}",
        candidate_images
    );

    // Look for the cadidate images in the music file's directory and in the config's picture_search_folders.
    for candidate_image in &candidate_images {
        cover_path = find_in_folders(candidate_image, music_file, config);
        if cover_path.is_some() {
            break;
        }
    }

    // return
    cover_path
} // fn find_cover()

/// Iterates through a list of search folder candidates as specified by the config, and returns the first match.
/// If no match is found, returns None.
/// If the candidate is a relative path, it is resolved relative to the music file's directory.
/// If the candidate is an absolute path, it is used as is.
///
/// # Arguments
/// * `filename` - The candidate image file name.
/// * `music_path` - The path to the music file, eg. "./music/". or "/home/user/music/".
/// * `config` - The config object.
///
/// # Returns
/// The path to the candidate image file, or None if no match is found.
fn find_in_folders(filename: &str, music_path: &str, config: &DefaultValues) -> Option<String> {
    for folder_name in config.picture_search_folders.as_ref().unwrap() {
        let cover_path = if folder_name == "." {
            format!("{}/{}", music_path, filename)
        } else {
            format!("{}/{}/{}", music_path, folder_name, filename)
        };
        log::debug!("Checking path: {}", cover_path);
        if std::path::Path::new(&cover_path).exists() {
            log::debug!("Found cover: {}", cover_path);
            return Some(cover_path);
        }
    }
    None
}

/// Check if the cover needs resizing and if the X:Y ratio is acceptable (i.e. not too wide or tall).
fn cover_needs_resizing(filename: &str, max_size: u32) -> Result<bool, Box<dyn Error>> {
    let img_res = image::open(&filename);
    let img = match img_res {
        Ok(img) => img,
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(Box::new(err));
        }
    };

    // The dimensions method returns the images width and height.
    let img_x = img.width();
    let img_y = img.height();
    log::debug!("{} dimensions: {}x{}", filename, img_x, img_y);

    let size_factor = f64::from(img_x) / f64::from(img_y);

    // Check if the image (likely) contains multiple covers.
    let size_factor_str = format!("{:.2}", size_factor);
    log::debug!("{} size factor: {}", filename, size_factor_str);
    if !(0.5..=1.5).contains(&size_factor) {
        return Err("Image is not in the expected ratio.".into());
    }

    // Image ratio is OK - check the size
    log::debug!("Image ratio is OK. Checking size.");
    if img_x > max_size || img_y > max_size {
        log::debug!("File needs resizing. Exiting cover_needs_resizing function.");
        Ok(true)
    } else {
        log::debug!("File does not need resizing. Exiting cover_needs_resizing function.");
        Ok(false)
    }
}

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
    src_filename: &str,
    dst_filename: &str,
    max_size: u32,
    dry_run: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    log::debug!("create_cover: Reading image file: {}", src_filename);
    // Read the source file.
    // `open` returns a `DynamicImage` on success.
    let img_res = image::open(&src_filename);
    let img = match img_res {
        Ok(img) => img,
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(Box::new(err));
        }
    };

    // Check image dimensions
    let img_x = img.width();
    let img_y = img.height();
    log::debug!("{} dimensions: {}x{}", src_filename, img_x, img_y);

    // Check if the iamge (likely) contains multiple covers.
    let size_factor = f64::from(img_x) / f64::from(img_y);
    log::debug!(
        "{} size factor: {}",
        src_filename,
        format!("{:2}", size_factor)
    );
    if !(0.5..=1.5).contains(&size_factor) {
        return Err("Image is not in the expected ratio.".into());
    }

    // Resize the image to the max size.
    let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);
    log::debug!(
        "Resized image dimensions: {}x{}",
        img_resized.width(),
        img_resized.height()
    );

    // Save the new image file
    if dry_run {
        log::debug!("Not saving image to file {}.", dst_filename);
    } else {
        log::debug!("Saving image to file {}.", dst_filename);
        img_resized.save(Path::new(&dst_filename))?;
    }

    // Return safely with the saved image data as a vector for later use.
    let return_vec = img_resized.as_rgb8().unwrap().to_vec();
    log::debug!(
        "Returning image data as vector with length {} bytes. Exiting create_cover function.",
        return_vec.len()
    );
    Ok(return_vec)
}

/// Reads the image file and resizes it if needed. Returns the resized image as a vector of bytes.
/// Set `max_size` to 0 to disable resizing.
/// Returns a vector of bytes with the image data.
pub fn read_cover(cover_file: &str, max_size: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    log::debug!("read_cover: Reading image file: {}", cover_file);
    // Read the source file.
    // `open` returns a `DynamicImage` on success.
    let img_res = image::open(&cover_file);
    let img = match img_res {
        Ok(img) => img,
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(Box::new(err));
        }
    };

    // Check image dimensions
    let img_x = img.width();
    let img_y = img.height();
    log::debug!("{} dimensions: {}x{}", cover_file, img_x, img_y);

    let return_vec: Vec<u8> = if (img_x > max_size || img_y > max_size) && max_size > 0 {
        log::debug!("Resizing.");

        // Check if the iamge (likely) contains multiple covers.
        let size_factor = f64::from(img_x) / f64::from(img_y);
        log::debug!(
            "{} size factor: {}",
            cover_file,
            format!("{:2}", size_factor)
        );
        if !(0.5..=1.5).contains(&size_factor) {
            return Err("Image is not in the expected ratio.".into());
        }

        // Resize the image to the max size.
        let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);
        log::debug!(
            "Resized image dimensions: {}x{}",
            img_resized.width(),
            img_resized.height()
        );

        // Return safely with the saved image data as a vector for later use.
        img_resized.as_rgb8().unwrap().to_vec()
    } else {
        log::debug!("File does not need resizing.");
        img.as_rgb8().unwrap().to_vec()
    };

    log::debug!(
        "Returning image data as vector with length {} bytes. Exiting create_cover function.",
        return_vec.len()
    );
    Ok(return_vec)
}

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;
    use std::fs;

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg", "../testdata/id3tag-config.toml", "../testdata/sample.flac", "../testdata/DSOTM_Back.jpeg"])]
    /// Tests the find_cover function.
    fn test_find_cover() {
        let music_file = "../testdata/sample.flac";
        let fc_filename = "../testdata/DSOTM_Cover.jpeg";
        // let bc_filename = "../testdata/DSOTM_Back.jpeg";

        // Create a config.
        let mut dv = DefaultValues::load_config("../testdata/id3tag-config.toml")?;
        dv.dry_run = Some(false);

        // Create a cover file in the current directory (alongside the music file) with the expected name and then look for that file.
        let _ = create_cover(fc_filename, "../testdata/cover-small.jpg", 500, false);

        let cover_file = find_cover(CoverType::Front, music_file, &dv);
        println!("cover_file = {:?}", cover_file);
        assert!(cover_file.is_some());
        assert_eq!(cover_file.unwrap(), "../testdata/cover-small.jpg");
        fs::remove_file(Path::new("../testdata/cover-small.jpg")).unwrap();

        // Create a cover file in the parent directory (of the music file) with the expected name and then look for that file.
        // Note that the cover file name hasn't changed - it's just in a different directory. We should still be able to find it.
        let _ = create_cover(fc_filename, "../cover-small.jpg", 500, false);
        let cover_file = find_cover(CoverType::Front, music_file, &dv);

        assert!(cover_file.is_some());
        assert_eq!(cover_file.unwrap(), "../testdata/../cover-small.jpg");
        fs::remove_file(Path::new("../cover-small.jpg")).unwrap();

        // Create a back cover in the Artwork directory with the expected name and then look for that file.
        // let _ = create_cover(
        //     bc_filename,
        //     "../testdata/Artwork/back-small.jpg",
        //     500,
        //     false,
        // );
        // let cover_file = find_cover(CoverType::Back, music_file, &dv);
        // assert!(cover_file.is_some());
        // assert_eq!(cover_file.unwrap(), "../testdata/Artwork/back-small.jpg");
    }

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg"])]
    /// Tests that the read_cover function works as expected.
    fn test_read_cover() {
        let cover_file = "../testdata/DSOTM_Cover.jpeg";

        // Read the file without resizing.
        let max_size = 0;
        let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
        println!("Image size: {}", return_vec.len());
        assert!(!return_vec.is_empty());
        assert!(return_vec.len() > 0);
        assert_eq!(return_vec.len(), 3_630_000);
    }

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg"])]
    /// Tests that the create_cover function works as expected.
    fn test_create_cover() {
        let src_filename = "../testdata/DSOTM_Cover.jpeg";
        let dst_filename = "../testdata/DSOTM_Cover_resized.jpeg";
        let max_size = 500;
        let dry_run = false;

        let res = create_cover(src_filename, dst_filename, max_size, dry_run);
        assert!(res.is_ok());
        let return_vec = res.unwrap();
        println!("Image size: {}", return_vec.len());
        assert!(!return_vec.is_empty());
        assert!(return_vec.len() > 0);
        assert_eq!(return_vec.len(), 750_000);

        // Check that the file was created.
        let res = std::fs::metadata(dst_filename);
        assert!(res.is_ok());
        let md = res.unwrap();
        assert!(md.is_file());
        // assert_eq!(md.len(), 15_627);

        // Delete the created file.
        let res = std::fs::remove_file(dst_filename);
        assert!(res.is_ok());
    }

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg"])]
    /// Tests that the needs_resizing function works as expected.
    fn test_needs_resizing() {
        let fname = "../testdata/DSOTM_Cover.jpeg";

        let res = cover_needs_resizing(fname, 500);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), true);

        let res = cover_needs_resizing(fname, 1100);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), false);
    }
}
