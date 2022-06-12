//! Image processing module. Contains functions for reading, resizing and writing cover images.

use clap;
use image;
use image::imageops::FilterType;
use std::error::Error;
use std::path::Path;

/// Catch the image-related CLI parameters and process the image(s).
/// Returns OK if the image(s) were processed successfully. This may change to return the path to the resulting image(s).
pub fn process_images(
    cli_args: &clap::ArgMatches,
    music_file: &str,
    dry_run: bool,
) -> Result<(), Box<dyn Error>> {
    if dry_run {
        log::debug!("process_images dry run.");
    }

    // Get the various parameters from the CLI.
    let pfc = cli_args.values_of("picture-front-candidate").unwrap();
    log::debug!("picture-front-candiates = {:?}", pfc);

    let pbc = cli_args
        .values_of("picture-back-candidate")
        .unwrap_or_default();
    log::debug!("picture-back-candidates = {:?}", pbc);

    let mut psf: Vec<&str> = cli_args
        .values_of("picture-search-folder")
        .unwrap_or_default()
        .collect();

    // If current and parent folders aren't included in search, just add them.
    if !psf.contains(&".") {
        log::debug!("Adding '.' to search path.");
        psf.push(".");
    }
    if !psf.contains(&"..") {
        log::debug!("Adding '..' to search path.");
        psf.push("..");
    }
    log::debug!("picture-search-folder = {:?}", psf);

    let pms = cli_args
        .value_of("picture-max-size")
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);
    log::debug!("picture-max-size = {:?}", pms);

    // Get the full path of the music file - we need to add our search folder(s) to it.
    let music_path = if let Some(mpath) = Path::new(&music_file).parent() {
        mpath.to_str().unwrap_or_default().to_string()
    } else {
        Path::new(".").to_str().unwrap_or_default().to_string()
    };
    log::debug!("music_path = {:?}", music_path);

    // Placeholders for the front and back cover image file locations.
    let mut full_pf = "".to_string();
    let mut full_pb = "".to_string();

    // Front cover
    if cli_args.is_present("picture-front") {
        let pf = cli_args.value_of("picture-front").unwrap_or_default();
        log::debug!("picture-front = {:?}", pf);
        if let Some(found) = find_cover(&pf, &music_file, psf.clone()) {
            full_pf = found;
            log::debug!("Front cover found at: {}", full_pf);
        } else {
            log::debug!("Front cover not found.")
        }
        if cover_needs_resizing(&full_pf, pms)? {
            create_cover(&full_pf, &full_pf, pms)?;
        }
    }

    // Back cover
    if cli_args.is_present("picture-back") {
        let pb = cli_args.value_of("picture-back").unwrap_or_default();
        log::debug!("picture-back = {:?}", pb);

        log::debug!("full_pb = {:?}", full_pb);
        if let Some(found) = find_cover(&pb, &music_file, psf) {
            full_pb = found;
            log::debug!("Back cover found at: {}", full_pb);
        } else {
            log::debug!("Back cover not found.")
        }
    }

    // return safely
    Ok(())
} // end of process_images()

/// Search for the cover file in the locations provided.
fn find_cover(cover_name: &str, music_file: &str, paths: Vec<&str>) -> Option<String> {
    let music_path = if let Some(mpath) = Path::new(&music_file).parent() {
        mpath.to_str().unwrap_or_default().to_string()
    } else {
        Path::new(".").to_str().unwrap_or_default().to_string()
    };
    log::debug!("music_path = {:?}", music_path);

    for folder_name in paths {
        let cover_path = format!("{}/{}/{}", music_path, folder_name, cover_name);
        log::debug!("Checking path: {}", cover_path);
        if std::path::Path::new(&cover_path).exists() {
            log::debug!("Found cover: {}", cover_path);
            return Some(cover_path.to_string());
        }
    }

    // Not found.
    None
}

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
    log::debug!("{} dimensions {}x{}", filename, img_x, img_y);

    let size_factor = (img_x / img_y) as f32;

    // Check if the image (likely) contains multiple covers.
    if size_factor > 1.5 || size_factor < 0.5 {
        return Err("Image is not in the expected ratio.".into());
    }

    // Image ratio is OK - check the size
    if img_x > max_size || img_y > max_size {
        Ok(true)
    } else {
        Ok(false)
    }
} // fn cover_needs_resizing

/// Read the image file and write the resulting image to file and buffer.
fn create_cover(
    src_filename: &str,
    dst_filename: &str,
    max_size: u32,
) -> Result<Vec<u8>, Box<dyn Error>> {
    log::debug!("Reading image file: {}", src_filename);
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

    // The dimensions method returns the images width and height.
    let img_x = img.width();
    let img_y = img.height();
    log::debug!("src_filename dimensions {}x{}", img_x, img_y);

    let size_factor = (img_x / img_y) as f32;
    if size_factor >= 1.0 {
        log::debug!("Image is landscape.");
    } else {
        log::debug!("Image is portrait.");
    }

    // Check if the iamge (likely) contains multiple covers.
    if size_factor > 1.5 || size_factor < 0.5 {
        return Err("Image is not in the expected ratio.".into());
    }

    let img_resized = img.resize(max_size, max_size, FilterType::Lanczos3);
    img_resized.save(Path::new(&dst_filename))?;

    // return safely
    let return_vec = img_resized.as_rgb8().unwrap().to_vec();
    Ok(return_vec)
} // end of create_cover()
