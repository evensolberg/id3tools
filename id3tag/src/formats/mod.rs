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

/// Insert tags into the new tags list. Replaces mucho repeated code.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
/// - `$arg:expr` - The CLI parameter we're matching on
/// - `$name:ident` - The name of the variable in the `DefaultValues`
/// - `true|false` - Indicates whether to use the `clone()` version of the macro or not. The `clone()` version is used if the $name is used later in the function.
///
/// # Examples
///
/// `tag!(am, dv, nt, ot, "album-title", album_title, false);`
/// `tag!(am, dv, nt, ot, "disc-number", disc_number, true);`
macro_rules! tag {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $arg:expr, $name:ident, false) => {
        if $cli.is_present($arg) {
            $nt.insert($t.$name, $cli.value_of($arg).unwrap_or("").to_string());
        } else if $cli.is_present("config-file") {
            if let Some(val) = &$cfg.$name {
                $nt.insert($t.$name, val.to_string());
            }
        }
    };
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $arg:expr, $name:ident, true) => {
        if $cli.is_present($arg) {
            $nt.insert(
                $t.$name.clone(),
                $cli.value_of($arg).unwrap_or("").to_string(),
            );
        } else if $cli.is_present("config-file") {
            if let Some(val) = &$cfg.$name {
                $nt.insert($t.$name.clone(), val.to_string());
            }
        }
    };
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
/// - `Ok(bool)` if everything goes well. The boolean indicates whether the file was processed or not.
/// - `Box<dyn Error>` if we run into problems
pub fn process_file(
    file_type: FileTypes,
    filename: &str,
    cfg: &DefaultValues,
    cli_args: &clap::ArgMatches,
) -> Result<bool, Box<dyn Error>> {
    // Check if we need to create one or more cover images.
    let mut config = cfg.clone();
    let (front_cover_path, back_cover_path) = images::get_cover_filenames(filename, &config)?;
    log::debug!("front_cover_path = {front_cover_path:?}, back_cover_path = {back_cover_path:?}, ");

    if front_cover_path.is_some() {
        config.picture_front = front_cover_path;
    }

    if back_cover_path.is_some() {
        config.picture_back = back_cover_path;
    }

    let new_tags_result = parse_options(filename, file_type, &config, cli_args);

    let mut new_tags;
    let mut processed = false;

    // Process the music files(s)
    match new_tags_result {
        Ok(res) => {
            new_tags = res;
            let proc_res = match file_type {
                FileTypes::Ape => ape::process(filename, &new_tags, &config),
                FileTypes::Dsf => dsf::process(filename, &new_tags, &config),
                FileTypes::Flac => flac::process(filename, &mut new_tags, &config),
                FileTypes::MP3 => mp3::process(filename, &new_tags, &config),
                FileTypes::MP4 => mp4::process(filename, &new_tags, &config),
                FileTypes::Unknown => {
                    return Err(format!("{filename} is unknown file type.").into())
                }
            };

            match proc_res {
                Ok(_) => processed = true,
                Err(err) => {
                    if config.stop_on_error.unwrap_or(true) {
                        return Err(format!("Unable to process {filename}. Error: {err}").into());
                    }
                    log::error!("Unable to process {filename}. Error: {err}");
                }
            }
        } // Ok(_)
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!("Unable to parse tags for {filename}. Error: {err}").into());
            }
            log::error!("Unable to parse tags for {filename}. Error: {err}");
        } // Err(err)
    } // match new_tags_result

    // return safely
    Ok(processed)
}

