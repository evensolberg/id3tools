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
    assert!(!return_vec.is_empty());
    assert_eq!(return_vec.len(), 52_429);
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
    assert_eq!(return_vec.len(), 52_429);

    // Read the file with resizing.
    let max_size = 300;
    let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert_eq!(return_vec.len(), 52_429);

    // Read the file with resizing.
    let max_size = 2500;
    let return_vec = read_cover(cover_file, max_size).unwrap_or_default();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert_eq!(return_vec.len(), 52_429);
}
