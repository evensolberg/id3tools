use std::error::Error;
use std::path::Path;

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

#[cfg(test)]
///
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
}
