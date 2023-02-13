//! Image processing module. Contains functions for reading, resizing and writing cover images.

use image;
use image::imageops::FilterType;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

// Homegrown stuff
use crate::default_values::DefaultValues;
use crate::rename_file::resized_filename;
use common::directory;

/// The types of covers we deal with - `Front`, `Back`, `FrontCandidate` and `BackCandidate`
#[derive(PartialEq, Default, Copy, Clone)]
enum CoverType {
    #[default]
    Front,
    Back,
    FrontCandidate,
    BackCandidate,
}

/// Implements the `Display` trait for the `CoverType` enum
impl Display for CoverType {
    /// Display function for the `CoverType`.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Front => write!(f, "front"),
            Self::Back => write!(f, "back"),
            Self::FrontCandidate => write!(f, "front candidate"),
            Self::BackCandidate => write!(f, "back candidate"),
        }
    }
}

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
    Ok((front_cover_path, back_cover_path))
} // fn process_images()

/// Find the cover image for the given type.
///
/// Argi,emts:
///
/// `cover_type: CoverType` - the type of cover, i.e., `CoverType::Front` or `CoverType::Back`
/// `music_file: &str` - the name of the music file being processed. This is used to find relative paths.
/// `config: &DefaultValues` - the configuration for the program, containing parameters such as search paths, dry run, etc.
///
/// Returns:
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
        log::debug!("cover_type = {cover_type}");
        return Err("Incorrect cover type supplied. Should be CoverType::Front or CoverType::Back. Exiting the function.".into());
    }

    // Look for the cover file next to the music file and in the candidate paths provided.
    // If found, we need to do a bunch of processing.
    if let Some(cover_found_path) = find_cover(cover_type, music_file, cfg)? {
        log::debug!("{cover_type} cover found: {cover_found_path:?}");

        // Get the cover name from the config
        let cover_file_name = cover_filename_from_config(cover_type, cfg);
        log::debug!("cover_file_name = {cover_file_name}");

        // Get the path to the music file, so we can save the cover file next to it if needed.
        let music_file_path = directory(music_file)?;
        log::debug!("music_file_path = {music_file_path:?}");

        // TODO: Refactor this into a couple of functions
        // If the cover found is the same as the --picture-XXXXX parameter, we need to check the size of the cover
        if cover_file_name == cover_found_path {
            log::debug!("Cover path parameter equals found path.");
            // Create the picture file.
            cover_path_returned = Some(cover_found_path.clone());
            if cover_needs_resizing(&cover_found_path, max_size)? {
                let cp_resize = crate::rename_file::resized_filename(&cover_found_path)?;
                cover_path_returned = Some(cp_resize.clone());
                let res = create_cover(&cover_found_path, &cp_resize, max_size, dry_run)?;
                log::debug!(
                    "Resized {cover_type} {cp_resize} cover size: {} bytes.",
                    res.len()
                );
            }
        // If the cover found is different from the --picture-XXXXX parameter, we need to create the cover.
        } else {
            log::debug!(
                "{cover_type} cover path parameter differs from found path. Creating cover: {cover_file_name}.",
            );
            // Create the picture file.
            cover_path_returned = Some(cover_found_path.clone());
            let cover_output_filename = create_complete_path(&music_file_path, &cover_file_name);
            log::debug!("cover_output_filename = {cover_output_filename}");

            if dry_run {
                log::debug!("Not creating the resized cover since we're in dry-run mode.");
            } else {
                let res =
                    create_cover(&cover_found_path, &cover_output_filename, max_size, dry_run)
                        .unwrap_or_default();
                log::debug!("Resized {cover_type} cover size: {} bytes.", res.len());
            }
        }
        // return the resulting path
    }

    // Cover not found.
    log::debug!("cover_path_returned = {cover_path_returned:?}");
    Ok(cover_path_returned)
}

