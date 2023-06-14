//! Read the contents of an APE file and show the metadata.

use ape::{self, ItemValue};
use std::error::Error;

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
