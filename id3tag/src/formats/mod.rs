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
mod tag_macros;
mod tags;

// Import the macros
use crate::{disc_number_count, pic, tag, track_album_artist, track_genre_num, track_number_count};

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
    log::debug!("process_file::filename = {filename}");
    let mut config = cfg.clone();
    let (front_cover_path, back_cover_path) = images::get_cover_filenames(filename, &config)?;
    log::debug!("process_file::front_cover_path = {front_cover_path:?}, back_cover_path = {back_cover_path:?}, ");

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
    cli: &clap::ArgMatches,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut nt = HashMap::new();
    let ot = tags::get_tag_names(file_type);

    // Track and album artist at the same time.
    track_album_artist!(cli, dv, nt, ot);

    // We should never hit "album-artist" and "track-artist" if we have the one above,
    // but the compiler doesn't know that. So we have to do a bunch of cloning above to ensure the
    // code below still compiles as expected.
    tag!(cli, dv, nt, ot, "album-artist", album_artist, false);
    tag!(cli, dv, nt, ot, "track-artist", track_artist, false);
    tag!(
        cli,
        dv,
        nt,
        ot,
        "album-artist-sort",
        album_artist_sort,
        false
    );
    tag!(cli, dv, nt, ot, "album-title", album_title, false);
    tag!(cli, dv, nt, ot, "album-title-sort", album_title_sort, false);
    tag!(cli, dv, nt, ot, "disc-number", disc_number, true);
    tag!(cli, dv, nt, ot, "disc-total", disc_number_total, true);
    tag!(
        cli,
        dv,
        nt,
        ot,
        "track-artist-sort",
        track_artist_sort,
        false
    );
    tag!(cli, dv, nt, ot, "track-title", track_title, false);
    tag!(cli, dv, nt, ot, "track-title-sort", track_title_sort, false);
    tag!(cli, dv, nt, ot, "track-number", track_number, false);
    tag!(cli, dv, nt, ot, "track-total", track_number_total, true);
    tag!(cli, dv, nt, ot, "track-genre", track_genre, true);
    tag!(cli, dv, nt, ot, "track-composer", track_composer, false);
    tag!(
        cli,
        dv,
        nt,
        ot,
        "track-composer-sort",
        track_composer_sort,
        false
    );
    tag!(cli, dv, nt, ot, "track-date", track_date, false);
    tag!(cli, dv, nt, ot, "track-comments", track_comments, false);

    disc_number_count!(cli, dv, nt, ot, filename);
    track_number_count!(cli, dv, nt, ot, filename);
    track_genre_num!(cli, dv, nt, ot);

    pic!(cli, dv, nt, ot, front);
    pic!(cli, dv, nt, ot, back);

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
// TODO: Need to be able to handle cases like CD1of3 ("CD1 of 3" is fine)
fn disc_number(filename: &str) -> Result<u16, Box<dyn Error>> {
    let mut parent_dir = common::directory(filename)?
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();

    log::debug!("disc_number::parent_dir initial = {parent_dir}");

    let mut dn = 1; // Disc number
    let disc_candidates = disc_candidates();

    log::debug!("disc_number::disc_candidates = {disc_candidates:?}");

    if disc_candidates
        .iter()
        .any(|&s| parent_dir.to_uppercase().starts_with(s))
    {
        log::debug!("disc_number::parent_dir before = {parent_dir}");
        parent_dir = disc_candidates
            .iter()
            .fold(parent_dir.to_uppercase(), |dir, c| {
                dir.replace(c, "").trim().to_owned()
            });
        log::debug!("disc_number::parent_dir after = {parent_dir}");

        // Check for longer name - eg CD01 - Something
        if parent_dir.contains(' ') || parent_dir.contains('-') {
            let space = parent_dir.find(' ').unwrap_or(256);
            let dash = parent_dir.find('-').unwrap_or(256);
            let delimiter = if space < dash { ' ' } else { '-' };

            log::debug!("disc_number::space = {space}, dash = {dash}, delimiter = {delimiter}");

            parent_dir = parent_dir
                .split_once(delimiter)
                .unwrap_or_default()
                .0
                .to_string();

            log::debug!("disc_number::parent_dir final = {parent_dir}");
        }

        dn = parent_dir.parse().unwrap_or(0);
        log::debug!("disc_number::dn = {dn}");

        // Check for roman numerals
        if dn == 0 {
            log::debug!("disc_number::Checking for Roman numerals.");
            dn = common::roman_to_decimal(&parent_dir);

            // If roman --> decimal didn't work either, just go with 1.
            if dn == 0 {
                dn = 1;
            }
        }
    }

    log::debug!("disc_number::dn = {dn}");
    Ok(dn)
}