/// Collect the various options/tags submitted into a `HashMap` for later use.
/// Also checks the default values loaded from a config file.
#[allow(clippy::cognitive_complexity)]
// TODO: This function is too long. Split it up.
fn parse_options(
    filename: &str,
    file_type: common::FileTypes,
    dv: &DefaultValues,
    am: &clap::ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut nt = HashMap::new();

    // Set tag names based on file type -- see tag_names function below
    let ot = tags::get_tag_names(file_type);

    // TODO: Refactor to check for -c and use, and then for parameter and overwrite.
    // TODO: Look into creating a macro for a bunch of the stuff below

    // ALBUM & TRACK ARTIST //

    if am.is_present("track-album-artist") {
        let taa = am.value_of("track-album-artist").unwrap_or("").to_string();
        nt.insert(ot.track_artist.clone(), taa.clone());
        nt.insert(ot.album_artist.clone(), taa);
    } else if am.is_present("config-file") {
        if let Some(val) = &dv.track_album_artist {
            let taa = val.to_string();
            nt.insert(ot.track_artist.clone(), taa.clone());
            nt.insert(ot.album_artist.clone(), taa);
        }
    }

    // We should never hit these two ("album-artist" and "track-artist") if we have the one above,
    // but the compiler doesn't know that. So we have to do a bunch of cloning above to ensure the
    // code below still compiles as expected.

    tag!(am, dv, nt, ot, "album-artist", album_artist, false);
    tag!(am, dv, nt, ot, "track-artist", track_artist, false);
    tag!(
        am,
        dv,
        nt,
        ot,
        "album-artist-sort",
        album_artist_sort,
        false
    );
    tag!(am, dv, nt, ot, "album-title", album_title, false);
    tag!(am, dv, nt, ot, "album-title-sort", album_title_sort, false);
    tag!(am, dv, nt, ot, "disc-number", disc_number, true);
    tag!(am, dv, nt, ot, "disc-total", disc_number_total, true);
    tag!(
        am,
        dv,
        nt,
        ot,
        "track-artist-sort",
        track_artist_sort,
        false
    );
    tag!(am, dv, nt, ot, "track-title", track_title, false);
    tag!(am, dv, nt, ot, "track-title-sort", track_title_sort, false);
    tag!(am, dv, nt, ot, "track-number", track_number, false);
    tag!(am, dv, nt, ot, "track-total", track_number_total, true);
    tag!(am, dv, nt, ot, "track-genre", track_genre, true);
    tag!(am, dv, nt, ot, "track-composer", track_composer, false);
    tag!(
        am,
        dv,
        nt,
        ot,
        "track-composer-sort",
        track_composer_sort,
        false
    );
    tag!(am, dv, nt, ot, "track-date", track_date, false);
    tag!(am, dv, nt, ot, "track-comments", track_comments, false);

    // Count the number of discs instead of taking a value
    if am.is_present("disc-number-count")
        || (am.is_present("config-file") && dv.disc_count.unwrap_or(false))
    {
        let disc_num = disc_number(filename)?;
        let disc_count = disc_count(filename)?;
        nt.insert(ot.disc_number.clone(), format!("{disc_num:0>2}"));
        nt.insert(ot.disc_number_total.clone(), format!("{disc_count:0>2}"));
    }

    // Count the number of tracks instead of taking a value
    if am.is_present("track-count")
        || (am.is_present("config-file") && dv.track_count.unwrap_or(false))
    {
        let file_count = common::count_files(filename)?;
        nt.insert(ot.track_number_total, file_count);
    }

    // Insert genre by number instead of name
    if am.is_present("track-genre-number") {
        nt.insert(
            ot.track_genre.clone(),
            genre_name(
                am.value_of("track-genre-number")
                    .unwrap_or_default()
                    .parse::<u16>()
                    .unwrap_or_default(),
            )?,
        );
    } else if am.is_present("config-file") {
        if let Some(val) = &dv.track_genre_number {
            nt.insert(ot.track_genre.clone(), genre_name(*val)?);
        }
    }

    if let Some(p) = &dv.picture_front {
        nt.insert(ot.picture_front, p.clone());
    }

    if let Some(p) = &dv.picture_back {
        nt.insert(ot.picture_back, p.clone());
    }

    Ok(nt)
}

