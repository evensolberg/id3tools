//! Read the contents of an APE file and show the metadata.

use anyhow::Result;
use ape::{self, ItemType};

/// Show the metadata of an APE file.
/// If `show_detail` is true, show more detailed information such as binary data (items and lengths) and locator data (items and lengths).
///
/// # Arguments
///
/// * `filename` - A string slice that holds the name of the file to read.
/// * `show_detail` - A boolean that indicates whether to show detailed information.
///
/// # Returns
///
/// * `Result<()>` - A result that indicates whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// let filename = "test.ape";
/// let show_detail = true;
/// let res = id3show::show_metadata(filename, show_detail);
/// assert!(res.is_ok());
/// ```
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<()> {
    let tags = ape::read_from_path(filename)?;

    for item in tags.iter() {
        match item.get_type() {
            ItemType::Text | ItemType::Locator => {
                if let Ok(s) = <&str>::try_from(item) {
                    if item.get_type() == ItemType::Locator {
                        if show_detail {
                            println!("  Locator:");
                            println!("    {} = {s}", item.key);
                        }
                    } else {
                        println!("  {} = {s}", item.key);
                    }
                }
            }
            ItemType::Binary => {
                if show_detail {
                    let bytes: Vec<u8> = item.into();
                    println!("  Binary:");
                    println!("    {} = {} bytes", item.key, bytes.len());
                }
            }
        }
    }

    // Return safely
    Ok(())
}