/// Counts the number of discs by looking for the number of `disk`, `CD` etc subdirectories
///
/// Calculates the number of discs based on the given filename.
///
/// This function reads the directory structure of the grandparent directory of the given filename
/// and counts the number of directories whose names match the disc candidates. The disc candidates
/// are determined by the `disc_candidates` function.
///
/// # Arguments
///
/// * `filename` - A string slice representing the path to the file.
///
/// # Returns
///
/// * `Result<u16, Box<dyn Error>>` - The number of discs as a `u16` if successful, otherwise an error.
///
/// # Examples
///
/// ```
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let filename = "/path/to/file.mp3";
///     let disc_count = disc_count(filename)?;
///     println!("Number of discs: {disc_count}");
///     Ok(())
/// }
/// ```
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
            log::debug!("component_name = {component_name}");

            if disc_candidates
                .iter()
                .any(|&s| component_name.to_uppercase().starts_with(s))
            {
                disc_count += 1;
            }
        }
    }

    log::debug!("disc_count = {disc_count}");

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
    vec!["CD", "DISC", "DISK", "PART", "VOL", "VOLUME"]
}

/// Retrieves the track number of a file based on its filename.
///
/// This function takes a filename as input and returns the track number of the file.
/// It first obtains the full path of the file by resolving any symbolic links or relative paths.
/// Then, it retrieves a list of files of the same type in the same directory as the input file.
/// The list is filtered to include only regular files with the same file extension as the input file.
/// The files are then sorted by name.
/// Finally, the function determines the index of the input file in the sorted list and returns the track number.
///
/// # Arguments
///
/// * `filename` - A string slice that represents the filename of the file to be processed.
///
/// # Returns
///
/// * `Result<usize, Box<dyn Error>>` - The track number of the file, wrapped in a `Result` indicating success or failure.
///   If the track number cannot be determined, an error is returned.
///
/// # Examples
///
/// ```
/// use std::error::Error;
/// use std::fs;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let filename = "/path/to/file.mp3";
///     let track_number = track_number(filename)?;
///     println!("Track number: {track_number}");
///     Ok(())
/// }
/// ```
fn track_number(filename: &str) -> Result<usize, Box<dyn Error>> {
    let full_path = fs::canonicalize(filename)?;

    // Get the list of files of the same type in the same directory
    let parent_path = full_path.parent().unwrap_or_else(|| Path::new(""));
    let mut files = fs::read_dir(parent_path)?
        .filter_map(Result::ok)
        .filter(|f| f.path().is_file())
        .filter(|f| {
            f.path()
                .extension()
                .map(std::ffi::OsStr::to_ascii_uppercase)
                == full_path
                    .extension()
                    .map(std::ffi::OsStr::to_ascii_uppercase)
        })
        .collect::<Vec<_>>();

    // Sort the files by name
    files.sort_by_key(std::fs::DirEntry::path);

    // Get the index of the current file in the list
    let track_number = files
        .iter()
        .position(|file| file.path() == full_path)
        .map_or(0, |i| i + 1);

    Ok(track_number)
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
    /// Tests the `disc_candidates`() function.
    fn test_disc_candidates() {
        let dc = disc_candidates();
        assert_eq!(dc[0], "CD");
    }

    #[test]
    /// Tests that the track number gets returned correctly.
    /// This test assumes that the files are sorted by name.
    ///
    /// TODO: This test is not very good. It should be rewritten to use a temporary directory
    fn test_track_number() {
        assert_eq!(track_number("../t_mp3/CD 1/02. Titanskull.mp3").unwrap(), 2);
        assert_eq!(
            track_number("../t_mp3/CD 1/10. The Gods All Sleep.mp3").unwrap(),
            10
        );
        assert_eq!(track_number("../testdata/sample.flac").unwrap(), 1);
    }
}
