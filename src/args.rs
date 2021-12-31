// Process the CLI arguments and find out which flags to set
use std::collections::HashMap;
use std::error::Error;

use crate::default_values::DefaultValues;
use crate::shared;
use clap::ArgMatches;

/// The types of files we can process
#[derive(Debug, Copy, Clone)]
pub enum FileType {
    Flac,
    MP3,
    MP4,
}

/// Used to store the various tag names based on the file type
#[derive(Debug, Default, Clone)]
struct TagNames {
    album_artist: String,
    album_artist_sort: String,
    album_title: String,
    album_title_sort: String,
    disc_number: String,
    disc_total: String,
    track_artist: String,
    track_artist_sort: String,
    track_title: String,
    track_title_sort: String,
    track_number: String,
    track_number_total: String,
    track_genre: String,
    track_composer: String,
    track_composer_sort: String,
    track_date: String,
    track_comments: String,
    picture_front: String,
    picture_back: String,
}

/// Collect the various options submitted into a HashMap for later use.
/// Also checks the default values loaded from a config file.
pub fn parse_options(
    filename: &str,
    file_type: FileType,
    defaults: &DefaultValues,
    args: &ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_tags = HashMap::new();

    // Set tag names based on file type -- see tag_names function below
    let tag_names = get_tag_names(file_type);

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
            get_genre_name(u16::from_str_radix(
                &args
                    .value_of("track-genre-number")
                    .unwrap_or("")
                    .to_string(),
                16,
            )?)?,
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_genre_number {
            new_tags.insert(tag_names.track_genre.clone(), get_genre_name(*val)?);
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
        if let Some(picture) = shared::find_picture(&filename, pf_arg, defaults)? {
            new_tags.insert(tag_names.picture_front, picture.to_string());
        } else if defaults.stop_on_error.unwrap_or(false) {
            return Err(format!("Argument picture-front file {} not found.", &pf_arg).into());
        } else {
            log::warn!(
                "Argument picture_front: file {} not found. Continuing.",
                &pf_arg
            );
        }
    } else if args.is_present("config") {
        if let Some(pf_arg) = &defaults.picture_front {
            if let Some(picture) = shared::find_picture(&filename, pf_arg, defaults)? {
                new_tags.insert(tag_names.picture_front, picture.to_string());
            } else if defaults.stop_on_error.unwrap_or(false) {
                return Err(
                    format!("Config file picture_front: file {} not found.", &pf_arg).into(),
                );
            } else {
                log::warn!(
                    "Config file picture_front: file {} not found. Continuing.",
                    &pf_arg
                );
            }
        } // if let Some(picture_front)
    }

    // Back cover
    if args.is_present("picture-back") {
        let pf_arg = args.value_of("picture-back").unwrap_or("");
        if let Some(picture) = shared::find_picture(&filename, pf_arg, defaults)? {
            new_tags.insert(tag_names.picture_back, picture.to_string());
        } else if defaults.stop_on_error.unwrap_or(false) {
            return Err(format!("Config file picture_back: file {} not found.", &pf_arg).into());
        } else {
            log::warn!(
                "Config file picture_back: file {} not found. Continuing.",
                &pf_arg
            );
        }
    } else if args.is_present("config") {
        if let Some(pf_arg) = &defaults.picture_back {
            if let Some(picture) = shared::find_picture(&filename, pf_arg, defaults)? {
                new_tags.insert(tag_names.picture_back, picture.to_string());
            } else if defaults.stop_on_error.unwrap_or(false) {
                return Err(
                    format!("Config file picture_back: file {} not found.", &pf_arg).into(),
                );
            } else {
                log::warn!(
                    "Config file picture_back: file {} not found. Continuing.",
                    &pf_arg
                );
            }
        }
    }

    // Return safely
    Ok(new_tags)
}

// Housekeeping functions to check which flags have been set, either here or in the config file.