/// Convert a numerical ID3 genre to a string
/// Ref: <https://en.wikipedia.org/wiki/ID3#Genre_list_in_ID3v1%5B12%5D>
#[allow(clippy::too_many_lines)] // Not much we can do about this one.
fn genre_name(tagnumber: u16) -> Result<String, Box<dyn Error>> {
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
// TODO: There may be a better way to do this by tokenizing the filename.
fn disc_number(filename: &str) -> Result<u16, Box<dyn Error>> {
    let mut parent_dir = common::directory(filename)?
        .to_str()
        .unwrap_or_default()
        .to_owned();

    let mut dn = 1; // Disc number
    let disc_candidates = disc_candidates();

    if disc_candidates
        .iter()
        .any(|&s| parent_dir.to_uppercase().starts_with(s))
    {
        parent_dir = disc_candidates
            .iter()
            .fold(parent_dir.to_uppercase(), |dir, c| {
                dir.replace(c, "").trim().to_owned()
            });

        // Check for longer name - eg CD01 - Something
        if parent_dir.contains(' ') || parent_dir.contains('-') {
            let space = parent_dir.find(' ').unwrap_or(256);
            let dash = parent_dir.find('-').unwrap_or(256);
            let delimiter = if space < dash { ' ' } else { '-' };

            parent_dir = parent_dir
                .split_once(delimiter)
                .unwrap_or_default()
                .0
                .to_string();
        }

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

    Ok(dn)
}

/// Counts the number of discs by looking for the number of `disk`, `CD` etc subdirectories
fn disc_count(filename: &str) -> Result<u16, Box<dyn Error>> {
    let disc_candidates = disc_candidates();
    log::trace!("disc_candidates = {disc_candidates:?}");

    // Get the full path so we can figure out the grandparent below
    let full_path = fs::canonicalize(filename)?;
    let grandparent_dir = full_path
        .ancestors()
        .nth(2)
        .unwrap_or_else(|| Path::new(&"."));

    let dirs = fs::read_dir(grandparent_dir)?;
    let mut disc_count = 0;
    for entry in dirs {
        let path = entry?.path();
        if path.is_dir() {
            let component_name = path
                .file_name()
                .unwrap_or(Component::CurDir.as_os_str())
                .to_string_lossy()
                .to_ascii_uppercase();
            log::trace!("component_name = {component_name}");

            if disc_candidates
                .iter()
                .any(|&s| component_name.to_uppercase().starts_with(s))
            {
                disc_count += 1;
            }
        }
    }

    log::trace!("disc_count = {disc_count}");

    // Obviously, we have at least 1 disc. Return accordingly.
    if disc_count == 0 {
        return Ok(1);
    }

    Ok(disc_count)
}

/// Returns a vector containing the candidates for a disc subdirectory.
/// This is a convenience function so that there's only one place to edit this list.
///
/// # Arguments
///
/// None.
///
/// # Returns
///
/// `Vec<&'static str>` - a static vector containing a list of candidates.
///
/// # Errors
///
/// None.
///
/// # Panics
///
/// None.
///
/// # Examples
///
/// ```
/// let dc = disc_candidates();
/// assert_eq!(dc[0], "CD");
/// ```
///
fn disc_candidates() -> Vec<&'static str> {
    vec!["CD", "DISC", "DISK", "PART"]
}

/* ====================
       TESTS
==================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests that the genre number gets returned correctly.
    fn test_get_genre_name() {
        assert_eq!(genre_name(0).unwrap(), "Blues".to_string());
        assert_eq!(genre_name(9).unwrap(), "Metal".to_string());
        assert_eq!(genre_name(32).unwrap(), "Classical".to_string());
        assert!(genre_name(200).is_err());
    }

    #[test]
    fn test_get_disc_number() {
        assert_eq!(disc_number("../testdata/sample.flac").unwrap(), 1);
        assert_eq!(disc_number("../testdata/sample.mp3").unwrap(), 1);
    }

    #[test]
    fn test_get_disc_count() {
        assert_eq!(disc_count("../testdata/sample.flac").unwrap(), 1);
        assert_eq!(disc_count("../testdata/sample.mp3").unwrap(), 1);
    }

    #[test]
    /// Tests the disc_candidates() function.
    fn test_disc_candidates() {
        let dc = disc_candidates();
        assert_eq!(dc[0], "CD");
    }
}
