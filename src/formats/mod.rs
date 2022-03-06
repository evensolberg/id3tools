//! Various file format parsers. The different types of file formats (ie. APE, FLAC, MP3, MP4)
//! all reside under this crate, so they don't have to be exposed to the main body of code.

use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Component, Path},
};

use crate::{default_values::DefaultValues, shared};
mod ape;
mod flac;
mod mp3;
mod mp4;
mod tags;

/// The types of files we can process
#[derive(Debug, Copy, Clone)]
pub enum FileTypes {
    Ape,
    Flac,
    MP3,
    MP4,
    Unknown,
}

/// Performs the actual file processing
///
/// Parameters:
///
/// - `file_type: args::FileType` -- the type of file to process (`Flac`, `MP3` or `MP4`)
/// - `filename: &str` -- the name of the file
/// - `config: &DefaultValues` -- The default config values to use (stop on error, etc)
/// - `cli_args: &clap::ArgMatches` -- The config values and options supplied from the CLI
/// - `counts: &mut shared::Counts` -- A struct for various file counters (skipped, processed, total)
///
/// Returns:
///
/// - `Ok()` if everything goes well.
/// - `Box<dyn Error>` if we run into problems
pub fn process_file(
    file_type: FileTypes,
    filename: &str,
    config: &DefaultValues,
    cli_args: &clap::ArgMatches,
) -> Result<bool, Box<dyn Error>> {
    match file_type {
        FileTypes::Ape => log::debug!("Processing APE."),
        FileTypes::Flac => log::debug!("Processing FLAC."),
        FileTypes::MP3 => log::debug!("Processing MP3."),
        FileTypes::MP4 => log::debug!("Processing MP4."),
        FileTypes::Unknown => return Err(format!("Unknown file type: {}", filename).into()),
    }

    let new_tags_result = parse_options(filename, file_type, config, cli_args);
    log::debug!("new_tags_result: {:?}", new_tags_result);
    let new_tags;
    let mut processed = false;
    match new_tags_result {
        Ok(res) => {
            new_tags = res;
            log::debug!("New tags: {:?}", new_tags);

            log::debug!("Processing file {}", filename);
            let proc_res = match file_type {
                FileTypes::Ape => ape::process_ape(filename, &new_tags, config),
                FileTypes::Flac => flac::process_flac(filename, &new_tags, config),
                FileTypes::MP3 => mp3::process_mp3(filename, &new_tags, config),
                FileTypes::MP4 => mp4::process_mp4(filename, &new_tags, config),
                FileTypes::Unknown => {
                    return Err("We should never get here. That's a problem.".into())
                }
            };

            match proc_res {
                Ok(_) => processed = true,
                Err(err) => {
                    if config.stop_on_error.unwrap_or(true) {
                        return Err(
                            format!("Unable to process {}. Error: {}", filename, err).into()
                        );
                    } else {
                        log::error!("Unable to process {}. Error: {}", filename, err);
                    }
                }
            } // match flag::process_flac
        } // Ok(_)
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(
                    format!("Unable to parse tags for {}. Error: {}", filename, err).into(),
                );
            } else {
                log::error!("Unable to parse tags for {}. Error: {}", filename, err);
            }
        } // Err(err)
    } // match new_tags_result

    // return safely
    Ok(processed)
}