/// Check if the stop-on-error flag has been set, either in the config file
/// or via the CLI.
pub fn stop_on_error(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.stop_on_error {
            return_value = cfg;
        }
    }

    if args.is_present("stop-on-error") {
        return_value = true;
    }

    if return_value {
        log::debug!("Stop on error flag set. Will stop if errors occur.");
    } else {
        log::debug!("Stop on error flag not set. Will attempt to continue in case of errors.");
    }

    // return the value
    return_value
}

/// Check if the print-summary flag has been set, either in the config file
/// or via the CLI.
pub fn print_summary(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.print_summary {
            return_value = cfg;
        }
    }

    if args.is_present("print-summary") {
        return_value = true;
    }

    if return_value {
        log::debug!("Print summary flag set. Will output summary when all processing is done.");
    } else {
        log::debug!("Print summary not set. Will not output summary when all processing is done.");
    }

    // return the value
    return_value
}

/// Check if the quiet flag has been set, either in the config file
/// or via the CLI.
pub fn quiet(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.quiet {
            return_value = cfg;
        }
    }

    if args.is_present("quiet") {
        return_value = true;
    }

    if return_value {
        log::debug!("Quiet flag set. Will suppress output except warnings or errors.");
    } else {
        log::debug!("Quiet flag not set. Will output details as files are processed.");
    }

    // return the value
    return_value
}

/// Check if the detail-off flag has been set, either in the config file
/// or via the CLI.
pub fn detail_off(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.detail_off {
            return_value = cfg;
        }
    }

    if args.is_present("detail-off") {
        return_value = true;
    }

    if return_value {
        log::debug!("Detail off flag set. Will suppress output except warnings or errors.");
    } else {
        log::debug!("Detail off flag not set. Will output details as files are processed.");
    }

    // return the value
    return_value
}

/// Check if the detail-off flag has been set, either in the config file
/// or via the CLI.
pub fn dry_run(defaults: &DefaultValues, args: &clap::ArgMatches) -> bool {
    let mut return_value = false;
    if args.is_present("config") {
        if let Some(cfg) = defaults.dry_run {
            return_value = cfg;
        }
    }

    if args.is_present("dry-run") {
        return_value = true;
    }

    if return_value {
        log::debug!(
            "Dry run flag set. Will not perform any actual processing, only report output."
        );
    } else {
        log::debug!("Dry run flag not set. Will process files.");
    }

    // return the value
    return_value
}

