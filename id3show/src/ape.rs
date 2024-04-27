//! Read the contents of an APE file and show the metadata.

use ape::{self, ItemValue};
use std::error::Error;

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
/// * `Result<(), Box<dyn Error>>` - A result that indicates whether the operation was successful.
///
/// # Example
///
/// ```ignore
/// let filename = "test.ape";
/// let show_detail = true;
/// let res = id3show::show_metadata(filename, show_detail);
/// assert!(res.is_ok());
/// ```
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    let tags = ape::read_from_path(filename)?;

    for item in tags.iter() {
        match &item.value {
            // "Regular" metadata
            ItemValue::Text(ref s) => {
                println!("  {} = {s}", item.key);
            }
            // Pictures and such
            ItemValue::Binary(ref b) => {
                if show_detail {
                    println!("  Binary:");
                    println!("    {} = {} bytes", item.key, b.len());
                }
            }
            // Locator is an UTF-8 string contains a link to external information.
            ItemValue::Locator(l) => {
                if show_detail {
                    println!("  Locator:");
                    println!("    {} = {l}", item.key);
                }
            }
        }
    }

    // Return safely
    Ok(())
}
