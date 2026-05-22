// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
use super::*;
// use crate::rename_file::resized_filename;

#[test]
/// Tests that the `read_cover` function works as expected.
fn test_read_cover() {
    let cover_file = "../testdata/DSOTM_Cover.jpeg";
    // Skip if testdata is not available (e.g. in CI without LFS files)
    if !std::path::Path::new(cover_file).exists() {
        return;
    }

    // Read the file without resizing.
    let max_size = 0;
    let (return_vec, mime_type) = read_cover(cover_file, max_size).unwrap();
    println!("Image size: {}, mime: {mime_type}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert_eq!(mime_type, "image/jpeg");
    // Allow for slight variation in JPEG encoder output across versions
    assert!(
        (50_000..=55_000).contains(&return_vec.len()),
        "Expected image size ~52K, got {}",
        return_vec.len()
    );
}

// Test image sizes
#[test]
/// Tests that the image resizing works as expected.
fn test_image_resizing() {
    let cover_file = "../testdata/DSOTM_Cover.jpeg";
    // Skip if testdata is not available (e.g. in CI without LFS files)
    if !std::path::Path::new(cover_file).exists() {
        return;
    }

    // Read the file without resizing.
    let max_size = 0;
    let (return_vec, _) = read_cover(cover_file, max_size).unwrap();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert!(
        (50_000..=55_000).contains(&return_vec.len()),
        "Expected image size ~52K, got {}",
        return_vec.len()
    );

    // Read the file with resizing.
    let max_size = 300;
    let (return_vec, _) = read_cover(cover_file, max_size).unwrap();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert!(
        (6_000..=8_000).contains(&return_vec.len()),
        "Expected resized image size ~7K, got {}",
        return_vec.len()
    );

    // Read the file with resizing.
    let max_size = 2500;
    let result = read_cover(cover_file, max_size);
    assert!(result.is_err());
}