/// Convert a numerical ID3 genre to a string
/// Ref: <https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D>
pub fn get_genre_name(tagnumber: u16) -> Result<String, Box<dyn Error>> {
    // TODO: Make this more robust by erroring out if the wrong number is given.
    if tagnumber > 191 {
        return Err("Incorrent value supplied. Must be 0-191.".into());
    }

    let return_string = match tagnumber {
        0 => "Blues",
        1 => "Classic Rock",
        2 => "Country",
        3 => "Dance",
        4 => "Disco",
        5 => "Funk",
        6 => "Grunge",
        7 => "Hip-Hop",
        8 => "Jazz",
        9 => "Metal",
        10 => "New Age",
        11 => "Oldies",
        12 => "Other",
        13 => "Pop",
        14 => "Rhythm and Blues",
        15 => "Rap",
        16 => "Reggae",
        17 => "Rock",
        18 => "Techno",
        19 => "Industrial",
        20 => "Alternative",
        21 => "Ska",
        22 => "Death Metal",
        23 => "Pranks",
        24 => "Soundtrack",
        25 => "Euro-Techno",
        26 => "Ambient",
        27 => "Trip-Hop",
        28 => "Vocal",
        29 => "Jazz & Funk",
        30 => "Fusion",
        31 => "Trance",
        32 => "Classical",
        33 => "Instrumental",
        34 => "Acid",
        35 => "House",
        36 => "Game",
        37 => "Sound clip",
        38 => "Gospel",
        39 => "Noise",
        40 => "Alternative Rock",
        41 => "Bass",
        42 => "Soul",
        43 => "Punk",
        44 => "Space",
        45 => "Meditative",
        46 => "Instrumental Pop",
        47 => "Instrumental Rock",
        48 => "Ethnic",
        49 => "Gothic",
        50 => "Darkwave",
        51 => "Techno-Industrial",
        52 => "Electronic",
        53 => "Pop-Folk",
        54 => "Eurodance",
        55 => "Dream",
        56 => "Southern Rock",
        57 => "Comedy",
        58 => "Cult",
        59 => "Gangsta",
        60 => "Top 40",
        61 => "Christian Rap",
        62 => "Pop/Funk",
        63 => "Jungle",
        64 => "Native US",
        65 => "Cabaret",
        66 => "New Wave",
        67 => "Psychedelic",
        68 => "Rave",
        69 => "Show Tunes",
        70 => "Trailer",
        71 => "Lo-Fi",
        72 => "Tribal",
        73 => "Acid Punk",
        74 => "Acid Jazz",
        75 => "Polka",
        76 => "Retro",
        77 => "Musical",
        78 => "Rock 'n' Roll",
        79 => "Hard Rock",
        80 => "Folk",
        81 => "Folk-Rock",
        82 => "National Folk",
        83 => "Swing",
        84 => "Fast Fusion",
        85 => "Bebop",
        86 => "Latin",
        87 => "Revival",
        88 => "Celtic",
        89 => "Bluegrass",
        90 => "Avantgarde",
        91 => "Gothic Rock",
        92 => "Progressive Rock",
        93 => "Psychedelic Rock",
        94 => "Symphonic Rock",
        95 => "Slow Rock",
        96 => "Big Band",
        97 => "Chorus",
        98 => "Easy Listening",
        99 => "Acoustic",
        100 => "Humour",
        101 => "Speech",
        102 => "Chanson",
        103 => "Opera",
        104 => "Chamber Music",
        105 => "Sonata",
        106 => "Symphony",
        107 => "Booty Bass",
        108 => "Primus",
        109 => "Porn Groove",
        110 => "Satire",
        111 => "Slow Jam",
        112 => "Club",
        113 => "Tango",
        114 => "Samba",
        115 => "Folklore",
        116 => "Ballad",
        117 => "Power Ballad",
        118 => "Rhythmic Soul",
        119 => "Freestyle",
        120 => "Duet",
        121 => "Punk Rock",
        122 => "Drum Solo",
        123 => "A Cappella",
        124 => "Euro-House",
        125 => "Dancehall",
        126 => "Goa",
        127 => "Drum & Bass",
        128 => "Club-House",
        129 => "Hardcore Techno",
        130 => "Terror",
        131 => "Indie",
        132 => "BritPop",
        133 => "Negerpunk",
        134 => "Polsk Punk",
        135 => "Beat",
        136 => "Christian Gangsta Rap",
        137 => "Heavy Metal",
        138 => "Black Metal",
        139 => "Crossover",
        140 => "Contemporary Christian",
        141 => "Christian Rock",
        142 => "Merengue",
        143 => "Salsa",
        144 => "Thrash Metal",
        145 => "Anime",
        146 => "Jpop",
        147 => "Synthpop",
        148 => "Abstract",
        149 => "Art Rock",
        150 => "Baroque",
        151 => "Bhangra",
        152 => "Big Beat",
        153 => "Breakbeat",
        154 => "Chillout",
        155 => "Downtempo",
        156 => "Dub",
        157 => "EBM",
        158 => "Eclectic",
        159 => "Electro",
        160 => "Electroclash",
        161 => "Emo",
        162 => "Experimental",
        163 => "Garage",
        164 => "Global",
        165 => "IDM",
        166 => "Illbient",
        167 => "Industro-Goth",
        168 => "Jam Band",
        169 => "Krautrock",
        170 => "Leftfield",
        171 => "Lounge",
        172 => "Math Rock",
        173 => "New Romantic",
        174 => "Nu-Breakz",
        175 => "Post-Punk",
        176 => "Post-Rock",
        177 => "Psytrance",
        178 => "Shoegaze",
        179 => "Space Rock",
        180 => "Trop Rock",
        181 => "World Music",
        182 => "Neoclassical",
        183 => "Audiobook",
        184 => "Audio Theatre",
        185 => "Neue Deutche Welle",
        186 => "Podcast",
        187 => "Indie-Rock",
        188 => "G-Funk",
        189 => "Dubstep",
        190 => "Garage Rock",
        191 => "Psybient",
        // If all else fails:
        _ => "Unknown",
    };

    // return the value
    Ok(return_string.to_string())
}

