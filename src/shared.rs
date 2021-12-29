use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

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
        .unwrap()
        .to_string()
}
