//! Image path related functions

use std::error::Error;
use std::path::{Path, PathBuf};

use common::directory;

use super::covertype::CoverType;
use crate::default_values::DefaultValues;

/// Create the complete path name from the folder and the file name
///
/// # Arguments
///
/// * `folder: &Path` - the folder where the file is located
/// * `filename: &String` - the file name
///
/// # Returns
///
/// * `String` - the complete path name as a string
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let folder = Path::new("/home/user/music");
/// let filename = "cover.jpg";
/// let res = id3tag::formats::images::paths::complete_path(folder, &filename.to_string());
///
/// assert_eq!(res, "/home/user/music/cover.jpg");
/// ```
pub(crate) fn complete_path(folder: &Path, filename: &String) -> String {
    folder
        .join(Path::new(&filename))
        .to_str()
        .unwrap_or_default()
        .to_owned()
}

/// Gathers the cover paths into a single vector that can be used to look for the cover(s) we want.
/// Based on the input, the function will create the vector for:
///   - Front cover (candidates)
///   - Back cover (candidates)
///
/// The function works by iterating through the `picture-search-folder` candidates and joining with the relevant
/// picture arguments - `picture-front-candidate`, `picture-back-candidate`.
///
/// Parameters:
/// `cover_type: CoverType` - the type of cover we wish to consolidate for
/// `cfg: &DefaultValues` - program configuration as surmised from the CLI and config file
///
/// Returns:
/// `Result<Vec<String>, Box<dyn Error>>`: A vector of strings containing the paths to be searched, or an error if something goes wrong.
pub(crate) fn gather_cover_candidates(cover_type: CoverType, cfg: &DefaultValues) -> Vec<String> {
    let search_folders = cfg.search_folders();
    log::debug!("gather_cover_candidates::search_folders = {search_folders:?}");

    let picture_candidates = if cover_type == CoverType::Front {
        cfg.picture_front_candidates()
    } else {
        cfg.picture_back_candidates()
    };

    let mut cover_candidates: Vec<String> = Vec::new();
    for folder in search_folders {
        let current_folder = Path::new(&folder);
        for current_picture in &picture_candidates {
            cover_candidates.push(complete_path(current_folder, current_picture));
        }
    }

    cover_candidates.sort();
    itertools::Itertools::unique(cover_candidates.into_iter()).collect()
}

/// Finds the first image from a list relative to a music file.
/// Grabs the path (ie. directory) of the music file and looks for images relative to this.
/// The function will return with the full path of the first image found, or `Ok(None)` if nothing is found.
///
/// # Arguments
///
/// `music_file: &str` - the name (and full path) of the music file being used as the basis for the search.
/// `image_vec: &Vec<String>` - a vector of string values containing the candidate filenames to be searched.
///
/// # Returns
///
/// `Result<Option<PathBuf>, Box<dyn Error>>` - the full path of the first image found, or `Ok(None)` if nothing is found.
///
/// # Errors
///
/// - Returns an error if the music file does not appear to exist.
/// - Returns an error if the music directory cannot be canonicalized.
/// - Returns an error if the music file's directory cannot be determined.
/// - Returns an error if the image path cannot be canonicalized.
pub(crate) fn find_first_image(
    m_file: &str,
    image_vec: &Vec<String>,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    let music_file = Path::new(m_file);
    if !music_file.exists() {
        return Err(format!("Music file {m_file} does not appear to exist.").into());
    }

    let music_path = music_file.canonicalize()?;
    let music_dir = directory(&common::path_to_string(music_path))?;

    for img_candidate in image_vec {
        let image_path = music_dir.join(Path::new(&img_candidate));
        if image_path.exists() {
            return Ok(Some(image_path.canonicalize()?));
        }
    }

    log::debug!("No images found among the candidates supplied.");
    Ok(None)
}

// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests the `create_complete_path` function
    fn test_complete_path() {
        assert_eq!(
            complete_path(Path::new("/my/path"), &"my_file.txt".to_string()),
            "/my/path/my_file.txt".to_string()
        );
    }

    #[test]
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
        let res = gather_cover_candidates(CoverType::Front, &cfg);
        println!("CoverType::Front res ={:?} ({})", res, res.len());
        assert_eq!(res.len(), 15);
        assert_eq!(res[1], "../front.jpg".to_string());
        assert_eq!(res[3], "./cover.jpg".to_string());
        assert_eq!(res[5], "./front.png".to_string());
        assert_eq!(res[7], "Artwork/front.jpg".to_string());
        assert_eq!(res[9], "Images/cover.jpg".to_string());

        // Test CoverType::Back
        let res = gather_cover_candidates(CoverType::Back, &cfg);
        println!("CoverType::Back res = {:?} ({})", res, res.len());
        assert_eq!(res.len(), 15);
        assert_eq!(res[1], "../back.png".to_string());
        assert_eq!(res[3], "./back.jpg".to_string());
        assert_eq!(res[5], "./backcover.jpg".to_string());
        assert_eq!(res[7], "Artwork/back.png".to_string());
        assert_eq!(res[9], "Images/back.jpg".to_string());
    }

    #[test]
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