/// Gets the tag names based on the file type
fn get_tag_names(file_type: FileType) -> TagNames {
    match file_type {
        FileType::Flac => TagNames {
            album_artist: "ALBUMARTIST".to_string(),
            album_artist_sort: "ALBUMARTISTSORT".to_string(),
            album_title: "ALBUM".to_string(),
            album_title_sort: "ALBUMTITLESORT".to_string(),
            disc_number: "DISCNUMBER".to_string(),
            disc_total: "DISCTOTAL".to_string(),
            track_artist: "ARTIST".to_string(),
            track_artist_sort: "ARTISTSORT".to_string(),
            track_title: "TITLE".to_string(),
            track_title_sort: "TITLESORT".to_string(),
            track_number: "TRACKNUMBER".to_string(),
            track_number_total: "TRACKTOTAL".to_string(),
            track_genre: "GENRE".to_string(),
            track_composer: "COMPOSER".to_string(),
            track_composer_sort: "COMPOSERSORT".to_string(),
            track_date: "DATE".to_string(),
            track_comments: "DESCRIPTION".to_string(),
            picture_front: "PICTUREFRONT".to_string(),
            picture_back: "PICTUREBACK".to_string(),
        },
        FileType::MP3 => TagNames {
            album_artist: "TPE2".to_string(),
            album_artist_sort: "TSO2".to_string(),
            album_title: "TALB".to_string(),
            album_title_sort: "TSOA".to_string(),
            disc_number: "TPOS".to_string(),
            disc_total: "TPOS-T".to_string(),
            track_artist: "TPE1".to_string(),
            track_artist_sort: "TSOP".to_string(),
            track_title: "TIT2".to_string(),
            track_title_sort: "TSOT".to_string(),
            track_number: "TRCK".to_string(),
            track_number_total: "TRCK-T".to_string(),
            track_genre: "TCON".to_string(),
            track_composer: "TCOM".to_string(),
            track_composer_sort: "TSOC".to_string(),
            track_date: "TDRC".to_string(),
            track_comments: "COMM".to_string(),
            picture_front: "APIC-F".to_string(),
            picture_back: "APIC-B".to_string(),
        },
        FileType::MP4 => TagNames {
            album_artist: "aART".to_string(),
            album_artist_sort: "soaa".to_string(),
            album_title: "©alb".to_string(),
            album_title_sort: "soal".to_string(),
            disc_number: "disk".to_string(),
            disc_total: "disk-t".to_string(),
            track_artist: "©ART".to_string(),
            track_artist_sort: "soar".to_string(),
            track_title: "©nam".to_string(),
            track_title_sort: "sonm".to_string(),
            track_number: "trkn".to_string(),
            track_number_total: "trkn-t".to_string(),
            track_genre: "©gen".to_string(),
            track_composer: "©wrt".to_string(),
            track_composer_sort: "soco".to_string(),
            track_date: "©day".to_string(),
            track_comments: "©cmt".to_string(),
            picture_front: "covr-f".to_string(),
            picture_back: "covr-b".to_string(),
        },
    }
}
