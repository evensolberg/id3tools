use std::{collections::HashMap, error::Error};

use crate::default_values::DefaultValues;

/// Renames the file provided based on the pattern provided.
///
/// **Parameters:**
///
/// - `filename: &str` -- the name of the file to be renamed
/// - `tags: &HashMap<String, String>` -- The various tag values (ie. Album Artist, Genre, etc.)
/// - `pattern: &str` -- the tag pattern for the new filename. This has been validated to be OK by the CLI.
/// - `config: &DefaultValues` -- The tags that have been set, and any config settings such as dry-run.
///
/// Note that you'll need to populate the tags struct _before_ using this function. This is to avoid having to re-open the file and re-read the data.
///
/// **Returns**
/// - The new file name if successful
/// - An error message if unsuccessful.
///
/// **Example:**
///
/// ```
/// let res = rename_file("somefile.flac", "%dn-%tn %tt", &config)?;
/// ```
///
/// This will rename the file based on disc number (`%dn`), track number (`%tn`)

pub fn rename_file(
    filename: &str,
    tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<String, Box<dyn Error>> {
    let mut new_filename;

    if let Some(nfn) = &config.rename_file {
        new_filename = nfn.to_string();
    } else {
        return Err("No filename pattern presented. Unable to continue.".into());
    }

    // replace any options (eg. %aa, %tg) with the corresponding tag
    for (key, value) in tags {
        new_filename = new_filename.replace(key, value);
    }

    log::debug!("{} --> {}", filename, new_filename);

    // return safely
    Ok(new_filename.to_string())
}
