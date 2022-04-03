use std::{collections::HashMap, error::Error, path::Path};

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
/// This will rename the file based on disc number (`%dn`), track number (`%tn`) and track title (%tt).
pub fn rename_file(
    filename: &str,
    tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<String, Box<dyn Error>> {
    // Check if there is a rename pattern
    let mut new_filename = if let Some(nfn) = &config.rename_file {
        nfn.to_string()
    } else {
        return Err("No filename pattern presented. Unable to continue.".into());
    };

    // These tags (may) need to be padded with leading zeros.
    let pad_tags = vec![
        "%dn",
        "%dt",
        "%tn",
        "%to",
        "%disc-number",
        "%disc-number-total",
        "%track-number",
        "%track-number-total",
    ];

    // replace any options (eg. %aa, %tg) with the corresponding tag
    for (key, value) in tags {
        // Make sure to pad disc and track numbers with leading zeros
        let mut fixed_value = value.clone();
        fixed_value = fixed_value.trim().to_string();

        for tag in &pad_tags {
            if key == tag {
                fixed_value = format!("{:0>2}", value.trim());
            }
        }

        // Do the actual filename replacement
        new_filename = new_filename.replace(key, &fixed_value);

        // Fix a few things we know will give us trouble later.
        new_filename = new_filename.replace('/', "-");
        new_filename = new_filename.replace(':', " -");
        new_filename = new_filename.replace('.', "");

        // Remove leading or trailing spaces
        new_filename = new_filename.trim().to_string();
    }

    // Get the path in front of the filename (eg. "music/01.flac" returns "music/")
    let parent = Path::new(&filename)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Create the new filename
    let mut new_path =
        parent.join(Path::new(&new_filename).with_extension(common::get_extension(filename)));
    log::debug!("new_path = {:?}", new_path);

    // Return if the new filename is the same as the old
    let np = new_path.to_string_lossy().to_string();
    if np == *filename {
        log::debug!("New filename == old filename. Returning.");
        return Ok(np);
    }

    // Check if a file with the new filename already exists - make the filename unique if it does.
    if Path::new(&new_path).exists() {
        // Set a unique value based on the current time.
        let unique_val = common::get_unique_value();

        log::warn!(
            "{} already exists. Appending unique identifier.",
            new_filename
        );
        new_filename = format!("{} ({:0>4})", new_filename, unique_val);
        new_path =
            parent.join(Path::new(&new_filename).with_extension(common::get_extension(filename)));
    }

    // Perform the actual rename and check the outcome
    if config.dry_run.unwrap_or(true) {
        log::debug!("dr: {} --> {}", filename, new_path.display());
    } else {
        // Get parent dir
        let rn_res = std::fs::rename(&filename, &new_path);
        match rn_res {
            Ok(_) => log::debug!("{} --> {}", filename, new_path.to_string_lossy()),
            Err(err) => {
                if config.stop_on_error.unwrap_or(true) {
                    return Err(format!(
                        "Unable to rename {} to {}. Error message: {}",
                        filename,
                        new_path.to_string_lossy(),
                        err
                    )
                    .into());
                }
                log::warn!(
                    "Unable to rename {} to {}. Error message: {}",
                    filename,
                    new_path.to_string_lossy(),
                    err
                );
            }
        }
    }

    // return safely
    let result = new_path.to_string_lossy().into_owned();
    Ok(result)
}
