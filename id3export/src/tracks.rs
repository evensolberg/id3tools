use serde::Serialize;

#[derive(Serialize, Default, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Track {
    /// album artist
    pub album_artist: Option<String>,

    /// default name on which album artist is sorted. Example: Artist is "Alicia Keys", but artist_sort may be "Keys, Alicia".
    pub album_artist_sort: Option<String>,

    /// Album title.
    pub album_title: Option<String>,

    /// Album title sort.
    pub album_title_sort: Option<String>,

    /// Disc number, usually 1.
    pub disc_number: Option<u16>,

    /// Total number of discs that comprise album, usually 1.
    pub disc_number_total: Option<u16>,

    /// Track artist.
    pub track_artist: Option<String>,

    /// Track artist sort.
    pub track_artist_sort: Option<String>,

    /// Track title.
    pub track_title: Option<String>,

    /// Track title sort.
    pub track_title_sort: Option<String>,

    /// Track number.
    pub track_number: Option<u16>,

    /// Total number of tracks.
    pub track_number_total: Option<u16>,

    /// Track's genre.
    pub track_genre: Option<String>,

    /// Track's composer(s).
    pub track_composer: Option<String>,

    /// Track's composer(s).
    pub track_composer_sort: Option<String>,

    /// Track composer(s).
    pub track_date: Option<String>,

    /// Track comments.
    pub track_comments: Option<String>,
}

// impl Track {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }
