// Process the CLI arguments and find out which flags to set
use std::collections::HashMap;
use std::error::Error;

use crate::default_values::DefaultValues;
use crate::formats;
use crate::shared::{self};
use clap::ArgMatches;

/// Collect the various options submitted into a HashMap for later use.
/// Also checks the default values loaded from a config file.
pub fn parse_options(
    filename: &str,
    file_type: formats::FileTypes,
    defaults: &DefaultValues,
    args: &ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_tags = HashMap::new();

    // Set tag names based on file type -- see tag_names function below
    let tag_names = shared::get_tag_names(file_type);

    // TODO: Refactor to check for -c and use, and then for parameter and overwrite.

    if args.is_present("album-artist") {
        new_tags.insert(
            tag_names.album_artist,
            args.value_of("album-artist").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_artist {
            new_tags.insert(tag_names.album_artist, val.to_string());
        }
    }

    if args.is_present("album-artist-sort") {
        new_tags.insert(
            tag_names.album_artist_sort,
            args.value_of("album-artist-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_artist_sort {
            new_tags.insert(tag_names.album_artist_sort, val.to_string());
        }
    }

    if args.is_present("album-title") {
        new_tags.insert(
            tag_names.album_title,
            args.value_of("album-title").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_title {
            new_tags.insert(tag_names.album_title, val.to_string());
        }
    }

    if args.is_present("album-title-sort") {
        new_tags.insert(
            tag_names.album_title_sort,
            args.value_of("album-title-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_title {
            new_tags.insert(tag_names.album_title_sort, val.to_string());
        }
    }

    if args.is_present("disc-number") {
        new_tags.insert(
            tag_names.disc_number,
            args.value_of("disc-number").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.disc_number {
            new_tags.insert(tag_names.disc_number, val.to_string());
        }
    }

    if args.is_present("disc-total") {
        new_tags.insert(
            tag_names.disc_total,
            args.value_of("disc-total").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.disc_total {
            new_tags.insert(tag_names.disc_total, val.to_string());
        }
    }

    // TRACK //

    if args.is_present("track-artist") {
        new_tags.insert(
            tag_names.track_artist,
            args.value_of("track-artist").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_artist {
            new_tags.insert(tag_names.track_artist, val.to_string());
        }
    }

    if args.is_present("track-artist-sort") {
        new_tags.insert(
            tag_names.track_artist_sort,
            args.value_of("track-artist-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_artist_sort {
            new_tags.insert(tag_names.track_artist_sort, val.to_string());
        }
    }

    if args.is_present("track-title") {
        new_tags.insert(
            tag_names.track_title,
            args.value_of("track-title").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_title {
            new_tags.insert(tag_names.track_title, val.to_string());
        }
    }

    if args.is_present("track-title-sort") {
        new_tags.insert(
            tag_names.track_title_sort,
            args.value_of("track-title-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_title_sort {
            new_tags.insert(tag_names.track_title_sort, val.to_string());
        }
    }

    if args.is_present("track-number") {
        new_tags.insert(
            tag_names.track_number,
            args.value_of("track-number").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_number {
            new_tags.insert(tag_names.track_number, val.to_string());
        }
    }

    if args.is_present("track-total") {
        new_tags.insert(
            tag_names.track_number_total,
            args.value_of("track-total").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_total {
            new_tags.insert(tag_names.track_number_total, val.to_string());
        }
    }

    if args.is_present("track-genre") {
        new_tags.insert(
            tag_names.track_genre.clone(),
            args.value_of("track-genre").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_genre {
            new_tags.insert(tag_names.track_genre.clone(), val.to_string());
        }
    }

    // Will update and override previous entry if one is found
    if args.is_present("track-genre-number") {
        // Turn the numeric tag into a string
        new_tags.insert(
            tag_names.track_genre.clone(),
            shared::get_genre_name(u16::from_str_radix(
                &args
                    .value_of("track-genre-number")
                    .unwrap_or("")
                    .to_string(),
                16,
            )?)?,
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_genre_number {
            new_tags.insert(tag_names.track_genre.clone(), shared::get_genre_name(*val)?);
        }
    }

    if args.is_present("track-composer") {
        new_tags.insert(
            tag_names.track_composer,
            args.value_of("track-composer").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_composer {
            new_tags.insert(tag_names.track_composer, val.to_string());
        }
    }

    if args.is_present("track-composer-sort") {
        new_tags.insert(
            tag_names.track_composer_sort,
            args.value_of("track-composer-sort")
                .unwrap_or("")
                .to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_composer_sort {
            new_tags.insert(tag_names.track_composer_sort, val.to_string());
        }
    }

    if args.is_present("track-date") {
        new_tags.insert(
            tag_names.track_date,
            args.value_of("track-date").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_date {
            new_tags.insert(tag_names.track_date, val.to_string());
        }
    }

    if args.is_present("track-comments") {
        new_tags.insert(
            tag_names.track_comments,
            args.value_of("track-comments").unwrap_or("").to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_comments {
            new_tags.insert(tag_names.track_comments, val.to_string());
        }
    }

    // PICTURE FILES //
    // Check if picture files exist
    // Check parameter first, then fall back to config file (if something is specified there)

    // Front cover
    if args.is_present("picture-front") {
        let pf_arg = args.value_of("picture-front").unwrap_or("");
        if let Some(picture) = shared::find_picture(filename, pf_arg, defaults)? {
            new_tags.insert(tag_names.picture_front, picture);
        } else if defaults.stop_on_error.unwrap_or(false) {
            return Err(format!(
                "{} - Argument picture-front file {} not found.",
                filename, &pf_arg
            )
            .into());
        } else {
            log::warn!(
                "{} - Argument picture_front: file {} not found. Continuing.",
                filename,
                &pf_arg
            );
        }
    } else if args.is_present("config") {
        if let Some(pf_arg) = &defaults.picture_front {
            if let Some(picture) = shared::find_picture(filename, pf_arg, defaults)? {
                new_tags.insert(tag_names.picture_front, picture);
            } else if defaults.stop_on_error.unwrap_or(false) {
                return Err(format!(
                    "{} - Config file picture_front: file {} not found.",
                    filename, &pf_arg
                )
                .into());
            } else {
                log::warn!(
                    "{} - Config file picture_front: file {} not found. Continuing.",
                    filename,
                    &pf_arg
                );
            }
        } // if let Some(picture_front)
    }

    // Back cover
    if args.is_present("picture-back") {
        let pf_arg = args.value_of("picture-back").unwrap_or("");
        if let Some(picture) = shared::find_picture(filename, pf_arg, defaults)? {
            new_tags.insert(tag_names.picture_back, picture);
        } else if defaults.stop_on_error.unwrap_or(false) {
            return Err(format!(
                "{} - Argument picture_back: file {} not found.",
                filename, &pf_arg
            )
            .into());
        } else {
            log::warn!(
                "{} - Argument picture_back: file {} not found. Continuing.",
                filename,
                &pf_arg
            );
        }
    } else if args.is_present("config") {
        if let Some(pf_arg) = &defaults.picture_back {
            if let Some(picture) = shared::find_picture(filename, pf_arg, defaults)? {
                new_tags.insert(tag_names.picture_back, picture);
            } else if defaults.stop_on_error.unwrap_or(false) {
                return Err(format!(
                    "{} - Config file picture_back: file {} not found.",
                    filename, &pf_arg
                )
                .into());
            } else {
                log::warn!(
                    "{} - Config file picture_back: file {} not found. Continuing.",
                    filename,
                    &pf_arg
                );
            }
        }
    }

    // Return safely
    Ok(new_tags)
}
