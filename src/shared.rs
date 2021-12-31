use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::default_values::DefaultValues;

#[derive(Debug, Default, Clone, Copy)]
pub struct Counts {
    pub total_file_count: usize,
    pub processed_file_count: usize,
    pub skipped_file_count: usize,
}

/// Find the MIME type (ie. `image/[bmp|gif|jpeg|png|tiff`) based on the file extension. Not perfect, but it'll do for now.
pub fn mime_type(filename: &str) -> Result<String, Box<dyn Error>> {
    let ext = get_extension(filename);
    let fmt_str = match ext.as_ref() {
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "tif" | "tiff" => "image/tiff",
        _ => {
            return Err(
                "Image format not supported. Must be one of BMP, GIF, JPEG, PNG or TIFF.".into(),
            )
        }
    };

    // Return safely
    Ok(fmt_str.to_string())
}

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Looks for the picture file with the name supplied. Initially tries to find it in the path of the music file.
/// If unsuccessful, tries to find it in the invocation directory. If still unsuccessful returns either None or
/// an Error, depending on whether the `stop_on_error` flag has been set.
pub fn find_picture(
    m_filename: &str,
    p_filename: &str,
    config: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    // Assume that the music file exists
    let m_path_name;
    if let Some(base_path) = Path::new(&m_filename).parent() {
        m_path_name = base_path;
    } else {
        m_path_name = Path::new(".");
    };

    log::debug!("music path_name = {:?}", m_path_name);

    if Path::new(m_path_name).join(p_filename).exists() {
        // Picture file exists alongside the music file
        log::debug!(
            "picture file path: {}",
            Path::new(m_path_name).join(p_filename).to_string_lossy()
        );
        return Ok(Some(
            Path::new(m_path_name)
                .join(p_filename)
                .to_str()
                .unwrap()
                .to_string(),
        ));
    } else if Path::new(p_filename).exists() {
        // Picture file exists in the invocation path
        log::debug!("p_filename = {}", p_filename);
        return Ok(Some(Path::new(p_filename).to_str().unwrap().to_string()));
    } else if config.stop_on_error.unwrap_or(false) {
        // No picture found - act accordingly
        return Err(format!("Picture file {} does not exist.", p_filename).into());
    } else {
        return Ok(None);
    }
}
