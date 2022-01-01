use env_logger::{Builder, Target};
use log::LevelFilter;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

use crate::default_values::DefaultValues;
use crate::formats::FileTypes;

#[derive(Debug, Default, Clone, Copy)]
pub struct Counts {
    pub total_file_count: usize,
    pub processed_file_count: usize,
    pub skipped_file_count: usize,
}

/// Used to store the various tag names based on the file type.
/// This is used in the `parse_options` function/
#[derive(Debug, Default, Clone)]
pub struct TagNames {
    pub album_artist: String,
    pub album_artist_sort: String,
    pub album_title: String,
    pub album_title_sort: String,
    pub disc_number: String,
    pub disc_total: String,
    pub track_artist: String,
    pub track_artist_sort: String,
    pub track_title: String,
    pub track_title_sort: String,
    pub track_number: String,
    pub track_number_total: String,
    pub track_genre: String,
    pub track_composer: String,
    pub track_composer_sort: String,
    pub track_date: String,
    pub track_comments: String,
    pub picture_front: String,
    pub picture_back: String,
}

/// Find the MIME type (ie. `image/[bmp|gif|jpeg|png|tiff`) based on the file extension. Not perfect, but it'll do for now.
pub fn mime_type(filename: &str) -> Result<String, Box<dyn Error>> {
    let ext = get_extension(filename);
    let fmt_str = match ext.as_ref() {
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "tif" | "tiff" => "image/tiff",
        _ => {
            return Err(
                "Image format not supported. Must be one of BMP, GIF, JPEG, PNG or TIFF.".into(),
            )
        }
    };

    // Return safely
    Ok(fmt_str.to_string())
}

/// Get the extension part of the filename and return it as a string
pub fn get_extension(filename: &str) -> String {
    Path::new(&filename)
        .extension()
        .unwrap_or_else(|| OsStr::new("unknown"))
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Looks for the picture file with the name supplied. Initially tries to find it in the path of the music file.
/// If unsuccessful, tries to find it in the invocation directory. If still unsuccessful returns either None or
/// an Error, depending on whether the `stop_on_error` flag has been set.
pub fn find_picture(
    m_filename: &str,
    p_filename: &str,
    config: &DefaultValues,
) -> Result<Option<String>, Box<dyn Error>> {
    // Assume that the music file exists
    let m_path_name;
    if let Some(base_path) = Path::new(&m_filename).parent() {
        m_path_name = base_path;
    } else {
        m_path_name = Path::new(".");
    };

    log::debug!("music path_name = {:?}", m_path_name);

    if Path::new(m_path_name).join(p_filename).exists() {
        // Picture file exists alongside the music file
        log::debug!(
            "picture file path: {}",
            Path::new(m_path_name).join(p_filename).to_string_lossy()
        );
        return Ok(Some(
            Path::new(m_path_name)
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
        return Ok(None);
    }
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
pub fn get_tag_names(file_type: FileTypes) -> TagNames {
    match file_type {
        FileTypes::Ape | FileTypes::Flac => TagNames {
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
        FileTypes::MP3 => TagNames {
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
        FileTypes::MP4 => TagNames {
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
        FileTypes::Unknown => TagNames {
            album_artist: "".to_string(),
            album_artist_sort: "".to_string(),
            album_title: "".to_string(),
            album_title_sort: "".to_string(),
            disc_number: "".to_string(),
            disc_total: "".to_string(),
            track_artist: "".to_string(),
            track_artist_sort: "".to_string(),
            track_title: "".to_string(),
            track_title_sort: "".to_string(),
            track_number: "".to_string(),
            track_number_total: "".to_string(),
            track_genre: "".to_string(),
            track_composer: "".to_string(),
            track_composer_sort: "".to_string(),
            track_date: "".to_string(),
            track_comments: "".to_string(),
            picture_front: "".to_string(),
            picture_back: "".to_string(),
        },
    }
}

/// Creates a log entity for us
pub fn build_log(cli_args: &clap::ArgMatches) -> Result<Builder, Box<dyn Error>> {
    let mut logbuilder = Builder::new();

    if cli_args.is_present("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.occurrences_of("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    logbuilder.filter_module("metaflac::block", LevelFilter::Warn);
    logbuilder.target(Target::Stdout).init();

    Ok(logbuilder)
}
