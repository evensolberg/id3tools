//! Contains the scaffoldig for processing tags in a generic way

use common::FileTypes;
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
        // May need to revisit this, so keeping it for now.
        // FileTypes::Ape => TagNames {
        //     album_artist: "Artist".to_string(),
        //     album_artist_sort: "ArtistSort".to_string(),
        //     album_title: "Album".to_string(),
        //     album_title_sort: "AlbumSort".to_string(),
        //     disc_number: "Media".to_string(),
        //     disc_number_total: "MediaTotal".to_string(),
        //     track_artist: "Artist".to_string(),
        //     track_artist_sort: "ArtistSort".to_string(),
        //     track_title: "Title".to_string(),
        //     track_title_sort: "TitleSort".to_string(),
        //     track_number: "Track".to_string(),
        //     track_number_total: "TrackTotal".to_string(),
        //     track_genre: "Genre".to_string(),
        //     track_composer: "Composer".to_string(),
        //     track_composer_sort: "ComposerSort".to_string(),
        //     track_date: "Year".to_string(),
        //     track_comments: "Comment".to_string(),
        //     picture_front: "PICTUREFRONT".to_string(),
        //     picture_back: "PICTUREBACK".to_string(),
        // },
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
        FileTypes::MP3 | FileTypes::Dsf => TagNames {
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

/// Returns a `HashMap` with the tag options and tag option aliases mapped to the right tag name based on file type.
/// Eg: %album-artist or %aa --> ALBUMARTIST (FLAC), TPE2 (MP3) or `aART` (MP4)
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
    tm.insert("%dnt".to_string(), tag_names.disc_number_total.clone());

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
    tm.insert("%tnt".to_string(), tag_names.track_number_total.clone());

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

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    /// Test that the right tag names are being returned.
    fn test_get_tag_names() {
        let ape_tag = get_tag_names(FileTypes::Ape);
        assert_eq!(ape_tag.album_artist, "ALBUMARTIST".to_string());
        assert_eq!(ape_tag.album_artist_sort, "ALBUMARTISTSORT".to_string());
        assert_eq!(ape_tag.album_title, "ALBUM".to_string());
        assert_eq!(ape_tag.album_title_sort, "ALBUMTITLESORT".to_string());
        assert_eq!(ape_tag.disc_number, "DISCNUMBER".to_string());
        assert_eq!(ape_tag.disc_number_total, "DISCTOTAL".to_string());
        assert_eq!(ape_tag.track_artist, "ARTIST".to_string());
        assert_eq!(ape_tag.track_artist_sort, "ARTISTSORT".to_string());
        assert_eq!(ape_tag.track_title, "TITLE".to_string());
        assert_eq!(ape_tag.track_title_sort, "TITLESORT".to_string());
        assert_eq!(ape_tag.track_number, "TRACKNUMBER".to_string());
        assert_eq!(ape_tag.track_number_total, "TRACKTOTAL".to_string());
        assert_eq!(ape_tag.track_genre, "GENRE".to_string());
        assert_eq!(ape_tag.track_composer, "COMPOSER".to_string());
        assert_eq!(ape_tag.track_composer_sort, "COMPOSERSORT".to_string());
        assert_eq!(ape_tag.track_date, "DATE".to_string());
        assert_eq!(ape_tag.track_comments, "DESCRIPTION".to_string());
        assert_eq!(ape_tag.picture_front, "PICTUREFRONT".to_string());
        assert_eq!(ape_tag.picture_back, "PICTUREBACK".to_string());

        let flac_tag = get_tag_names(FileTypes::Flac);
        assert_eq!(flac_tag.album_artist, "ALBUMARTIST".to_string());
        assert_eq!(flac_tag.album_artist_sort, "ALBUMARTISTSORT".to_string());
        assert_eq!(flac_tag.album_title, "ALBUM".to_string());
        assert_eq!(flac_tag.album_title_sort, "ALBUMTITLESORT".to_string());
        assert_eq!(flac_tag.disc_number, "DISCNUMBER".to_string());
        assert_eq!(flac_tag.disc_number_total, "DISCTOTAL".to_string());
        assert_eq!(flac_tag.track_artist, "ARTIST".to_string());
        assert_eq!(flac_tag.track_artist_sort, "ARTISTSORT".to_string());
        assert_eq!(flac_tag.track_title, "TITLE".to_string());
        assert_eq!(flac_tag.track_title_sort, "TITLESORT".to_string());
        assert_eq!(flac_tag.track_number, "TRACKNUMBER".to_string());
        assert_eq!(flac_tag.track_number_total, "TRACKTOTAL".to_string());
        assert_eq!(flac_tag.track_genre, "GENRE".to_string());
        assert_eq!(flac_tag.track_composer, "COMPOSER".to_string());
        assert_eq!(flac_tag.track_composer_sort, "COMPOSERSORT".to_string());
        assert_eq!(flac_tag.track_date, "DATE".to_string());
        assert_eq!(flac_tag.track_comments, "DESCRIPTION".to_string());
        assert_eq!(flac_tag.picture_front, "PICTUREFRONT".to_string());
        assert_eq!(flac_tag.picture_back, "PICTUREBACK".to_string());

        let mp3_tag = get_tag_names(FileTypes::MP3);
        assert_eq!(mp3_tag.album_artist, "TPE2".to_string());
        assert_eq!(mp3_tag.album_artist_sort, "TSO2".to_string());
        assert_eq!(mp3_tag.album_title, "TALB".to_string());
        assert_eq!(mp3_tag.album_title_sort, "TSOA".to_string());
        assert_eq!(mp3_tag.disc_number, "TPOS".to_string());
        assert_eq!(mp3_tag.disc_number_total, "TPOS-T".to_string());
        assert_eq!(mp3_tag.track_artist, "TPE1".to_string());
        assert_eq!(mp3_tag.track_artist_sort, "TSOP".to_string());
        assert_eq!(mp3_tag.track_title, "TIT2".to_string());
        assert_eq!(mp3_tag.track_title_sort, "TSOT".to_string());
        assert_eq!(mp3_tag.track_number, "TRCK".to_string());
        assert_eq!(mp3_tag.track_number_total, "TRCK-T".to_string());
        assert_eq!(mp3_tag.track_genre, "TCON".to_string());
        assert_eq!(mp3_tag.track_composer, "TCOM".to_string());
        assert_eq!(mp3_tag.track_composer_sort, "TSOC".to_string());
        assert_eq!(mp3_tag.track_date, "TDRC".to_string());
        assert_eq!(mp3_tag.track_comments, "COMM".to_string());
        assert_eq!(mp3_tag.picture_front, "APIC-F".to_string());
        assert_eq!(mp3_tag.picture_back, "APIC-B".to_string());

        let mp4_tag = get_tag_names(FileTypes::MP4);
        assert_eq!(mp4_tag.album_artist, "aART".to_string());
        assert_eq!(mp4_tag.album_artist_sort, "soaa".to_string());
        assert_eq!(mp4_tag.album_title, "©alb".to_string());
        assert_eq!(mp4_tag.album_title_sort, "soal".to_string());
        assert_eq!(mp4_tag.disc_number, "disk".to_string());
        assert_eq!(mp4_tag.disc_number_total, "disk-t".to_string());
        assert_eq!(mp4_tag.track_artist, "©ART".to_string());
        assert_eq!(mp4_tag.track_artist_sort, "soar".to_string());
        assert_eq!(mp4_tag.track_title, "©nam".to_string());
        assert_eq!(mp4_tag.track_title_sort, "sonm".to_string());
        assert_eq!(mp4_tag.track_number, "trkn".to_string());
        assert_eq!(mp4_tag.track_number_total, "trkn-t".to_string());
        assert_eq!(mp4_tag.track_genre, "©gen".to_string());
        assert_eq!(mp4_tag.track_composer, "©wrt".to_string());
        assert_eq!(mp4_tag.track_composer_sort, "soco".to_string());
        assert_eq!(mp4_tag.track_date, "©day".to_string());
        assert_eq!(mp4_tag.track_comments, "©cmt".to_string());
        assert_eq!(mp4_tag.picture_front, "covr-f".to_string());
        assert_eq!(mp4_tag.picture_back, "covr-b".to_string());

        let unk_tag = get_tag_names(FileTypes::Unknown);
        assert_eq!(unk_tag.album_artist, "".to_string());
        assert_eq!(unk_tag.album_artist_sort, "".to_string());
        assert_eq!(unk_tag.album_title, "".to_string());
        assert_eq!(unk_tag.album_title_sort, "".to_string());
        assert_eq!(unk_tag.disc_number, "".to_string());
        assert_eq!(unk_tag.disc_number_total, "".to_string());
        assert_eq!(unk_tag.track_artist, "".to_string());
        assert_eq!(unk_tag.track_artist_sort, "".to_string());
        assert_eq!(unk_tag.track_title, "".to_string());
        assert_eq!(unk_tag.track_title_sort, "".to_string());
        assert_eq!(unk_tag.track_number, "".to_string());
        assert_eq!(unk_tag.track_number_total, "".to_string());
        assert_eq!(unk_tag.track_genre, "".to_string());
        assert_eq!(unk_tag.track_composer, "".to_string());
        assert_eq!(unk_tag.track_composer_sort, "".to_string());
        assert_eq!(unk_tag.track_date, "".to_string());
        assert_eq!(unk_tag.track_comments, "".to_string());
        assert_eq!(unk_tag.picture_front, "".to_string());
        assert_eq!(unk_tag.picture_back, "".to_string());
    }

    #[assay]
    /// Ensure that the substitution values are being used properly.
    /// Note that values for description and front/back pictures aren't used. Obviously.
    fn test_option_to_tag() {
        let ape_tag = option_to_tag(FileTypes::Ape);
        assert_eq!(
            ape_tag.get("%album-artist").unwrap().to_owned(),
            "ALBUMARTIST".to_string()
        );
        assert_eq!(
            ape_tag.get("%aa").unwrap().to_owned(),
            "ALBUMARTIST".to_string()
        );
        assert_eq!(
            ape_tag.get("%album-artist-sort").unwrap().to_owned(),
            "ALBUMARTISTSORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%aas").unwrap().to_owned(),
            "ALBUMARTISTSORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%album-title").unwrap().to_owned(),
            "ALBUM".to_string()
        );
        assert_eq!(ape_tag.get("%at").unwrap().to_owned(), "ALBUM".to_string());
        assert_eq!(
            ape_tag.get("%album-title-sort").unwrap().to_owned(),
            "ALBUMTITLESORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%ats").unwrap().to_owned(),
            "ALBUMTITLESORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%disc-number").unwrap().to_owned(),
            "DISCNUMBER".to_string()
        );
        assert_eq!(
            ape_tag.get("%dn").unwrap().to_owned(),
            "DISCNUMBER".to_string()
        );
        assert_eq!(
            ape_tag.get("%disc-number-total").unwrap().to_owned(),
            "DISCTOTAL".to_string()
        );
        assert_eq!(
            ape_tag.get("%dnt").unwrap().to_owned(),
            "DISCTOTAL".to_string()
        );
        assert_eq!(
            ape_tag.get("%dt").unwrap().to_owned(),
            "DISCTOTAL".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-artist").unwrap().to_owned(),
            "ARTIST".to_string()
        );
        assert_eq!(ape_tag.get("%ta").unwrap().to_owned(), "ARTIST".to_string());
        assert_eq!(
            ape_tag.get("%track-artist-sort").unwrap().to_owned(),
            "ARTISTSORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%tas").unwrap().to_owned(),
            "ARTISTSORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-title").unwrap().to_owned(),
            "TITLE".to_string()
        );
        assert_eq!(ape_tag.get("%tt").unwrap().to_owned(), "TITLE".to_string());
        assert_eq!(
            ape_tag.get("%track-title-sort").unwrap().to_owned(),
            "TITLESORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%tts").unwrap().to_owned(),
            "TITLESORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-number").unwrap().to_owned(),
            "TRACKNUMBER".to_string()
        );
        assert_eq!(
            ape_tag.get("%tn").unwrap().to_owned(),
            "TRACKNUMBER".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-number-total").unwrap().to_owned(),
            "TRACKTOTAL".to_string()
        );
        assert_eq!(
            ape_tag.get("%to").unwrap().to_owned(),
            "TRACKTOTAL".to_string()
        );
        assert_eq!(
            ape_tag.get("%tnt").unwrap().to_owned(),
            "TRACKTOTAL".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-genre").unwrap().to_owned(),
            "GENRE".to_string()
        );
        assert_eq!(ape_tag.get("%tg").unwrap().to_owned(), "GENRE".to_string());
        assert_eq!(
            ape_tag.get("%track-composer").unwrap().to_owned(),
            "COMPOSER".to_string()
        );
        assert_eq!(
            ape_tag.get("%tc").unwrap().to_owned(),
            "COMPOSER".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-composer-sort").unwrap().to_owned(),
            "COMPOSERSORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%tcs").unwrap().to_owned(),
            "COMPOSERSORT".to_string()
        );
        assert_eq!(
            ape_tag.get("%track-date").unwrap().to_owned(),
            "DATE".to_string()
        );
        assert_eq!(ape_tag.get("%td").unwrap().to_owned(), "DATE".to_string());

        let flac_tag = option_to_tag(FileTypes::Flac);
        assert_eq!(
            flac_tag.get("%album-artist").unwrap().to_owned(),
            "ALBUMARTIST".to_string()
        );
        assert_eq!(
            flac_tag.get("%aa").unwrap().to_owned(),
            "ALBUMARTIST".to_string()
        );
        assert_eq!(
            flac_tag.get("%album-artist-sort").unwrap().to_owned(),
            "ALBUMARTISTSORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%aas").unwrap().to_owned(),
            "ALBUMARTISTSORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%album-title").unwrap().to_owned(),
            "ALBUM".to_string()
        );
        assert_eq!(flac_tag.get("%at").unwrap().to_owned(), "ALBUM".to_string());
        assert_eq!(
            flac_tag.get("%album-title-sort").unwrap().to_owned(),
            "ALBUMTITLESORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%ats").unwrap().to_owned(),
            "ALBUMTITLESORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%disc-number").unwrap().to_owned(),
            "DISCNUMBER".to_string()
        );
        assert_eq!(
            flac_tag.get("%dn").unwrap().to_owned(),
            "DISCNUMBER".to_string()
        );
        assert_eq!(
            flac_tag.get("%disc-number-total").unwrap().to_owned(),
            "DISCTOTAL".to_string()
        );
        assert_eq!(
            flac_tag.get("%dnt").unwrap().to_owned(),
            "DISCTOTAL".to_string()
        );
        assert_eq!(
            flac_tag.get("%dt").unwrap().to_owned(),
            "DISCTOTAL".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-artist").unwrap().to_owned(),
            "ARTIST".to_string()
        );
        assert_eq!(
            flac_tag.get("%ta").unwrap().to_owned(),
            "ARTIST".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-artist-sort").unwrap().to_owned(),
            "ARTISTSORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%tas").unwrap().to_owned(),
            "ARTISTSORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-title").unwrap().to_owned(),
            "TITLE".to_string()
        );
        assert_eq!(flac_tag.get("%tt").unwrap().to_owned(), "TITLE".to_string());
        assert_eq!(
            flac_tag.get("%track-title-sort").unwrap().to_owned(),
            "TITLESORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%tts").unwrap().to_owned(),
            "TITLESORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-number").unwrap().to_owned(),
            "TRACKNUMBER".to_string()
        );
        assert_eq!(
            flac_tag.get("%tn").unwrap().to_owned(),
            "TRACKNUMBER".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-number-total").unwrap().to_owned(),
            "TRACKTOTAL".to_string()
        );
        assert_eq!(
            flac_tag.get("%to").unwrap().to_owned(),
            "TRACKTOTAL".to_string()
        );
        assert_eq!(
            flac_tag.get("%tnt").unwrap().to_owned(),
            "TRACKTOTAL".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-genre").unwrap().to_owned(),
            "GENRE".to_string()
        );
        assert_eq!(flac_tag.get("%tg").unwrap().to_owned(), "GENRE".to_string());
        assert_eq!(
            flac_tag.get("%track-composer").unwrap().to_owned(),
            "COMPOSER".to_string()
        );
        assert_eq!(
            flac_tag.get("%tc").unwrap().to_owned(),
            "COMPOSER".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-composer-sort").unwrap().to_owned(),
            "COMPOSERSORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%tcs").unwrap().to_owned(),
            "COMPOSERSORT".to_string()
        );
        assert_eq!(
            flac_tag.get("%track-date").unwrap().to_owned(),
            "DATE".to_string()
        );
        assert_eq!(flac_tag.get("%td").unwrap().to_owned(), "DATE".to_string());

        let mp3_tag = option_to_tag(FileTypes::MP3);
        assert_eq!(
            mp3_tag.get("%album-artist").unwrap().to_owned(),
            "TPE2".to_string()
        );
        assert_eq!(mp3_tag.get("%aa").unwrap().to_owned(), "TPE2".to_string());
        assert_eq!(
            mp3_tag.get("%album-artist-sort").unwrap().to_owned(),
            "TSO2".to_string()
        );
        assert_eq!(mp3_tag.get("%aas").unwrap().to_owned(), "TSO2".to_string());
        assert_eq!(
            mp3_tag.get("%album-title").unwrap().to_owned(),
            "TALB".to_string()
        );
        assert_eq!(mp3_tag.get("%at").unwrap().to_owned(), "TALB".to_string());
        assert_eq!(
            mp3_tag.get("%album-title-sort").unwrap().to_owned(),
            "TSOA".to_string()
        );
        assert_eq!(mp3_tag.get("%ats").unwrap().to_owned(), "TSOA".to_string());
        assert_eq!(
            mp3_tag.get("%disc-number").unwrap().to_owned(),
            "TPOS".to_string()
        );
        assert_eq!(mp3_tag.get("%dn").unwrap().to_owned(), "TPOS".to_string());
        assert_eq!(
            mp3_tag.get("%disc-number-total").unwrap().to_owned(),
            "TPOS-T".to_string()
        );
        assert_eq!(
            mp3_tag.get("%dnt").unwrap().to_owned(),
            "TPOS-T".to_string()
        );
        assert_eq!(mp3_tag.get("%dt").unwrap().to_owned(), "TPOS-T".to_string());
        assert_eq!(
            mp3_tag.get("%track-artist").unwrap().to_owned(),
            "TPE1".to_string()
        );
        assert_eq!(mp3_tag.get("%ta").unwrap().to_owned(), "TPE1".to_string());
        assert_eq!(
            mp3_tag.get("%track-artist-sort").unwrap().to_owned(),
            "TSOP".to_string()
        );
        assert_eq!(mp3_tag.get("%tas").unwrap().to_owned(), "TSOP".to_string());
        assert_eq!(
            mp3_tag.get("%track-title").unwrap().to_owned(),
            "TIT2".to_string()
        );
        assert_eq!(mp3_tag.get("%tt").unwrap().to_owned(), "TIT2".to_string());
        assert_eq!(
            mp3_tag.get("%track-title-sort").unwrap().to_owned(),
            "TSOT".to_string()
        );
        assert_eq!(mp3_tag.get("%tts").unwrap().to_owned(), "TSOT".to_string());
        assert_eq!(
            mp3_tag.get("%track-number").unwrap().to_owned(),
            "TRCK".to_string()
        );
        assert_eq!(mp3_tag.get("%tn").unwrap().to_owned(), "TRCK".to_string());
        assert_eq!(
            mp3_tag.get("%track-number-total").unwrap().to_owned(),
            "TRCK-T".to_string()
        );
        assert_eq!(mp3_tag.get("%to").unwrap().to_owned(), "TRCK-T".to_string());
        assert_eq!(
            mp3_tag.get("%tnt").unwrap().to_owned(),
            "TRCK-T".to_string()
        );
        assert_eq!(
            mp3_tag.get("%track-genre").unwrap().to_owned(),
            "TCON".to_string()
        );
        assert_eq!(mp3_tag.get("%tg").unwrap().to_owned(), "TCON".to_string());
        assert_eq!(
            mp3_tag.get("%track-composer").unwrap().to_owned(),
            "TCOM".to_string()
        );
        assert_eq!(mp3_tag.get("%tc").unwrap().to_owned(), "TCOM".to_string());
        assert_eq!(
            mp3_tag.get("%track-composer-sort").unwrap().to_owned(),
            "TSOC".to_string()
        );
        assert_eq!(mp3_tag.get("%tcs").unwrap().to_owned(), "TSOC".to_string());
        assert_eq!(
            mp3_tag.get("%track-date").unwrap().to_owned(),
            "TDRC".to_string()
        );
        assert_eq!(mp3_tag.get("%td").unwrap().to_owned(), "TDRC".to_string());

        let mp4_tag = option_to_tag(FileTypes::MP4);
        assert_eq!(
            mp4_tag.get("%album-artist").unwrap().to_owned(),
            "aART".to_string()
        );
        assert_eq!(mp4_tag.get("%aa").unwrap().to_owned(), "aART".to_string());
        assert_eq!(
            mp4_tag.get("%album-artist-sort").unwrap().to_owned(),
            "soaa".to_string()
        );
        assert_eq!(mp4_tag.get("%aas").unwrap().to_owned(), "soaa".to_string());
        assert_eq!(
            mp4_tag.get("%album-title").unwrap().to_owned(),
            "©alb".to_string()
        );
        assert_eq!(mp4_tag.get("%at").unwrap().to_owned(), "©alb".to_string());
        assert_eq!(
            mp4_tag.get("%album-title-sort").unwrap().to_owned(),
            "soal".to_string()
        );
        assert_eq!(mp4_tag.get("%ats").unwrap().to_owned(), "soal".to_string());
        assert_eq!(
            mp4_tag.get("%disc-number").unwrap().to_owned(),
            "disk".to_string()
        );
        assert_eq!(mp4_tag.get("%dn").unwrap().to_owned(), "disk".to_string());
        assert_eq!(
            mp4_tag.get("%disc-number-total").unwrap().to_owned(),
            "disk-t".to_string()
        );
        assert_eq!(
            mp4_tag.get("%dnt").unwrap().to_owned(),
            "disk-t".to_string()
        );
        assert_eq!(mp4_tag.get("%dt").unwrap().to_owned(), "disk-t".to_string());
        assert_eq!(
            mp4_tag.get("%track-artist").unwrap().to_owned(),
            "©ART".to_string()
        );
        assert_eq!(mp4_tag.get("%ta").unwrap().to_owned(), "©ART".to_string());
        assert_eq!(
            mp4_tag.get("%track-artist-sort").unwrap().to_owned(),
            "soar".to_string()
        );
        assert_eq!(mp4_tag.get("%tas").unwrap().to_owned(), "soar".to_string());
        assert_eq!(
            mp4_tag.get("%track-title").unwrap().to_owned(),
            "©nam".to_string()
        );
        assert_eq!(mp4_tag.get("%tt").unwrap().to_owned(), "©nam".to_string());
        assert_eq!(
            mp4_tag.get("%track-title-sort").unwrap().to_owned(),
            "sonm".to_string()
        );
        assert_eq!(mp4_tag.get("%tts").unwrap().to_owned(), "sonm".to_string());
        assert_eq!(
            mp4_tag.get("%track-number").unwrap().to_owned(),
            "trkn".to_string()
        );
        assert_eq!(mp4_tag.get("%tn").unwrap().to_owned(), "trkn".to_string());
        assert_eq!(
            mp4_tag.get("%track-number-total").unwrap().to_owned(),
            "trkn-t".to_string()
        );
        assert_eq!(mp4_tag.get("%to").unwrap().to_owned(), "trkn-t".to_string());
        assert_eq!(
            mp4_tag.get("%tnt").unwrap().to_owned(),
            "trkn-t".to_string()
        );
        assert_eq!(
            mp4_tag.get("%track-genre").unwrap().to_owned(),
            "©gen".to_string()
        );
        assert_eq!(mp4_tag.get("%tg").unwrap().to_owned(), "©gen".to_string());
        assert_eq!(
            mp4_tag.get("%track-composer").unwrap().to_owned(),
            "©wrt".to_string()
        );
        assert_eq!(mp4_tag.get("%tc").unwrap().to_owned(), "©wrt".to_string());
        assert_eq!(
            mp4_tag.get("%track-composer-sort").unwrap().to_owned(),
            "soco".to_string()
        );
        assert_eq!(mp4_tag.get("%tcs").unwrap().to_owned(), "soco".to_string());
        assert_eq!(
            mp4_tag.get("%track-date").unwrap().to_owned(),
            "©day".to_string()
        );
        assert_eq!(mp4_tag.get("%td").unwrap().to_owned(), "©day".to_string());
    }
}
