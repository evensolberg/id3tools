//! Contains the functionality to process DSF files. Currently only rename is supported.

use crate::default_values::DefaultValues;
use crate::rename_file;
use crate::{formats::tags::*, shared::FileTypes};
use dsf::{self, DsfFile};
use id3::TagLike;
use std::{collections::HashMap, error::Error, path::Path};

/// Performs the actual processing of DSF files.
pub fn process_dsf(
    filename: &str,
    new_tags: &HashMap<String, String>,
    config: &DefaultValues,
) -> Result<bool, Box<dyn Error>> {
    log::debug!("Filename: {}", &filename);

    let mut processed_ok = false;

    if let Some(mut tag) = DsfFile::open(Path::new(&filename))?.id3_tag().to_owned() {
        log::debug!("Tag: {:?}", tag);
        for frame in tag.frames() {
            log::debug!("{} = {}", frame.id(), frame.content());
        }

        // Print new tags
        for (key, value) in new_tags {
            // Output information about tags getting changed
            if !(config.detail_off.unwrap_or(false)) {
                // If this is a dry run, output each change
                if config.dry_run.unwrap_or(false) {
                    log::info!("{} :: New {} = {}", &filename, key, value);
                } else {
                    log::debug!("{} :: New {} = {}", &filename, key, value);
                }
            } else {
                // If not dry run and detail
                log::debug!("{} :: New {} = {}", &filename, key, value);
            }

            // Process the tags into the file. Arguaby we could skip this if it's a
            // dry run, but it's good to do it anyway to ensure that it works.
            match key.as_ref() {
                // Pictures or comment
                "APIC-P" | "APIC-B" | "COMM" => (),

                // Disc number
                "TPOS" => {
                    let num;
                    match value.parse::<u32>() {
                        Ok(n) => num = n,
                        Err(err) => {
                            if config.stop_on_error.unwrap_or(false) {
                                return Err(format!(
                                    "Unable to set disc number to {}. Error: {}",
                                    value, err
                                )
                                .into());
                            } else {
                                log::error!(
                                    "Unable to set disc number to {}. Setting to 1 and continuing. Error: {}",
                                    value,
                                    err
                                );
                                num = 1
                            }
                        }
                    }
                    tag.set_disc(num);
                }

                // Disc count
                "TPOS-T" => {
                    let num;
                    match value.parse::<u32>() {
                        Ok(n) => num = n,
                        Err(err) => {
                            if config.stop_on_error.unwrap_or(false) {
                                return Err(format!(
                                    "Unable to set total discs to {}. Error: {}",
                                    value, err
                                )
                                .into());
                            } else {
                                log::error!(
                                    "Unable to set total discs to {}. Setting to 1 and continuing. Error: {}",
                                    value,
                                    err
                                );
                                num = 1
                            }
                        }
                    }
                    tag.set_total_discs(num);
                }

                // Track number
                "TRCK" => {
                    let num;
                    match value.parse::<u32>() {
                        Ok(n) => num = n,
                        Err(err) => {
                            if config.stop_on_error.unwrap_or(false) {
                                return Err(format!(
                                    "Unable to set track number to {}. Error: {}",
                                    value, err
                                )
                                .into());
                            } else {
                                log::error!(
                                    "Unable to set track number to {}. Setting to 1 and continuing. Error: {}",
                                    value,
                                    err
                                );
                                num = 1
                            }
                        }
                    }
                    tag.set_track(num);
                }

                // Track count
                "TRCK-T" => {
                    let num;
                    match value.parse::<u32>() {
                        Ok(n) => num = n,
                        Err(err) => {
                            if config.stop_on_error.unwrap_or(false) {
                                return Err(format!(
                                    "Unable to set total tracks to {}. Error: {}",
                                    value, err
                                )
                                .into());
                            } else {
                                log::error!(
                                    "Unable to set total tracks to {}. Setting to 1 and continuing. Error: {}",
                                    value,
                                    err
                                );
                                num = 1
                            }
                        }
                    }
                    tag.set_total_tracks(num)
                }

                // Everything else
                _ => tag.set_text(key, value.trim()),
            }
            processed_ok = true;
        }

        // Write tags to file
        log::debug!(
            "Writing to DSF files is currently not supported. Not writing {}",
            filename
        );

        // Rename file
        if config.rename_file.is_some() {
            if rename_dsf(filename, config, tag).is_ok() {
                processed_ok = true;
            } else {
                processed_ok = false;
            }
        }
    }

    Ok(processed_ok)
}

/// Renames an MP3 file based on the pattern provided
fn rename_dsf(filename: &str, config: &DefaultValues, tag: id3::Tag) -> Result<(), Box<dyn Error>> {
    let tags_names = option_to_tag(FileTypes::Dsf);
    let mut replace_map = HashMap::new();

    let mut pattern = "".to_string();
    if let Some(p) = &config.rename_file {
        pattern = p.clone();
    }

    // get the mappings of %aa --> ALBUMARTIST --> Madonna
    // key = %aa, vorbis_key = ALBUMARTIST, vval = Madonna
    for (key, tag_name) in tags_names {
        // Get the ID3 value based on the tag_name from the HashMap
        if let Some(vval) = tag.get(&tag_name).and_then(|frame| frame.content().text()) {
            if tag_name == "TPOS" || tag_name == "TRCK" {
                let separates: Vec<&str> = vval.split('/').collect();
                let mut count = "01".to_string();
                let mut total = "01".to_string();
                if !separates.is_empty() {
                    count = format!("{:0>2}", separates[0]);
                }
                if separates.len() > 1 {
                    total = format!("{:0>2}", separates[1]);
                }
                log::debug!("{} count = {}, total = {}", tag_name, count, total);
                match tag_name.as_str() {
                    "TPOS" => {
                        replace_map.insert("%dn".to_string(), count.clone());
                        replace_map.insert("%disc-number".to_string(), count);
                        replace_map.insert("%dt".to_string(), total.clone());
                        replace_map.insert("%dnt".to_string(), total.clone());
                        replace_map.insert("%disc-number-total".to_string(), total);
                    }
                    "TRCK" => {
                        replace_map.insert("%tn".to_string(), count.clone());
                        replace_map.insert("%track-number".to_string(), count);
                        replace_map.insert("%to".to_string(), total.clone());
                        replace_map.insert("%tnt".to_string(), total.clone());
                        replace_map.insert("%track-number-total".to_string(), total);
                    }
                    _ => {
                        return Err(format!(
                            "Unknown tag {} encountered when unwrapping disc/track information.",
                            tag_name
                        )
                        .into())
                    }
                }
            } else {
                let value = vval.to_string();
                log::debug!("key = {}, tag_name = {}, value = {}", key, tag_name, value);
                replace_map.insert(key, value);
            }
        }
    }

    log::debug!("replace_map = {:?}", replace_map);

    let rename_result = rename_file::rename_file(filename, &replace_map, config);
    match rename_result {
        Ok(new_filename) => log::info!("{} --> {}", filename, new_filename),
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!(
                    "Unable to rename {} with tags \"{}\". Error: {}",
                    filename, pattern, err
                )
                .into());
            } else {
                log::warn!(
                    "Unable to rename {} with tags \"{}\". Error: {} Continuing.",
                    filename,
                    pattern,
                    err
                );
            }
        }
    }

    // Return safely
    Ok(())
}
