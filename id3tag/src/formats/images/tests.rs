// --------------------------------------------------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
use super::*;
// use crate::rename_file::resized_filename;

// #[test]
// /// Tests the `find_cover` function.
// fn test_find_cover() {
//     let music_file = "../testdata/sample.flac";
//     let fc_filename = "../testdata/DSOTM_Cover.jpeg";
//     // let bc_filename = "../testdata/DSOTM_Back.jpeg";

//     // Create a config.
//     let mut dv = DefaultValues::load_config("../testdata/id3tag-config.toml").unwrap();
//     dv.dry_run = Some(false);

//     // Create a cover file in the current directory (alongside the music file) with the expected name and then look for that file.
//     let _ = create_cover(fc_filename, "../testdata/cover-resized.jpg", 500, false);

//     let cover_file = find_cover(CoverType::Front, music_file, &dv).unwrap();
//     println!("cover_file = {cover_file:?}");
//     assert!(cover_file.is_some());
//     // assert_eq!(cover_file.unwrap(), "../testdata/cover-resized.jpg");
//     std::fs::remove_file(Path::new("../testdata/cover-resized.jpg")).unwrap();

//     // Create a cover file in the parent directory (of the music file) with the expected name and then look for that file.
//     // Note that the cover file name hasn't changed - it's just in a different directory. We should still be able to find it.
//     let _ = create_cover(fc_filename, "../cover-resized.jpg", 500, false);
//     let cover_file = find_cover(CoverType::Front, music_file, &dv).unwrap();

//     assert!(cover_file.is_some());
//     // assert_eq!(cover_file.unwrap(), "../testdata/../cover-resized.jpg");
//     std::fs::remove_file(Path::new("../cover-resized.jpg")).unwrap();

//     // Create a back cover in the Artwork directory with the expected name and then look for that file.
//     // let _ = create_cover(
//     //     bc_filename,
//     //     "../testdata/Artwork/back-resized.jpg",
//     //     500,
//     //     false,
//     // );
//     // let cover_file = find_cover(CoverType::Back, music_file, &dv);
//     // assert!(cover_file.is_some());
//     // assert_eq!(cover_file.unwrap(), "../testdata/Artwork/back-resized.jpg");
// }

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
    assert_eq!(return_vec.len(), 3_630_000);
}

#[test]
/// Tests that the `create_cover` function works as expected.
fn test_create_cover() {
    let src_filename = "../testdata/DSOTM_Cover.jpeg";
    let dst_filename = crate::rename_file::resized_filename(src_filename).unwrap();
    let max_size = 500;
    let dry_run = false;

    let res = create_cover(src_filename, &dst_filename, max_size, dry_run);
    assert!(res.is_ok());
    let return_vec = res.unwrap();
    println!("Image size: {}", return_vec.len());
    assert!(!return_vec.is_empty());
    assert!(!return_vec.is_empty());
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

#[test]
/// Tests that the `needs_resizing` function works as expected.
fn test_needs_resizing() {
    let fname = "../testdata/DSOTM_Cover.jpeg";

    let res = cover_needs_resizing(fname, 500);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), true);

    let res = cover_needs_resizing(fname, 1100);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), false);
}
