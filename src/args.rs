// Process the CLI arguments and find out which flags to set
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::default_values::DefaultValues;
use clap::ArgMatches;

/// Collect the various options submitted into a HashMap for later use.
/// Also checks the default values loaded from a config file.
pub fn parse_options(
    defaults: &DefaultValues,
    args: &ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut new_tags = HashMap::new();

    // TODO: Refactor to check for -c and use, and then for parameter and overwrite.

    if args.is_present("album-artist") {
        new_tags.insert(
            "ALBUMARTIST".to_string(),
            args.value_of("album-artist").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_artist {
            new_tags.insert("ALBUMARTIST".to_string(), val.to_string());
        }
    }

    if args.is_present("album-artist-sort") {
        new_tags.insert(
            "ALBUMARTISTSORT".to_string(),
            args.value_of("album-artist-sort").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_artist_sort {
            new_tags.insert("ALBUMARTISTSORT".to_string(), val.to_string());
        }
    }

    if args.is_present("album-title") {
        new_tags.insert(
            "ALBUM".to_string(),
            args.value_of("album-title").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_title {
            new_tags.insert("ALBUM".to_string(), val.to_string());
        }
    }

    if args.is_present("album-title-sort") {
        new_tags.insert(
            "ALBUMTITLESORT".to_string(),
            args.value_of("album-title-sort").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.album_title {
            new_tags.insert("ALBUMTITLESORT".to_string(), val.to_string());
        }
    }

    if args.is_present("disc-number") {
        new_tags.insert(
            "DISCNUMBER".to_string(),
            args.value_of("disc-number").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.disc_number {
            new_tags.insert("DISCNUMBER".to_string(), val.to_string());
        }
    }

    if args.is_present("disc-total") {
        new_tags.insert(
            "DISCTOTAL".to_string(),
            args.value_of("disc-total").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.disc_total {
            new_tags.insert("DISCTOTAL".to_string(), val.to_string());
        }
    }

    // TRACK //

    if args.is_present("track-artist") {
        new_tags.insert(
            "ARTIST".to_string(),
            args.value_of("track-artist").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_artist {
            new_tags.insert("ARTIST".to_string(), val.to_string());
        }
    }

    if args.is_present("track-artist-sort") {
        new_tags.insert(
            "ARTISTSORT".to_string(),
            args.value_of("track-artist-sort").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_artist_sort {
            new_tags.insert("ARTISTSORT".to_string(), val.to_string());
        }
    }

    if args.is_present("track-title") {
        new_tags.insert(
            "TITLE".to_string(),
            args.value_of("track-title").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_title {
            new_tags.insert("TITLE".to_string(), val.to_string());
        }
    }

    if args.is_present("track-title-sort") {
        new_tags.insert(
            "TITLESORT".to_string(),
            args.value_of("track-title-sort").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_title_sort {
            new_tags.insert("TITLESORT".to_string(), val.to_string());
        }
    }

    if args.is_present("track-number") {
        new_tags.insert(
            "TRACKNUMBER".to_string(),
            args.value_of("track-number").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_number {
            new_tags.insert("TRACKNUMBER".to_string(), val.to_string());
        }
    }

    if args.is_present("track-total") {
        new_tags.insert(
            "TRACKTOTAL".to_string(),
            args.value_of("track-total").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_total {
            new_tags.insert("TRACKTOTAL".to_string(), val.to_string());
        }
    }

    if args.is_present("track-genre") {
        new_tags.insert(
            "GENRE".to_string(),
            args.value_of("track-genre").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_genre {
            new_tags.insert("GENRE".to_string(), val.to_string());
        }
    }

    // Will update and override previous entry if one is found
    if args.is_present("track-genre-number") {
        // Turn the numeric tag into a string
        new_tags.insert(
            "GENRE".to_string(),
            convert_id3_tag(u16::from_str_radix(
                &args.value_of("track-genre-number").unwrap().to_string(),
                16,
            )?),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_genre_number {
            new_tags.insert("GENRE".to_string(), val.to_string());
        }
    }

    if args.is_present("track-composer") {
        new_tags.insert(
            "COMPOSER".to_string(),
            args.value_of("track-composer").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_composer {
            new_tags.insert("COMPOSER".to_string(), val.to_string());
        }
    }

    if args.is_present("track-composer-sort") {
        new_tags.insert(
            "COMPOSERSORT".to_string(),
            args.value_of("track-composer-sort").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_composer_sort {
            new_tags.insert("COMPOSERSORT".to_string(), val.to_string());
        }
    }

    if args.is_present("track-date") {
        new_tags.insert(
            "DATE".to_string(),
            args.value_of("track-date").unwrap().to_string(),
        );
    } else if args.is_present("config") {
        if let Some(val) = &defaults.track_date {
            new_tags.insert("DATE".to_string(), val.to_string());
        }
    }

    // PICTURE FILES //
    // Check if picture files exist
    // Check parameter first, then fall back to config file (if something is specified there)

    // Front cover
    if args.is_present("picture-front") {
        let picture_front = args.value_of("picture-front").unwrap();
        if !Path::new(&picture_front).exists() {
            if defaults.stop_on_error.unwrap() {
                return Err(format!(
                    "Config file picture_front: file {} not found.",
                    &picture_front
                )
                .into());
            } else {
                log::warn!(
                    "Config file picture_front: file {} not found. Continuing.",
                    &picture_front
                );
            }
        } else {
            new_tags.insert("PICTUREFRONT".to_string(), picture_front.to_string());
        }
    } else if args.is_present("config") {
        if let Some(picture_front) = &defaults.picture_front {
            if !Path::new(&picture_front).exists() {
                if defaults.stop_on_error.unwrap() {
                    return Err(format!(
                        "Config file picture_front: file {} not found.",
                        &picture_front
                    )
                    .into());
                } else {
                    log::warn!(
                        "Config file picture_front: file {} not found. Continuing.",
                        &picture_front
                    );
                }
            } else {
                new_tags.insert("PICTUREFRONT".to_string(), picture_front.to_string());
                log::debug!("Picture insertion is not yet implemented.");
            }
        }
    }

    // Back cover
    if args.is_present("picture-back") {
        let picture_back = args.value_of("picture-back").unwrap();
        if !Path::new(&picture_back).exists() {
            if defaults.stop_on_error.unwrap() {
                return Err(format!(
                    "Config file picture_back: file {} not found.",
                    &picture_back
                )
                .into());
            } else {
                log::warn!(
                    "Config file picture_back: file {} not found. Continuing.",
                    &picture_back
                );
            }
        } else {
            new_tags.insert("PICTUREBACK".to_string(), picture_back.to_string());
        }
    } else if args.is_present("config") {
        if let Some(picture_back) = &defaults.picture_back {
            if !Path::new(&picture_back).exists() {
                if defaults.stop_on_error.unwrap() {
                    return Err(format!(
                        "Config file picture_back: file {} not found.",
                        &picture_back
                    )
                    .into());
                } else {
                    log::warn!(
                        "Config file picture_back: file {} not found. Continuing.",
                        &picture_back
                    );
                }
            } else {
                new_tags.insert("PICTUREBACK".to_string(), picture_back.to_string());
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

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap()
        .to_string()
}

/// Convert a numerical ID3 genre to a string
/// Ref: https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D
pub fn convert_id3_tag(tagnumber: u16) -> String {
    let return_string = match tagnumber {
        000 => "Blues",
        001 => "Classic Rock",
        002 => "Country",
        003 => "Dance",
        004 => "Disco",
        005 => "Funk",
        006 => "Grunge",
        007 => "Hip-Hop",
        008 => "Jazz",
        009 => "Metal",
        010 => "New Age",
        011 => "Oldies",
        012 => "Other",
        013 => "Pop",
        014 => "Rhythm and Blues",
        015 => "Rap",
        016 => "Reggae",
        017 => "Rock",
        018 => "Techno",
        019 => "Industrial",
        020 => "Alternative",
        021 => "Ska",
        022 => "Death Metal",
        023 => "Pranks",
        024 => "Soundtrack",
        025 => "Euro-Techno",
        026 => "Ambient",
        027 => "Trip-Hop",
        028 => "Vocal",
        029 => "Jazz & Funk",
        030 => "Fusion",
        031 => "Trance",
        032 => "Classical",
        033 => "Instrumental",
        034 => "Acid",
        035 => "House",
        036 => "Game",
        037 => "Sound clip",
        038 => "Gospel",
        039 => "Noise",
        040 => "Alternative Rock",
        041 => "Bass",
        042 => "Soul",
        043 => "Punk",
        044 => "Space",
        045 => "Meditative",
        046 => "Instrumental Pop",
        047 => "Instrumental Rock",
        048 => "Ethnic",
        049 => "Gothic",
        050 => "Darkwave",
        051 => "Techno-Industrial",
        052 => "Electronic",
        053 => "Pop-Folk",
        054 => "Eurodance",
        055 => "Dream",
        056 => "Southern Rock",
        057 => "Comedy",
        058 => "Cult",
        059 => "Gangsta",
        060 => "Top 40",
        061 => "Christian Rap",
        062 => "Pop/Funk",
        063 => "Jungle",
        064 => "Native US",
        065 => "Cabaret",
        066 => "New Wave",
        067 => "Psychedelic",
        068 => "Rave",
        069 => "Show Tunes",
        070 => "Trailer",
        071 => "Lo-Fi",
        072 => "Tribal",
        073 => "Acid Punk",
        074 => "Acid Jazz",
        075 => "Polka",
        076 => "Retro",
        077 => "Musical",
        078 => "Rock 'n' Roll",
        079 => "Hard Rock",
        080 => "Folk",
        081 => "Folk-Rock",
        082 => "National Folk",
        083 => "Swing",
        084 => "Fast Fusion",
        085 => "Bebop",
        086 => "Latin",
        087 => "Revival",
        088 => "Celtic",
        089 => "Bluegrass",
        090 => "Avantgarde",
        091 => "Gothic Rock",
        092 => "Progressive Rock",
        093 => "Psychedelic Rock",
        094 => "Symphonic Rock",
        095 => "Slow Rock",
        096 => "Big Band",
        097 => "Chorus",
        098 => "Easy Listening",
        099 => "Acoustic",
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
    return_string.to_string()
}
