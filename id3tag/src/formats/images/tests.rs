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

    // Read the file without resizing.
    let max_size = 0;
    let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
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

    // Read the file without resizing.
    let max_size = 0;
    let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert!(
        (50_000..=55_000).contains(&return_vec.len()),
        "Expected image size ~52K, got {}",
        return_vec.len()
    );

    // Read the file with resizing.
    let max_size = 300;
    let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert!(
        (6_000..=8_000).contains(&return_vec.len()),
        "Expected resized image size ~7K, got {}",
        return_vec.len()
    );

    // Read the file with resizing.
    let max_size = 2500;
    let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
    println!("Image size: {}", return_vec.len());
    assert!(return_vec.is_empty());
}