/// Returns the cover name from the config, depending on the type we ask for.
fn cover_filename_from_config(cover_type: CoverType, cfg: &DefaultValues) -> String {
    match cover_type {
        CoverType::Front | CoverType::FrontCandidate => cfg
            .picture_front
            .as_ref()
            .unwrap_or(&"front-cover.jpg".to_string())
            .clone(),
        CoverType::Back | CoverType::BackCandidate => cfg
            .picture_back
            .as_ref()
            .unwrap_or(&"back-cover.jpg".to_string())
            .clone(),
    }
}

/// Search for the cover file in the locations provided.
fn find_cover(
    cover_type: CoverType,
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    // Get the front or back cover file name.
    log::debug!("find_cover: config = {:?}", cfg);

    // Get the file name from the CLI, based on the type we're looking for.
    // If we're looking for a candidate, the corresponding name will be used for output.
    let cover_file_name = cover_filename_from_config(cover_type, cfg);
    log::debug!("cover_file_name = {cover_file_name}");

    // Get the path to the music file.
    let music_path = directory(music_file)?
        .to_str()
        .unwrap_or_default()
        .to_string();
    log::debug!("music_path = {music_path}");

    // Look for the cover file in the music file's directory and in the config's picture_search_folders.
    let cover_path = if cover_file_name.is_empty() {
        None
    } else {
        find_in_folders(&cover_file_name, &music_path, cfg)
    };

    if cover_path.is_some() {
        log::debug!("Found cover file: {cover_path:?}");
        return Ok(cover_path);
    }

    // If we get here, we didn't find the cover. Let's see if we can create it from candidates
    log::debug!("No cover file found yet.");
    let candidate_images = if cover_type == CoverType::Front {
        gather_cover_paths(CoverType::FrontCandidate, cfg)?
    } else {
        gather_cover_paths(CoverType::BackCandidate, cfg)?
    };

    if candidate_images.is_empty() {
        log::debug!("No candidate images found.");
        return Ok(None);
    }

    log::debug!("Cover not found. Searching for candidate images: {candidate_images:?}",);
    let image_path = find_first_image(music_file, &candidate_images)?;
    if image_path.is_some() {
        log::debug!("image_path = {image_path:?}");
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

/// Iterates through a list of search folder candidates as specified by the config, and returns the first match.
/// If no match is found, returns None.
/// If the candidate is a relative path, it is resolved relative to the music file's directory.
/// If the candidate is an absolute path, it is used as is.
///
/// # Arguments
///
/// * `filename` - The candidate image file name.
/// * `music_path` - The path to the music file, eg. "./music/". or "/home/user/music/".
/// * `config` - The config object.
///
/// # Returns
/// The path to the candidate image file, or None if no match is found.
fn find_in_folders(filename: &str, music_path: &str, config: &DefaultValues) -> Option<String> {
    for folder_name in config.picture_search_folders.as_ref().unwrap() {
        let cover_path = if folder_name == "." {
            format!("{music_path}/{filename}")
        } else {
            format!("{music_path}/{folder_name}/{filename}")
        };

        if std::path::Path::new(&cover_path).exists() {
            return Some(cover_path);
        }
    }
    None
}

/// Check if the image ratio is within acceptable limits
fn aspect_ratio_is_ok(x: u32, y: u32) -> bool {
    let min_ratio = 1.0 / 2.0; // 1:2 ratio
    let max_ratio = 2.0 / 1.0; // 2:1 ratio

    let ratio = f64::from(x) / f64::from(y);
    (min_ratio..=max_ratio).contains(&ratio)
}

/// Check if the cover needs resizing and if the X:Y ratio is acceptable (i.e. not too wide or tall).
fn cover_needs_resizing(filename: &str, max_size: u32) -> Result<bool, Box<dyn Error>> {
    let img = image::open(filename)?;

    // If image ratio is "reasonable", see if it needs resizing and return accordingly.
    // Otherwise return error.
    if aspect_ratio_is_ok(img.width(), img.height()) {
        if img.width() > max_size || img.height() > max_size {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err("Image is not in the expected ratio.".into())
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
    source: &str,
    destination: &str,
    max_size: u32,
    dry_run: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(source)?;

    if !aspect_ratio_is_ok(img.width(), img.height()) {
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

    let return_vec = img_resized.as_rgb8().unwrap().to_vec();
    Ok(return_vec)
}

/// Reads the image file and resizes it if needed. Returns the resized image as a vector of bytes.
/// Set `max_size` to 0 to disable resizing.
/// Returns a vector of bytes with the image data.
pub fn read_cover(cover_file: &str, max_size: u32) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(cover_file)?;

    let return_vec: Vec<u8> = if (img.width() > max_size || img.height() > max_size) && max_size > 0
    {
        if !aspect_ratio_is_ok(img.width(), img.height()) {
            return Err("Image is outside the expected ratio.".into());
        }

        let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);

        // Return safely with the saved image data as a vector for later use.
        img_resized.as_rgb8().unwrap().to_vec()
    } else {
        img.as_rgb8().unwrap().to_vec()
    };

    Ok(return_vec)
}

/// Gathers the cover paths into a single vector that can be used to look for the cover(s) we want.
/// Based on the input, the function will create the vector for:
///   - Front cover
///   - Back cover
///   - Front cover candidates
///   - Back cover candidates
///
/// The function works by iterating through the `picture-search-folder` candidates and joining with the relevant
/// picture arguments such as `picture-front`, `picture-back` or the corresponding `-candidate` parameters.
/// The function will also return `-resize` versions of the file names.
///
/// Parameters:
/// `cover_type: CoverType` - the type of cover we wish to consolidate for
/// `cfg: &DefaultValues` - program configuration as surmised from the CLI and config file
///
/// Returns:
/// `Result<Vec<String>, Box<dyn Error>>`: A vector of strings containing the paths to be searched, or an error if something goes wrong.
fn gather_cover_paths(
    cover_type: CoverType,
    cfg: &DefaultValues,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res_vec: Vec<String> = Vec::new();

    // Gather the folders - we'll use these in all cases.
    let psf = cfg.picture_search_folders.as_ref();
    let psf = psf.map_or_else(
        || vec![".".to_string(), "..".to_string()],
        std::clone::Clone::clone,
    );
    log::debug!("formats::images::gather_cover_paths::psf = {psf:?}");

    // Depending on the cover type, collect the folder+filename combos, including the "-resized" versions.
    for f in psf {
        let folder = Path::new(&f);
        match cover_type {
            CoverType::Front => {
                if let Some(pn) = &cfg.picture_front {
                    res_vec.push(create_complete_path(folder, pn));
                    res_vec.push(create_complete_resized_path(folder, pn)?);
                } else {
                    return Err("No front cover submitted.".into());
                }
            }
            CoverType::Back => {
                if let Some(pn) = &cfg.picture_back {
                    res_vec.push(create_complete_path(folder, pn));
                    res_vec.push(create_complete_resized_path(folder, pn)?);
                } else {
                    return Err("No back cover submitted.".into());
                }
            }
            CoverType::FrontCandidate => {
                if let Some(pcs) = &cfg.picture_front_candidates {
                    for c in pcs {
                        res_vec.push(create_complete_path(folder, c));
                        res_vec.push(create_complete_resized_path(folder, c)?);
                    }
                } else {
                    return Err("No front cover candidates identified.".into());
                }
            }
            CoverType::BackCandidate => {
                if let Some(pcs) = &cfg.picture_back_candidates {
                    for c in pcs {
                        res_vec.push(create_complete_path(folder, c));
                        res_vec.push(create_complete_resized_path(folder, c)?);
                    }
                } else {
                    return Err("No back cover candidates identified.".into());
                }
            } // CoverType::BackCandidate
        } // match cover_type
    } // for f in psf

    res_vec.sort();
    log::debug!(
        "formats::images::gather_cover_paths::res_vec = {:?}",
        res_vec
    );
    Ok(res_vec)
}

/// Finds the first image from a list relative to a music file.
/// Grabs the path (ie. directory) of the music file and looks for images relative to this.
/// The function will return with the full path of the first image found, or `Ok(None)` if nothing is found.
///
/// Parameters:
/// `music_file: &str` - the name (and full path) of the music file being used as the basis for the search.
/// `image_vec: &Vec<String>` - a vector of string values containing the candidate filenames to be searched.
fn find_first_image(
    music_file: &str,
    image_vec: &Vec<String>,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    if music_file.is_empty() {
        return Err("No music file supplied.".into());
    }

    if image_vec.is_empty() {
        return Err("No image candidates supplied.".into());
    }

    let mf = Path::new(music_file);
    if !mf.exists() {
        return Err(format!("music file {music_file} does not appear to exist.").into());
    }

    let music_path = mf.canonicalize()?;
    log::debug!("music_path = {music_path:?}");

    let music_dir = music_path.parent().unwrap_or_else(|| Path::new("."));
    log::debug!("music_dir = {music_dir:?}");

    for img_candidate in image_vec {
        let image_path = music_dir.join(Path::new(&img_candidate));
        log::debug!("image_path = {image_path:?}");
        if image_path.exists() {
            let image_path = image_path.canonicalize()?;
            return Ok(Some(image_path));
        }
    }

    log::warn!("No images found among the candidates supplied.");

    // Nothing found - return safely
    Ok(None)
}

/// Create the complete path name from the folder and the file name
fn create_complete_path(folder: &Path, filename: &String) -> String {
    folder
        .join(Path::new(&filename))
        .to_str()
        .unwrap_or_default()
        .to_owned()
}

/// Create the complete path name from the folder and the file name with -resized appended.
fn create_complete_resized_path(folder: &Path, filename: &str) -> Result<String, Box<dyn Error>> {
    Ok(folder
        .join(resized_filename(filename)?)
        .to_str()
        .unwrap_or_default()
        .to_owned())
}

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
///
mod tests {
    use crate::rename_file::resized_filename;

    use super::*;
    use assay::assay;
    use std::fs;

    #[assay]
    /// Tests the `create_complete_path` function
    fn test_create_complete_path() {
        assert_eq!(
            create_complete_path(Path::new("/my/path"), &"my_file.txt".to_string()),
            "/my/path/my_file.txt".to_string()
        );
    }

    #[assay]
    /// Tests the `create_complete_resized_path` function
    fn test_create_complete_resized_path() {
        assert_eq!(
            create_complete_resized_path(Path::new("/my/path"), "my_file.txt").unwrap(),
            "/my/path/my_file-resize.txt".to_string()
        );
    }

    #[assay(include = ["../testdata/DSOTM_Back.jpeg", "../testdata/DSOTM_Cover.jpeg", "../testdata/id3tag-config.toml", "../testdata/sample.flac"])]
    /// Tests the `find_cover` function.
    fn test_find_cover() {
        let music_file = "../testdata/sample.flac";
        let fc_filename = "../testdata/DSOTM_Cover.jpeg";
        // let bc_filename = "../testdata/DSOTM_Back.jpeg";

        // Create a config.
        let mut dv = DefaultValues::load_config("../testdata/id3tag-config.toml")?;
        dv.dry_run = Some(false);

        // Create a cover file in the current directory (alongside the music file) with the expected name and then look for that file.
        let _ = create_cover(fc_filename, "../testdata/cover-resized.jpg", 500, false);

        let cover_file = find_cover(CoverType::Front, music_file, &dv)?;
        println!("cover_file = {cover_file:?}");
        assert!(cover_file.is_some());
        // assert_eq!(cover_file.unwrap(), "../testdata/cover-resized.jpg");
        fs::remove_file(Path::new("../testdata/cover-resized.jpg")).unwrap();

        // Create a cover file in the parent directory (of the music file) with the expected name and then look for that file.
        // Note that the cover file name hasn't changed - it's just in a different directory. We should still be able to find it.
        let _ = create_cover(fc_filename, "../cover-resized.jpg", 500, false);
        let cover_file = find_cover(CoverType::Front, music_file, &dv)?;

        assert!(cover_file.is_some());
        // assert_eq!(cover_file.unwrap(), "../testdata/../cover-resized.jpg");
        fs::remove_file(Path::new("../cover-resized.jpg")).unwrap();

        // Create a back cover in the Artwork directory with the expected name and then look for that file.
        // let _ = create_cover(
        //     bc_filename,
        //     "../testdata/Artwork/back-resized.jpg",
        //     500,
        //     false,
        // );
        // let cover_file = find_cover(CoverType::Back, music_file, &dv);
        // assert!(cover_file.is_some());
        // assert_eq!(cover_file.unwrap(), "../testdata/Artwork/back-resized.jpg");
    }

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg"])]
    /// Tests that the `read_cover` function works as expected.
    fn test_read_cover() {
        let cover_file = "../testdata/DSOTM_Cover.jpeg";

        // Read the file without resizing.
        let max_size = 0;
        let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
        println!("Image size: {}", return_vec.len());
        assert!(!return_vec.is_empty());
        assert!(!return_vec.is_empty());
        assert_eq!(return_vec.len(), 3_630_000);
    }

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg"])]
    /// Tests that the `create_cover` function works as expected.
    fn test_create_cover() {
        let src_filename = "../testdata/DSOTM_Cover.jpeg";
        let dst_filename = resized_filename(src_filename).unwrap();
        let max_size = 500;
        let dry_run = false;

        let res = create_cover(src_filename, &dst_filename, max_size, dry_run);
        assert!(res.is_ok());
        let return_vec = res.unwrap();
        println!("Image size: {}", return_vec.len());
        assert!(!return_vec.is_empty());
        assert!(!return_vec.is_empty());
        assert_eq!(return_vec.len(), 750_000);

        // Check that the file was created.
        let res = std::fs::metadata(&dst_filename);
        assert!(res.is_ok());
        let md = res.unwrap();
        assert!(md.is_file());
        // assert_eq!(md.len(), 15_627);

        // Delete the created file.
        let res = std::fs::remove_file(dst_filename);
        assert!(res.is_ok());
    }

    #[assay(include = ["../testdata/DSOTM_Cover.jpeg"])]
    /// Tests that the `needs_resizing` function works as expected.
    fn test_needs_resizing() {
        let fname = "../testdata/DSOTM_Cover.jpeg";

        let res = cover_needs_resizing(fname, 500);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), true);

        let res = cover_needs_resizing(fname, 1100);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), false);
    }

    #[assay]
    /// Tests the `gather_cover_paths` function
    fn test_gather_cover_paths() {
        let mut cfg = DefaultValues::new();
        cfg.picture_front = Some("front.jpg".to_string());
        cfg.picture_back = Some("back.jpg".to_string());
        cfg.picture_front_candidates = Some(vec![
            "cover.jpg".to_string(),
            "front.jpg".to_string(),
            "front.png".to_string(),
        ]);
        cfg.picture_back_candidates = Some(vec![
            "backcover.jpg".to_string(),
            "back.jpg".to_string(),
            "back.png".to_string(),
        ]);
        cfg.picture_search_folders = Some(vec![
            ".".to_string(),
            "..".to_string(),
            "Artwork".to_string(),
            "Scans".to_string(),
            "Images/".to_string(),
        ]);

        // Test CoverType::Front
        let res = gather_cover_paths(CoverType::Front, &cfg);
        println!(
            "CoverType::Front res ={:?} ({})",
            res,
            res.as_ref().unwrap().len()
        );
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().len(), 10);
        assert_eq!(res.as_ref().unwrap()[0], "../front-resize.jpg".to_string());
        assert_eq!(res.as_ref().unwrap()[1], "../front.jpg".to_string());
        assert_eq!(res.as_ref().unwrap()[2], "./front-resize.jpg".to_string());
        assert_eq!(res.as_ref().unwrap()[3], "./front.jpg".to_string());
        assert_eq!(
            res.as_ref().unwrap()[4],
            "Artwork/front-resize.jpg".to_string()
        );
        assert_eq!(res.as_ref().unwrap()[5], "Artwork/front.jpg".to_string());
        assert_eq!(
            res.as_ref().unwrap()[6],
            "Images/front-resize.jpg".to_string()
        );
        assert_eq!(res.as_ref().unwrap()[7], "Images/front.jpg".to_string());
        assert_eq!(
            res.as_ref().unwrap()[8],
            "Scans/front-resize.jpg".to_string()
        );
        assert_eq!(res.as_ref().unwrap()[9], "Scans/front.jpg".to_string());

        // Test CoverType::Back
        let res = gather_cover_paths(CoverType::Back, &cfg);
        println!(
            "CoverType::Back res = {:?} ({})",
            res,
            res.as_ref().unwrap().len()
        );
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().len(), 10);
        assert_eq!(res.as_ref().unwrap()[0], "../back-resize.jpg".to_string());
        assert_eq!(res.as_ref().unwrap()[1], "../back.jpg".to_string());
        assert_eq!(res.as_ref().unwrap()[2], "./back-resize.jpg".to_string());
        assert_eq!(res.as_ref().unwrap()[3], "./back.jpg".to_string());
        assert_eq!(
            res.as_ref().unwrap()[4],
            "Artwork/back-resize.jpg".to_string()
        );
        assert_eq!(res.as_ref().unwrap()[5], "Artwork/back.jpg".to_string());
        assert_eq!(
            res.as_ref().unwrap()[6],
            "Images/back-resize.jpg".to_string()
        );
        assert_eq!(res.as_ref().unwrap()[7], "Images/back.jpg".to_string());
        assert_eq!(
            res.as_ref().unwrap()[8],
            "Scans/back-resize.jpg".to_string()
        );
        assert_eq!(res.as_ref().unwrap()[9], "Scans/back.jpg".to_string());

        // Test CoverType::FrontCandidate
        let res = gather_cover_paths(CoverType::FrontCandidate, &cfg);
        println!(
            "CoverType::FrontCandidate res = {:?} ({})",
            res,
            res.as_ref().unwrap().len()
        );
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().len(), 30);

        // Test CoverType::BackCandidate
        let res = gather_cover_paths(CoverType::BackCandidate, &cfg);
        println!(
            "CoverType::BackCandidate res = {:?} ({})",
            res,
            res.as_ref().unwrap().len()
        );
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().len(), 30);
    }

    #[assay(include = ["../testdata/sample.flac", "../testdata/DSOTM_Cover.jpeg"])]
    /// tests the `find_first_image` function
    fn test_find_first_image() {
        // test the failure scenarios first
        let mut music_file = "";
        let mut image_vec: Vec<String> = Vec::new();

        // Start with everything empty - should fail on the music file.
        let res = find_first_image(music_file, &image_vec);
        assert!(res.is_err());

        // Shoould now fail on the empty vector.
        music_file = "../testdata/somefile.flac";
        let res = find_first_image(music_file, &image_vec);
        assert!(res.is_err());

        // Should now fail on the music file not being found.

        let res = find_first_image(music_file, &image_vec);
        assert!(res.is_err());

        // should now return with None
        music_file = "../testdata/sample.flac";
        image_vec.push("front.jpg".to_string());
        image_vec.push("cover.jpg".to_string());
        let res = find_first_image(music_file, &image_vec).unwrap();
        assert!(res.is_none());

        // Should find DSOTM_Cover.jpeg
        image_vec.clear();
        image_vec.push("front.jpg".to_string());
        image_vec.push("cover.jpg".to_string());
        image_vec.push("../testdata/DSOTM_Cover.jpeg".to_string());
        println!("image_vec = {image_vec:?}");
        let res = find_first_image(music_file, &image_vec);
        println!("res = {res:?}");

        assert!(res.is_ok());
        assert!(res.as_ref().unwrap().is_some());
        println!("res = {res:?}");

        // There's gotta be a better way to do this
        let filename = res.unwrap().unwrap();
        let filename = filename.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "DSOTM_Cover.jpeg");
    }
}
