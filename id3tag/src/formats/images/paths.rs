//! Image path related functions

use std::error::Error;
use std::path::{Path, PathBuf};

use super::covertype::CoverType;
use crate::default_values::DefaultValues;

/// Create the complete path name from the folder and the file name
pub fn complete_path(folder: &Path, filename: &String) -> String {
    folder
        .join(Path::new(&filename))
        .to_str()
        .unwrap_or_default()
        .to_owned()
}

/// Create the complete path name from the folder and the file name with -resized appended.
pub fn complete_resized_path(folder: &Path, filename: &str) -> Result<String, Box<dyn Error>> {
    Ok(folder
        .join(crate::rename_file::resized_filename(filename)?)
        .to_str()
        .unwrap_or_default()
        .to_owned())
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
pub fn gather_cover_paths(
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

    // Depending on the cover type, collect the folder+filename combos, including the "-resized" versions.
    for f in psf {
        let folder = Path::new(&f);
        match cover_type {
            CoverType::Front => {
                if let Some(pn) = &cfg.picture_front {
                    res_vec.push(complete_path(folder, pn));
                    res_vec.push(complete_resized_path(folder, pn)?);
                } else {
                    return Err("No front cover submitted.".into());
                }
            }
            CoverType::Back => {
                if let Some(pn) = &cfg.picture_back {
                    res_vec.push(complete_path(folder, pn));
                    res_vec.push(complete_resized_path(folder, pn)?);
                } else {
                    return Err("No back cover submitted.".into());
                }
            }
            CoverType::FrontCandidate => {
                if let Some(pcs) = &cfg.picture_front_candidates {
                    for c in pcs {
                        res_vec.push(complete_path(folder, c));
                        res_vec.push(complete_resized_path(folder, c)?);
                    }
                } else {
                    return Err("No front cover candidates identified.".into());
                }
            }
            CoverType::BackCandidate => {
                if let Some(pcs) = &cfg.picture_back_candidates {
                    for c in pcs {
                        res_vec.push(complete_path(folder, c));
                        res_vec.push(complete_resized_path(folder, c)?);
                    }
                } else {
                    return Err("No back cover candidates identified.".into());
                }
            } // CoverType::BackCandidate
        } // match cover_type
    } // for f in psf

    res_vec.sort();

    Ok(res_vec)
}

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
pub fn find_in_folders(filename: &str, music_path: &str, config: &DefaultValues) -> Option<String> {
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

/// Finds the first image from a list relative to a music file.
/// Grabs the path (ie. directory) of the music file and looks for images relative to this.
/// The function will return with the full path of the first image found, or `Ok(None)` if nothing is found.
///
/// Parameters:
/// `music_file: &str` - the name (and full path) of the music file being used as the basis for the search.
/// `image_vec: &Vec<String>` - a vector of string values containing the candidate filenames to be searched.
pub fn find_first_image(
    music_file: &str,
    image_vec: &Vec<String>,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let mf = Path::new(music_file);
    if !mf.exists() {
        return Err(format!("music file {music_file} does not appear to exist.").into());
    }

    let music_path = mf.canonicalize()?;
    let music_dir = music_path.parent().unwrap_or_else(|| Path::new("."));

    for img_candidate in image_vec {
        let image_path = music_dir.join(Path::new(&img_candidate));
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
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    /// Tests the `create_complete_path` function
    fn test_complete_path() {
        assert_eq!(
            complete_path(Path::new("/my/path"), &"my_file.txt".to_string()),
            "/my/path/my_file.txt".to_string()
        );
    }

    #[assay]
    /// Tests the `create_complete_resized_path` function
    fn test_create_complete_resized_path() {
        assert_eq!(
            complete_resized_path(Path::new("/my/path"), "my_file.txt").unwrap(),
            "/my/path/my_file-resize.txt".to_string()
        );
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