/// Collect the various options/tags submitted into a HashMap for later use.
/// Also checks the default values loaded from a config file.
fn parse_options(
    filename: &str,
    file_type: FileTypes,
    defaults: &DefaultValues,
    args: &clap::ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    log::debug!("parse_options Start");
    let mut new_tags = HashMap::new();

    // Set tag names based on file type -- see tag_names function below
    let tag_names = tags::get_tag_names(file_type);

    // TODO: Refactor to check for -c and use, and then for parameter and overwrite.

    if args.is_present("album-artist") {
        new_tags.insert(
            tag_names.album_artist,
            args.value_of("album-artist").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.album_artist {
            new_tags.insert(tag_names.album_artist, val.to_string());
        }
    }

    if args.is_present("album-artist-sort") {
        new_tags.insert(
            tag_names.album_artist_sort,
            args.value_of("album-artist-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.album_artist_sort {
            new_tags.insert(tag_names.album_artist_sort, val.to_string());
        }
    }

    if args.is_present("album-title") {
        new_tags.insert(
            tag_names.album_title,
            args.value_of("album-title").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.album_title {
            new_tags.insert(tag_names.album_title, val.to_string());
        }
    }

    if args.is_present("album-title-sort") {
        new_tags.insert(
            tag_names.album_title_sort,
            args.value_of("album-title-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.album_title {
            new_tags.insert(tag_names.album_title_sort, val.to_string());
        }
    }

    if args.is_present("disc-number") {
        new_tags.insert(
            tag_names.disc_number.clone(),
            args.value_of("disc-number").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.disc_number {
            new_tags.insert(tag_names.disc_number.clone(), val.to_string());
        }
    }

    if args.is_present("disc-total") {
        new_tags.insert(
            tag_names.disc_number_total.clone(),
            args.value_of("disc-total").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.disc_total {
            new_tags.insert(tag_names.disc_number_total.clone(), val.to_string());
        }
    }

    if args.is_present("disc-number-count")
        || (args.is_present("config-file") && defaults.disc_count.unwrap_or(false))
    {
        log::debug!("parse_options: Trying to figure out the disc number automagically.");
        let disc_num = get_disc_number(filename)?;
        log::debug!("parse_options::disc number: {}", disc_num);
        let disc_count = get_disc_count(filename)?;
        log::debug!("parse_options: disc count: {}", disc_count);
        new_tags.insert(tag_names.disc_number.clone(), format!("{:0>2}", disc_num));
        new_tags.insert(
            tag_names.disc_number_total.clone(),
            format!("{:0>2}", disc_count),
        );
    }

    // TRACK //

    if args.is_present("track-artist") {
        new_tags.insert(
            tag_names.track_artist,
            args.value_of("track-artist").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_artist {
            new_tags.insert(tag_names.track_artist, val.to_string());
        }
    }

    if args.is_present("track-artist-sort") {
        new_tags.insert(
            tag_names.track_artist_sort,
            args.value_of("track-artist-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_artist_sort {
            new_tags.insert(tag_names.track_artist_sort, val.to_string());
        }
    }

    if args.is_present("track-title") {
        new_tags.insert(
            tag_names.track_title,
            args.value_of("track-title").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_title {
            new_tags.insert(tag_names.track_title, val.to_string());
        }
    }

    if args.is_present("track-title-sort") {
        new_tags.insert(
            tag_names.track_title_sort,
            args.value_of("track-title-sort").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_title_sort {
            new_tags.insert(tag_names.track_title_sort, val.to_string());
        }
    }

    if args.is_present("track-number") {
        new_tags.insert(
            tag_names.track_number,
            args.value_of("track-number").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_number {
            new_tags.insert(tag_names.track_number, val.to_string());
        }
    }

    if args.is_present("track-total") {
        new_tags.insert(
            tag_names.track_number_total.clone(),
            args.value_of("track-total").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") && !args.is_present("track-count") {
        if let Some(val) = &defaults.track_total {
            new_tags.insert(tag_names.track_number_total.clone(), val.to_string());
        }
    }

    if args.is_present("track-count")
        || (args.is_present("config-file") && defaults.track_count.unwrap_or(false))
    {
        let file_count = shared::count_files(filename)?;
        log::debug!("file_count = {}", file_count);
        new_tags.insert(tag_names.track_number_total, file_count);
    }

    if args.is_present("track-genre") {
        new_tags.insert(
            tag_names.track_genre.clone(),
            args.value_of("track-genre").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_genre {
            new_tags.insert(tag_names.track_genre.clone(), val.to_string());
        }
    }

    // Will update and override previous entry if one is found
    if args.is_present("track-genre-number") {
        // Turn the numeric tag into a string
        new_tags.insert(
            tag_names.track_genre.clone(),
            get_genre_name(
                args.value_of("track-genre-number")
                    .unwrap_or("")
                    .to_string()
                    .parse::<u16>()
                    .unwrap_or_default(),
            )?,
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_genre_number {
            new_tags.insert(tag_names.track_genre.clone(), get_genre_name(*val)?);
        }
    }

    if args.is_present("track-composer") {
        new_tags.insert(
            tag_names.track_composer,
            args.value_of("track-composer").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
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
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_composer_sort {
            new_tags.insert(tag_names.track_composer_sort, val.to_string());
        }
    }

    if args.is_present("track-date") {
        new_tags.insert(
            tag_names.track_date,
            args.value_of("track-date").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_date {
            new_tags.insert(tag_names.track_date, val.to_string());
        }
    }

    if args.is_present("track-comments") {
        new_tags.insert(
            tag_names.track_comments,
            args.value_of("track-comments").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
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
        if let Some(picture) = find_picture(filename, pf_arg, defaults)? {
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
    } else if args.is_present("config-file") {
        if let Some(pf_arg) = &defaults.picture_front {
            if let Some(picture) = find_picture(filename, pf_arg, defaults)? {
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
        if let Some(picture) = find_picture(filename, pf_arg, defaults)? {
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
    } else if args.is_present("config-file") {
        if let Some(pf_arg) = &defaults.picture_back {
            if let Some(picture) = find_picture(filename, pf_arg, defaults)? {
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
    log::debug!("parse_options return -- new_tags = {:?}", &new_tags);
    Ok(new_tags)
}

/// Looks for the picture file with the name supplied. Initially tries to find it in the path of the music file.
/// If unsuccessful, tries to find it in the invocation directory. If still unsuccessful returns either None or
/// an Error, depending on whether the `stop_on_error` flag has been set.
fn find_picture(
    m_filename: &str,
    p_filename: &str,
    config: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    // Assume that the music file exists
    let m_component_name = if let Some(base_path) = Path::new(&m_filename).parent() {
        base_path
    } else {
        Path::new(".")
    };

    log::debug!("music component_name = {:?}", m_component_name);

    if Path::new(m_component_name).join(p_filename).exists() {
        // Picture file exists alongside the music file
        log::debug!(
            "picture file path: {}",
            Path::new(m_component_name)
                .join(p_filename)
                .to_string_lossy()
        );
        return Ok(Some(
            Path::new(m_component_name)
                .join(p_filename)
                .to_str()
                .unwrap()
                .to_string(),
        ));
    } else if Path::new(p_filename).exists() {
        // Picture file exists in the invocation path
        log::debug!("p_filename = {}", p_filename);
        return Ok(Some(Path::new(p_filename).to_str().unwrap().to_string()));
    } else if config.stop_on_error.unwrap_or(false) {
        // No picture found - act accordingly
        return Err(format!("Picture file {} does not exist.", p_filename).into());
    } else {
        Ok(None)
    }
}

/// Convert a numerical ID3 genre to a string
/// Ref: <https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D>
fn get_genre_name(tagnumber: u16) -> Result<String, Box<dyn Error>> {
    if tagnumber > 191 {
        return Err("Incorrect value supplied. Must be 0-191.".into());
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

/// Figures out the disc number based on the directory above it.
/// It it is named 'CD xx' or 'disc xx' (case insensitive), we get the number and use it.
fn get_disc_number(filename: &str) -> Result<u16, Box<dyn Error>> {
    log::trace!("get_disc_number::filename: {}", filename);

    // Get the full path so we can figure out the parent below
    let full_path = fs::canonicalize(&filename)?;
    log::debug!("get_disc_number::full_path = {:?}", full_path);

    // Get the parent directory
    let mut components = Path::new(&full_path).components();
    log::debug!("get_disc_number::components = {:?}", components);
    let mut parent_dir = components
        .nth_back(1)
        .unwrap_or(Component::ParentDir)
        .as_os_str()
        .to_str()
        .unwrap_or("Awkward!")
        .to_ascii_uppercase();

    // log::debug!("components next = {:?}", components.next_back());
    log::debug!("get_disc_number::parent_dir = {:?}", parent_dir);

    let mut dn = 1; // Disc number

    // Check if the parent directory starts "properly" and extract just the number
    if parent_dir.starts_with("CD")
        || parent_dir.starts_with("DISC")
        || parent_dir.starts_with("PART")
    {
        parent_dir = parent_dir.replace("CD", "");
        parent_dir = parent_dir.replace("DISC", "").trim().to_string();
        parent_dir = parent_dir.replace("PART", "").trim().to_string();

        // Check for longer name - eg CD01 - Something
        if parent_dir.contains(' ') || parent_dir.contains('-') {
            let space = parent_dir.find(' ').unwrap_or(256);
            let dash = parent_dir.find('-').unwrap_or(256);
            let delimiter = if space < dash { ' ' } else { '-' };
            log::trace!(
                "get_disc_number::space = {}, dash = {}, delimiter = {}",
                space,
                dash,
                delimiter
            );

            parent_dir = parent_dir
                .split_once(delimiter)
                .unwrap_or_default()
                .0
                .to_string();
        }

        log::debug!(
            "get_disc_number::parent_dir after processing = {:?}",
            parent_dir
        );
        dn = parent_dir.parse().unwrap_or(0);

        // Check for roman numerals
        if dn == 0 {
            dn = shared::roman_to_decimal(&parent_dir);

            // If roman --> decimal didn't work either, just go with 1.
            if dn == 0 {
                dn = 1;
            }
        }
    }

    // return safely
    log::debug!("get_disc_number::dn = Ok({})", dn);
    Ok(dn)
}

/// Counts the number of discs by looking for the number of `disk`, `CD` etc subdirectories
fn get_disc_count(filename: &str) -> Result<u16, Box<dyn Error>> {
    log::debug!("get_disc_count::get_disc_number filename: {}", filename);

    // Get the full path so we can figure out the grandparent below
    let full_path = fs::canonicalize(&filename)?;
    log::debug!("get_disc_count::full_path = {:?}", full_path);

    // Get the grandparent directory so we can look for disc subdirectories underneath.
    let grandparent_dir = full_path
        .ancestors()
        .nth(2)
        .unwrap_or_else(|| Path::new(&"."))
        .as_os_str()
        .to_str()
        .unwrap_or("None");
    log::debug!("get_disc_count::grandparent_dir = {:?}", grandparent_dir);

    // Find the subdirectories of the grandparent
    let dirs = fs::read_dir(&grandparent_dir)?;
    log::debug!("get_disc_count::dirs = {:?}", dirs);

    // Determine the number of disc subdirs
    let mut disc_count = 0;
    for entry in dirs {
        let path = entry?.path();
        log::debug!("get_disc_count::path = {:?}", path);
        if path.is_dir() {
            let component_name = path
                .components()
                .last()
                .unwrap_or(Component::CurDir)
                .as_os_str()
                .to_str()
                .unwrap_or("None")
                .to_ascii_uppercase();

            log::debug!("get_disc_count::component_name = {}", component_name);
            if component_name.starts_with("CD")
                || component_name.starts_with("DISC")
                || component_name.starts_with("PART")
            {
                disc_count += 1;
                log::debug!("get_disc_count::disc_count = {}", disc_count);
            }
        } else {
            log::debug!("get_disc_count::path.is_dir() == false");
        }
    }

    // Obviously, we have at least 1 disc.
    if disc_count == 0 {
        disc_count = 1;
    }

    log::debug!("get_disc_count::disc_count returned = Ok({})", disc_count);
    // return safely
    Ok(disc_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    /// Tests that the genre number gets returned correctly.
    fn test_get_genre_name() {
        assert_eq!(get_genre_name(0).unwrap(), "Blues".to_string());
        assert_eq!(get_genre_name(9).unwrap(), "Metal".to_string());
        assert_eq!(get_genre_name(32).unwrap(), "Classical".to_string());
        assert!(get_genre_name(200).is_err());
    }

    // #[test]
    // fn test_get_disc_number() {
    //     assert_eq!(
    //         get_disc_number("Test/Part I/01 Veni, creator spiritus.flac").unwrap(),
    //         1
    //     );
    //     assert_eq!(
    //         get_disc_number("Test/Part II/01 Poco adagio.flac").unwrap(),
    //         2
    //     );
    // }

    // #[test]
    // fn test_get_disc_count() {
    //     assert_eq!(
    //         get_disc_count("Test/Part I/01 Veni, creator spiritus.flac").unwrap(),
    //         2
    //     );
    //     assert_eq!(
    //         get_disc_count("Test/Part II/01 Poco adagio.flac").unwrap(),
    //         2
    //     );
    // }
}
