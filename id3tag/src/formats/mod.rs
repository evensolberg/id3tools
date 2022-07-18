//! Various file format parsers. The different types of file formats (ie. APE, FLAC, MP3, MP4)
//! all reside under this crate, so they don't have to be exposed to the main body of code.

#![forbid(unsafe_code)]
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Component, Path},
};

use crate::default_values::DefaultValues;
use common::FileTypes;

mod ape;
mod dsf;
mod flac;
pub mod images;
mod mp3;
mod mp4;
mod tags;

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
/// - `Ok(bool)` if everything goes well. The boolean indicates whether the file was processed or not.
/// - `Box<dyn Error>` if we run into problems
pub fn process_file(
    file_type: FileTypes,
    filename: &str,
    config: &DefaultValues,
    cli_args: &clap::ArgMatches,
) -> Result<bool, Box<dyn Error>> {
    log::debug!("Processing {}", file_type);

    // Check if we need to create one or more cover images.
    let (front_cover_path, back_cover_path) = images::process_images(filename, config)?;
    log::debug!(
        "front_cover_path / bcp: {:?} / {:?}",
        front_cover_path,
        back_cover_path
    );

    let new_tags_result = parse_options(filename, file_type, config, cli_args);

    log::debug!("new_tags_result: {:?}", new_tags_result);
    let mut new_tags;
    let mut processed = false;

    // Process the music files(s)
    match new_tags_result {
        Ok(res) => {
            new_tags = res;
            log::debug!("New tags: {:?}", new_tags);

            log::debug!("Processing file {}", filename);
            let proc_res = match file_type {
                FileTypes::Ape => ape::process(filename, &new_tags, config),
                FileTypes::Dsf => dsf::process(filename, &new_tags, config),
                FileTypes::Flac => flac::process(filename, &mut new_tags, config),
                FileTypes::MP3 => mp3::process(filename, &new_tags, config),
                FileTypes::MP4 => mp4::process(filename, &new_tags, config),
                FileTypes::Unknown => {
                    return Err(format!("{} is unknown file type.", filename).into())
                }
            };

            match proc_res {
                Ok(_) => processed = true,
                Err(err) => {
                    if config.stop_on_error.unwrap_or(true) {
                        return Err(
                            format!("Unable to process {}. Error: {}", filename, err).into()
                        );
                    }
                    log::error!("Unable to process {}. Error: {}", filename, err);
                }
            } // match flag::process_flac
        } // Ok(_)
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(
                    format!("Unable to parse tags for {}. Error: {}", filename, err).into(),
                );
            }
            log::error!("Unable to parse tags for {}. Error: {}", filename, err);
        } // Err(err)
    } // match new_tags_result

    // return safely
    Ok(processed)
}

/// Collect the various options/tags submitted into a `HashMap` for later use.
/// Also checks the default values loaded from a config file.
#[allow(clippy::too_many_lines)]
// TODO: This function is too long. Split it up.
fn parse_options(
    filename: &str,
    file_type: common::FileTypes,
    defaults: &DefaultValues,
    args: &clap::ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    log::debug!("parse_options Start");
    let mut new_tags = HashMap::new();

    // Set tag names based on file type -- see tag_names function below
    let tag_names = tags::get_tag_names(file_type);

    // TODO: Refactor to check for -c and use, and then for parameter and overwrite.

    // ALBUM & TRACK ARTIST //

    if args.is_present("track-album-artist") {
        let taa = args
            .value_of("track-album-artist")
            .unwrap_or("")
            .to_string();
        new_tags.insert(tag_names.track_artist.clone(), taa.clone());
        new_tags.insert(tag_names.album_artist.clone(), taa);
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_album_artist {
            let taa = val.to_string();
            new_tags.insert(tag_names.track_artist.clone(), taa.clone());
            new_tags.insert(tag_names.album_artist.clone(), taa);
        }
    }

    // We should never hit these two ("album-artist" and "track-artist") if we have the one above,
    // but the compiler doesn't know that. So we have to do a bunch of cloning above to ensure the
    // code below still compiles as expected.

    if args.is_present("album-artist") {
        new_tags.insert(
            tag_names.album_artist.clone(),
            args.value_of("album-artist").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.album_artist {
            new_tags.insert(tag_names.album_artist, val.to_string());
        }
    }

    if args.is_present("track-artist") {
        new_tags.insert(
            tag_names.track_artist.clone(),
            args.value_of("track-artist").unwrap_or("").to_string(),
        );
    } else if args.is_present("config-file") {
        if let Some(val) = &defaults.track_artist {
            new_tags.insert(tag_names.track_artist.clone(), val.to_string());
        }
    }

    // ALBUM DETAILS //

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
        let file_count = common::count_files(filename)?;
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
    }

    Ok(None)
}

