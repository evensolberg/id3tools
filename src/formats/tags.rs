//! Contains the scaffoldig for processing tags in a generic way

use super::FileTypes;
use std::collections::HashMap;

/// Used to store the various tag names based on the file type.
/// This is used in the `parse_options` function/
#[derive(Debug, Default, Clone)]
pub struct TagNames {
    pub album_artist: String,
    pub album_artist_sort: String,
    pub album_title: String,
    pub album_title_sort: String,
    pub disc_number: String,
    pub disc_number_total: String,
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

/// Gets the tag names based on the file type
pub fn get_tag_names(file_type: FileTypes) -> TagNames {
    match file_type {
        FileTypes::Ape | FileTypes::Flac => TagNames {
            album_artist: "ALBUMARTIST".to_string(),
            album_artist_sort: "ALBUMARTISTSORT".to_string(),
            album_title: "ALBUM".to_string(),
            album_title_sort: "ALBUMTITLESORT".to_string(),
            disc_number: "DISCNUMBER".to_string(),
            disc_number_total: "DISCTOTAL".to_string(),
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
            disc_number_total: "TPOS-T".to_string(),
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
            disc_number_total: "disk-t".to_string(),
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
            disc_number_total: "".to_string(),
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

/// Returns a HashMap with the tag options and tag option aliases mapped to the right tag name based on file type.
/// Eg: %album-artist or %aa --> ALBUMARTIST (FLAC), TPE2 (MP3) or aART (MP4)
pub fn option_to_tag(file_type: FileTypes) -> HashMap<String, String> {
    let tag_names = get_tag_names(file_type);

    let mut tm = HashMap::new();
    tm.insert("%album-artist".to_string(), tag_names.album_artist.clone());
    tm.insert("%aa".to_string(), tag_names.album_artist.clone());

    tm.insert(
        "%album-artist-sort".to_string(),
        tag_names.album_artist_sort.clone(),
    );
    tm.insert("%aas".to_string(), tag_names.album_artist_sort.clone());

    tm.insert("%album-title".to_string(), tag_names.album_title.clone());
    tm.insert("%at".to_string(), tag_names.album_title.clone());

    tm.insert(
        "%album-title-sort".to_string(),
        tag_names.album_title_sort.clone(),
    );
    tm.insert("%ats".to_string(), tag_names.album_title_sort.clone());

    tm.insert("%disc-number".to_string(), tag_names.disc_number.clone());
    tm.insert("%dn".to_string(), tag_names.disc_number.clone());

    tm.insert(
        "%disc-number-total".to_string(),
        tag_names.disc_number_total.clone(),
    );
    tm.insert("%dt".to_string(), tag_names.disc_number_total.clone());

    tm.insert("%track-artist".to_string(), tag_names.track_artist.clone());
    tm.insert("%ta".to_string(), tag_names.track_artist.clone());

    tm.insert(
        "%track-artist-sort".to_string(),
        tag_names.track_artist_sort.clone(),
    );
    tm.insert("%tas".to_string(), tag_names.track_artist_sort.clone());

    tm.insert("%track-title".to_string(), tag_names.track_title.clone());
    tm.insert("%tt".to_string(), tag_names.track_title.clone());

    tm.insert(
        "%track-title-sort".to_string(),
        tag_names.track_title_sort.clone(),
    );
    tm.insert("%tts".to_string(), tag_names.track_title_sort.clone());

    tm.insert("%track-number".to_string(), tag_names.track_number.clone());
    tm.insert("%tn".to_string(), tag_names.track_number.clone());

    tm.insert(
        "%track-number-total".to_string(),
        tag_names.track_number_total.clone(),
    );
    tm.insert("%to".to_string(), tag_names.track_number_total.clone());

    tm.insert("%track-genre".to_string(), tag_names.track_genre.clone());
    tm.insert("%tg".to_string(), tag_names.track_genre.clone());

    tm.insert(
        "%track-composer".to_string(),
        tag_names.track_composer.clone(),
    );
    tm.insert("%tc".to_string(), tag_names.track_composer.clone());

    tm.insert(
        "%track-composer-sort".to_string(),
        tag_names.track_composer_sort.clone(),
    );
    tm.insert("%tcs".to_string(), tag_names.track_composer_sort.clone());

    tm.insert("%track-date".to_string(), tag_names.track_date.clone());
    tm.insert("%td".to_string(), tag_names.track_date);

    // return it
    tm
}
