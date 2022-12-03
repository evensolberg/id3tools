//! Image processing module. Contains functions for reading, resizing and writing cover images.

use image;
use image::imageops::FilterType;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::canonicalize;
use std::path::{Path, PathBuf};

use crate::default_values::DefaultValues;
use crate::rename_file::filename_resize;

#[derive(PartialEq, Default, Copy, Clone)]
enum CoverType {
    #[default]
    Front,
    Back,
    FrontCandidate,
    BackCandidate,
}

impl Display for CoverType {
    /// Display function for the CoverType.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CoverType::Front => write!(f, "front"),
            CoverType::Back => write!(f, "back"),
            CoverType::FrontCandidate => write!(f, "front candidate"),
            CoverType::BackCandidate => write!(f, "back candidate"),
        }
    }
}

/// Catch the image-related CLI parameters and process the image(s).
/// Returns OK if the image(s) were processed successfully. This may change to return the path to the resulting image(s).
#[allow(clippy::module_name_repetitions)]
pub fn process_images(
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
    if cfg.dry_run.unwrap_or(true) {
        log::debug!("process_images dry run.");
    }

    // Check if the front cover path is set. Return None if not, otherwise return the path.
    let fcp = get_cover(CoverType::Front, music_file, cfg)?;
    let fcp_r = if fcp.is_empty() { None } else { Some(fcp) };

    // Check if the back cover path is set. Return None if not, otherwise return the path.
    let bcp = get_cover(CoverType::Back, music_file, cfg)?;
    let bcp_r = if bcp.is_empty() { None } else { Some(bcp) };

    // return safely
    Ok((fcp_r, bcp_r))
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
///
/// To do:
///
/// Change the `String` to `Option<String>` to be able to return `None` if nothing is found.
fn get_cover(
    cover_type: CoverType,
    music_file: &str,
    cfg: &DefaultValues,
) -> Result<String, Box<dyn Error>> {
    // TODO: Consider replacing the String with Option<String> so we can return None if not found.
    let dry_run = cfg.dry_run.unwrap_or(true);
    let max_size = cfg.picture_max_size.unwrap_or(500);
    let mut cover_path_returned = String::new();

    if let Some(cover_found_path) = find_cover(cover_type, music_file, cfg)? {
        log::debug!("{} cover found: {:?}", cover_type, cover_found_path);

        // Get the cover name from the config.
        let cover_file_name = if cover_type == CoverType::Front {
            cfg.picture_front
                .as_ref()
                .unwrap_or(&"front-cover.jpg".to_string())
                .to_owned()
        } else {
            cfg.picture_back
                .as_ref()
                .unwrap_or(&"back-cover.jpg".to_string())
                .to_owned()
        };

        // Get the path to the music file, so we can save the cover file next to it if needed.
        let mut music_file_path = canonicalize(music_file)?;
        music_file_path = music_file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        log::debug!("music_file_path = {:?}", music_file_path);

        // If the cover found is different from the --picture-XXXXX parameter, we need to create the cover.
        if cover_file_name != cover_found_path {
            log::debug!(
                "{cover_type} cover path parameter differs from found path. Creating cover: {cover_file_name}.",
            );
            // Create the picture file.
            cover_path_returned = cover_found_path.clone();
            let cover_output_filename = music_file_path
                .join(cover_file_name)
                .to_str()
                .unwrap()
                .to_owned();
            if !dry_run {
                let res =
                    create_cover(&cover_found_path, &cover_output_filename, max_size, dry_run)
                        .unwrap_or_default();
                log::debug!("Resized {cover_type} cover size: {} bytes.", res.len());
            } else {
                log::debug!("Not creating the resized cover since we're in dry-run mode.")
            }
        } else {
            // If the cover found is the same as the --picture-XXXXX parameter, we need to check the size of the cover
            log::debug!("Cover path parameter equals found path.");
            // Create the picture file.
            cover_path_returned = cover_found_path.clone();
            if cover_needs_resizing(&cover_found_path, max_size)? {
                log::debug!("Resizing {cover_type} cover.");
                let cp_resize = crate::rename_file::filename_resize(&cover_found_path)?;
                cover_path_returned = cp_resize.clone();
                let res = create_cover(&cover_found_path, &cp_resize, max_size, dry_run)?;
                log::debug!("Resized {cover_type} cover size: {} bytes.", res.len());
            }
        }
    } else {
        log::debug!("Cover not found.");
    }

    // return the resulting path
    Ok(cover_path_returned)
}

/// Search for the cover file in the locations provided.
fn find_cover(
    cover_type: CoverType,
    music_file: &str,
    config: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    // Get the front or back cover file name.
    log::debug!("find_cover: config = {:?}", config);

    // Get the file name from the CLI, based on the type we're looking for.
    // If we're looking for a candidate, the corresponding name will be used for output.
    let cover_file_name = match cover_type {
        CoverType::Front | CoverType::FrontCandidate => {
            config.picture_front.clone().unwrap_or_default()
        }
        CoverType::Back | CoverType::BackCandidate => {
            config.picture_back.clone().unwrap_or_default()
        }
    };
    log::debug!("cover_file_name = {:?}", cover_file_name);

    // Get the path to the music file.
    let music_path = if let Some(p) = canonicalize(Path::new(&music_file))?.as_path().parent() {
        p.to_str().unwrap_or_default().to_string()
    } else {
        canonicalize(Path::new("."))?
            .as_path()
            .to_str()
            .unwrap_or_default()
            .to_string()
    };
    log::debug!("music_path = {:?}", music_path);

    // Look for the cover file in the music file's directory and in the config's picture_search_folders.
    let mut cover_path = None;
    if !cover_file_name.is_empty() {
        cover_path = find_in_folders(&cover_file_name, &music_path, config);
    }
    if cover_path.is_some() {
        log::debug!("Found cover file: {:?}", cover_path.as_ref().unwrap());
        return Ok(cover_path);
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
        return Ok(None);
    }
    log::debug!(
        "Cover not found. Searching for candidate images: {:?}",
        candidate_images
    );

    // Look for the cadidate images in the music file's directory and in the config's picture_search_folders.
    for candidate_image in &candidate_images {
        cover_path = find_in_folders(candidate_image, &music_path, config);
        if cover_path.is_some() {
            break;
        }
    }

    // return
    Ok(cover_path)
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
    let x = img.width();
    let y = img.height();
    log::debug!("{} dimensions: {}x{}", src_filename, x, y);

    // Check if the iamge (likely) contains multiple covers.
    let size_factor = f64::from(x) / f64::from(y);
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
    let img_res = image::open(&cover_file);
    let img = match img_res {
        Ok(img) => img,
        Err(err) => {
            log::error!("Error: {}", err);
            return Err(Box::new(err));
        }
    };

    // Check image dimensions
    let x = img.width();
    let y = img.height();
    log::debug!("{} dimensions: {}x{}", cover_file, x, y);

    let return_vec: Vec<u8> = if (x > max_size || y > max_size) && max_size > 0 {
        log::debug!("Resizing.");

        // Check if the image (likely) contains multiple covers.
        let size_factor = f64::from(x) / f64::from(y);
        log::debug!(
            "{} size factor: {}",
            cover_file,
            format!("{:2}", size_factor)
        );
        if !(0.5..=1.5).contains(&size_factor) {
            return Err("Image is not in the expected ratio (0.5..=1.5).".into());
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
    let psf = if let Some(ps) = psf {
        ps.to_owned()
    } else {
        vec![".".to_string(), "..".to_string()]
    };
    log::debug!("formats::images::gather_cover_paths::psf = {:?}", psf);

    // Depending on the cover type, collect the folder+filename combos
    for f in psf {
        let folder = Path::new(&f);
        match cover_type {
            CoverType::Front => {
                if let Some(pn) = &cfg.picture_front {
                    let p = Path::new(&pn);
                    let ppath = folder.join(&p).to_str().unwrap_or_default().to_owned();
                    res_vec.push(ppath);
                    let pr = filename_resize(&pn)?;
                    let prp = folder.join(&pr).to_str().unwrap_or_default().to_owned();
                    res_vec.push(prp);
                } else {
                    return Err("No front cover submitted.".into());
                }
            }
            CoverType::Back => {
                if let Some(pn) = &cfg.picture_back {
                    let p = Path::new(&pn);
                    let ppath = folder.join(&p).to_str().unwrap_or_default().to_owned();
                    res_vec.push(ppath);
                    let pr = filename_resize(&pn)?;
                    let prp = folder.join(&pr).to_str().unwrap_or_default().to_owned();
                    res_vec.push(prp);
                } else {
                    return Err("No back cover submitted.".into());
                }
            }
            CoverType::FrontCandidate => {
                if let Some(pcs) = &cfg.picture_front_candidates {
                    for c in pcs {
                        let pc = Path::new(&c);
                        let ppath = folder.join(&pc).to_str().unwrap_or_default().to_owned();
                        res_vec.push(ppath);
                        let pr = filename_resize(&c)?;
                        let prp = folder.join(&pr).to_str().unwrap_or_default().to_owned();
                        res_vec.push(prp);
                    }
                } else {
                    return Err("No front cover candidates identified.".into());
                }
            }
            CoverType::BackCandidate => {
                if let Some(pcs) = &cfg.picture_back_candidates {
                    for c in pcs {
                        let pc = Path::new(&c);
                        let ppath = folder.join(&pc).to_str().unwrap_or_default().to_owned();
                        res_vec.push(ppath);
                        let pr = filename_resize(&c)?;
                        let prp = folder.join(&pr).to_str().unwrap_or_default().to_owned();
                        res_vec.push(prp);
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
        return Err(format!("music file {} does not appear to exist.", music_file).into());
    }

    let music_path = mf.canonicalize()?;
    log::debug!("music_path = {:?}", music_path);

    let music_dir = music_path.parent().unwrap_or_else(|| Path::new("."));
    log::debug!("music_dir = {:?}", music_dir);

    for img_candidate in image_vec {
        let image_path = music_dir.join(Path::new(&img_candidate));
        log::debug!("image_path = {:?}", image_path);
        if image_path.exists() {
            let image_path = image_path.canonicalize()?;
            return Ok(Some(image_path));
        }
    }

    log::warn!("No images found among the candidates supplied.");

    // Nothing found - return safely
    Ok(None)
}

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
///
mod tests {
    use crate::rename_file::filename_resize;

    use super::*;
    use assay::assay;
    use std::fs;

    #[assay(include = ["../testdata/DSOTM_Back.jpeg", "../testdata/DSOTM_Cover.jpeg", "../testdata/id3tag-config.toml", "../testdata/sample.flac"])]
    /// Tests the find_cover function.
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
        println!("cover_file = {:?}", cover_file);
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
        let dst_filename = filename_resize(&src_filename).unwrap();
        let max_size = 500;
        let dry_run = false;

        let res = create_cover(src_filename, &dst_filename, max_size, dry_run);
        assert!(res.is_ok());
        let return_vec = res.unwrap();
        println!("Image size: {}", return_vec.len());
        assert!(!return_vec.is_empty());
        assert!(return_vec.len() > 0);
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

    #[assay]
    /// Tests the gather_cover_paths function
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
    /// tests the find_first_image function
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
        image_vec.push("front.jpg".to_string());
        image_vec.push("cover.jpg".to_string());
        let res = find_first_image(music_file, &image_vec);
        assert!(res.is_err());

        // Should find something
        music_file = "../testdata/sample.flac";
        image_vec.push("../testdata/DSOTM_Cover.jpeg".to_string());
        let res = find_first_image(music_file, &image_vec);
        assert!(res.is_ok());
        println!("res = {:?}", res);
    }
}