/// Convert a numerical ID3 genre to a string
/// Ref: <https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D>
#[allow(clippy::too_many_lines)] // Not much we can do about this one.
fn get_genre_name(tagnumber: u16) -> Result<String, Box<dyn Error>> {
    if tagnumber > 191 {
        return Err("Incorrect value supplied. Must be 0-191.".into());
    }

    let tags = vec![
        "Blues", // 0
        "Classic Rock",
        "Country",
        "Dance",
        "Disco",
        "Funk",
        "Grunge",
        "Hip-Hop",
        "Jazz",
        "Metal",
        "New Age", // 10
        "Oldies",
        "Other",
        "Pop",
        "Rhythm and Blues",
        "Rap",
        "Reggae",
        "Rock",
        "Techno",
        "Industrial",
        "Alternative", // 20
        "Ska",
        "Death Metal",
        "Pranks",
        "Soundtrack",
        "Euro-Techno",
        "Ambient",
        "Trip-Hop",
        "Vocal",
        "Jazz & Funk",
        "Fusion", // 30
        "Trance",
        "Classical",
        "Instrumental",
        "Acid",
        "House",
        "Game",
        "Sound clip",
        "Gospel",
        "Noise",
        "Alternative Rock", // 40
        "Bass",
        "Soul",
        "Punk",
        "Space",
        "Meditative",
        "Instrumental Pop",
        "Instrumental Rock",
        "Ethnic",
        "Gothic",
        "Darkwave", // 50
        "Techno-Industrial",
        "Electronic",
        "Pop-Folk",
        "Eurodance",
        "Dream",
        "Southern Rock",
        "Comedy",
        "Cult",
        "Gangsta",
        "Top 40", // 60
        "Christian Rap",
        "Pop/Funk",
        "Jungle",
        "Native US",
        "Cabaret",
        "New Wave",
        "Psychedelic",
        "Rave",
        "Show Tunes",
        "Trailer", // 70
        "Lo-Fi",
        "Tribal",
        "Acid Punk",
        "Acid Jazz",
        "Polka",
        "Retro",
        "Musical",
        "Rock 'n' Roll",
        "Hard Rock",
        "Folk", // 80
        "Folk-Rock",
        "National Folk",
        "Swing",
        "Fast Fusion",
        "Bebop",
        "Latin",
        "Revival",
        "Celtic",
        "Bluegrass",
        "Avantgarde", // 90
        "Gothic Rock",
        "Progressive Rock",
        "Psychedelic Rock",
        "Symphonic Rock",
        "Slow Rock",
        "Big Band",
        "Chorus",
        "Easy Listening",
        "Acoustic",
        "Humour", // 100
        "Speech",
        "Chanson",
        "Opera",
        "Chamber Music",
        "Sonata",
        "Symphony",
        "Booty Bass",
        "Primus",
        "Porn Groove",
        "Satire", // 110
        "Slow Jam",
        "Club",
        "Tango",
        "Samba",
        "Folklore",
        "Ballad",
        "Power Ballad",
        "Rhythmic Soul",
        "Freestyle",
        "Duet", // 120
        "Punk Rock",
        "Drum Solo",
        "A Cappella",
        "Euro-House",
        "Dancehall",
        "Goa",
        "Drum & Bass",
        "Club-House",
        "Hardcore Techno",
        "Terror", // 130
        "Indie",
        "BritPop",
        "Negerpunk",
        "Polsk Punk",
        "Beat",
        "Christian Gangsta Rap",
        "Heavy Metal",
        "Black Metal",
        "Crossover",
        "Contemporary Christian", // 140
        "Christian Rock",
        "Merengue",
        "Salsa",
        "Thrash Metal",
        "Anime",
        "Jpop",
        "Synthpop",
        "Abstract",
        "Art Rock",
        "Baroque", // 150
        "Bhangra",
        "Big Beat",
        "Breakbeat",
        "Chillout",
        "Downtempo",
        "Dub",
        "EBM",
        "Eclectic",
        "Electro",
        "Electroclash", // 160
        "Emo",
        "Experimental",
        "Garage",
        "Global",
        "IDM",
        "Illbient",
        "Industro-Goth",
        "Jam Band",
        "Krautrock",
        "Leftfield", // 170
        "Lounge",
        "Math Rock",
        "New Romantic",
        "Nu-Breakz",
        "Post-Punk",
        "Post-Rock",
        "Psytrance",
        "Shoegaze",
        "Space Rock",
        "Trop Rock", // 180
        "World Music",
        "Neoclassical",
        "Audiobook",
        "Audio Theatre",
        "Neue Deutche Welle",
        "Podcast",
        "Indie-Rock",
        "G-Funk",
        "Dubstep",
        "Garage Rock", // 190
        "Psybient",
    ];

    Ok(tags[tagnumber as usize].to_string())
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
        || parent_dir.starts_with("DISK")
        || parent_dir.starts_with("PART")
    {
        parent_dir = parent_dir.replace("CD", "");
        parent_dir = parent_dir.replace("DISC", "").trim().to_string();
        parent_dir = parent_dir.replace("DISK", "").trim().to_string();
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
            dn = common::roman_to_decimal(&parent_dir);

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
                || component_name.starts_with("DISK")
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

    #[assay(include = ["../testdata/sample.flac", "../testdata/sample.mp3"])]
    fn test_get_disc_number() {
        assert_eq!(get_disc_number("../testdata/sample.flac").unwrap(), 1);
        assert_eq!(get_disc_number("../testdata/sample.mp3").unwrap(), 1);
    }

    #[assay(include = ["../testdata/sample.flac", "../testdata/sample.mp3"])]
    fn test_get_disc_count() {
        assert_eq!(get_disc_count("../testdata/sample.flac").unwrap(), 1);
        assert_eq!(get_disc_count("../testdata/sample.mp3").unwrap(), 1);
    }
}
