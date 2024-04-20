use id3::TagLike;
use metaflac::block;
use serde::Serialize;

use common::FileTypes;
use std::{error::Error, time::Duration};

macro_rules! mp3_tags {
    ($self:ident, $field:ident, $tags:ident, $fields:ident) => {
        let $field: Vec<String> = $tags
            .$fields()
            .unwrap_or_default()
            .iter()
            .map(|a| a.to_string())
            .collect();
        log::debug!("$field: {:?}", $field);
        if $field.is_empty() {
            $self.artist = None;
        } else {
            $self.$field = Some($field);
        }
    };
}

macro_rules! mp3_tag {
    ($self:ident, $to_field:ident, $tags:ident, $from_field:ident) => {
        let $to_field = $tags.$from_field().unwrap_or_default().to_string();
        log::debug!("$field: {:?}", $to_field);
        if $to_field.is_empty() {
            $self.artist = None;
        } else {
            $self.$to_field = Some(vec![$to_field]);
        }
    };
}

#[derive(Serialize, Default, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Track {
    /// Path to the audio file.
    pub path: Option<String>,

    /// album artist
    pub album_artist: Option<Vec<String>>,

    /// default name on which album artist is sorted. Example: Artist is "Alicia Keys", but artist_sort may be "Keys, Alicia".
    pub album_artist_sort: Option<Vec<String>>,

    /// Album title.
    pub album_title: Option<Vec<String>>,

    /// Album title sort.
    pub album_title_sort: Option<Vec<String>>,

    /// Disc number, usually 1.
    pub disc_number: Option<Vec<String>>,

    /// Total number of discs that comprise album, usually 1.
    pub disc_count: Option<Vec<String>>,

    /// Track artist.
    pub artist: Option<Vec<String>>,

    /// Track artist sort.
    pub artist_sort: Option<Vec<String>>,

    /// Track title.
    pub title: Option<Vec<String>>,

    /// Track title sort.
    pub title_sort: Option<Vec<String>>,

    /// Track number.
    pub number: Option<Vec<String>>,

    /// Total number of tracks.
    pub count: Option<Vec<String>>,

    /// Track's genre.
    pub genre: Option<Vec<String>>,

    /// Track's composer(s).
    pub composer: Option<Vec<String>>,

    /// Track's composer sort.
    pub composer_sort: Option<Vec<String>>,

    /// Track date(s).
    pub date: Option<Vec<String>>,

    /// Track comments.
    pub comments: Option<Vec<String>>,

    /// Duration.
    pub duration: Option<Duration>,

    /// Bitrate.
    pub bitrate: Option<u32>,

    /// Track bits per sample.
    pub bits_per_sample: Option<u8>,

    /// Track sample rate.
    pub sample_rate: Option<u32>,

    /// Track channels.
    pub channels: Option<u8>,

    /// Track replaygain.
    pub replaygain: Option<f32>,

    /// Track replaygain peak.
    pub replaygain_peak: Option<f32>,
}

impl Track {
    /// Creates a new, empty `Track` struct.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `Track` struct with a path to an audio file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the audio file.
    ///
    /// # Returns
    ///
    /// A `Track` struct with the path set.
    ///
    /// # Examples
    ///
    /// ```
    /// let track = Track::from_path("/path/to/audio.flac".to_string());
    /// ```
    #[allow(dead_code)]
    pub fn from_path(path: String) -> Self {
        Self {
            path: Some(path),
            ..Self::default()
        }
    }
}

/// Handles reading of various audio file formats.
pub trait Reader {
    fn read(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn read_flac(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn read_mp3(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;
}

impl Reader for Track {
    /// Reads an audio file. The function will determine the file format and call the appropriate
    /// detailed function to read the file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file type is not supported.
    ///
    fn read(&mut self) -> Result<(), Box<dyn Error>> {
        if self.path.is_none() {
            return Err("No path provided".into());
        }

        let file_type = FileTypes::from_filename(self.path.as_ref().unwrap_or(&String::new()));
        log::debug!("File type: {file_type}");
        match file_type {
            FileTypes::Flac => self.read_flac()?,
            FileTypes::MP3 => self.read_mp3()?,
            // FileTypes::Mp4 => self.read_mp4()?,
            // FileTypes::Ape => self.read_ape()?,
            // FileTypes::Dsf => self.read_dsf()?,
            _ => return Err("Unsupported file type".into()),
        }
        Ok(())
    }

    /// Builds a `Track` struct from a FLAC file.
    fn read_flac(&mut self) -> Result<(), Box<dyn Error>> {
        let tags = if self.path.is_some() {
            metaflac::Tag::read_from_path(self.path.as_ref().unwrap_or(&String::new()))?
        } else {
            return Err("No path provided".into());
        };

        for block in tags.blocks() {
            match block {
                block::Block::VorbisComment(vc) => {
                    let vcc = &vc.comments;
                    log::debug!("Vorbis comments: {vcc:?}");

                    // While there are native functions for some of these (e.g. vc.album_artist()),
                    // they don't return the values in the format expected, so they would need to be converted.
                    // It is just easier to do it this way. This may change in the future.
                    self.album_artist = vcc.get("ALBUMARTIST").cloned();
                    self.album_artist_sort = vcc.get("ALBUMARTISTSORT").cloned();
                    self.album_title = vcc.get("ALBUM").cloned();
                    self.album_title_sort = vcc.get("ALBUMSORT").cloned();
                    self.disc_number = vcc.get("DISCNUMBER").cloned();
                    self.disc_count = vcc.get("DISCTOTAL").cloned();
                    self.artist = vcc.get("ARTIST").cloned();
                    self.artist_sort = vcc.get("ARTISTSORT").cloned();
                    self.title = vcc.get("TITLE").cloned();
                    self.title_sort = vcc.get("TITLESORT").cloned();
                    self.number = vcc.get("TRACKNUMBER").cloned();
                    self.count = vcc.get("TRACKTOTAL").cloned();
                    self.genre = vcc.get("GENRE").cloned();
                    self.composer = vcc.get("COMPOSER").cloned();
                    self.composer_sort = vcc.get("COMPOSERSORT").cloned();
                    self.date = vcc.get("DATE").cloned();
                    self.comments = vcc.get("COMMENT").cloned();
                    log::debug!("Track after comments: {self:?}");
                }
                block::Block::StreamInfo(si) => {
                    log::debug!("StreamInfo: {si:?}");

                    self.duration = Some(duration_from_samples(si.total_samples, si.sample_rate));

                    self.bits_per_sample = Some(si.bits_per_sample);
                    self.channels = Some(si.num_channels);
                    self.sample_rate = Some(si.sample_rate);

                    log::debug!("Track after StreamInfo: {self:?}");
                }
                block::Block::Application(_) => {
                    log::trace!("Application block");
                }
                block::Block::CueSheet(_) => {
                    log::trace!("CueSheet block");
                }
                block::Block::Padding(_) => {
                    log::trace!("Padding block");
                }
                block::Block::Picture(_) => {
                    log::trace!("Picture block");
                }
                block::Block::SeekTable(_) => {
                    log::trace!("SeekTable block");
                }
                block::Block::Unknown(_) => {
                    log::trace!("Unknown block");
                }
            }
        }
        Ok(())
    }

    /// Builds a `Track` struct from an MP3 file.
    fn read_mp3(&mut self) -> Result<(), Box<dyn Error>> {
        let tags = if self.path.is_some() {
            id3::Tag::read_from_path(self.path.as_ref().unwrap_or(&String::new()))?
        } else {
            return Err("No path provided".into());
        };

        log::debug!("MP3 tags: {tags:?}");

        mp3_tags!(self, artist, tags, artists);
        mp3_tag!(self, album_title, tags, album);

        Ok(())
    }
}

/// Converts samples to a `Duration`.
///
/// # Arguments
///
/// * `samples` - Number of samples.
/// * `sample_rate` - Sample rate in Hz.
///
/// # Returns
///
/// A `Duration` representing the number of samples.
///
/// # Examples
///
/// ```
/// let duration = duration_from_samples(44100, 44100);
/// assert_eq!(duration, Duration::from_secs(1));
/// ```
///
#[allow(clippy::cast_precision_loss)]
fn duration_from_samples(samples: u64, sample_rate: u32) -> Duration {
    Duration::from_secs_f64(samples as f64 / f64::from(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let track = Track::new();

        assert_eq!(track.path, None);
        assert_eq!(track.album_artist, None);
        assert_eq!(track.album_artist_sort, None);
        assert_eq!(track.album_title, None);
        assert_eq!(track.album_title_sort, None);
        assert_eq!(track.disc_number, None);
        assert_eq!(track.disc_count, None);
        assert_eq!(track.artist, None);
        assert_eq!(track.artist_sort, None);
        assert_eq!(track.title, None);
        assert_eq!(track.title_sort, None);
        assert_eq!(track.number, None);
        assert_eq!(track.count, None);
        assert_eq!(track.genre, None);
        assert_eq!(track.composer, None);
        assert_eq!(track.composer_sort, None);
        assert_eq!(track.date, None);
        assert_eq!(track.comments, None);
    }

    #[test]
    fn test_from_path() {
        let path = String::from("/path/to/audio.flac");
        let track = Track::from_path(path.clone());

        assert_eq!(track.path, Some(path));
        assert_eq!(track.album_artist, None);
        // Add more assertions for other fields
    }

    #[test]
    fn test_read() {
        // FLAC
        let mut track = Track::from_path("../t_flac/CD 1 - Stuff/01-01 Slavonic Dances, Series II, Op 72 (B 147, 1886–87) No 7 in C major Presto.flac".to_string());
        track.read().expect("Uh oh...");

        assert_eq!(
            track.album_artist,
            Some(vec!["Various Artists".to_string()])
        );

        // MP3
    }

    #[test]
    fn test_read_flac_with_path() {
        let mut track = Track {
            path: Some("../t_flac/CD 1 - Stuff/01-01 Slavonic Dances, Series II, Op 72 (B 147, 1886–87) No 7 in C major Presto.flac".to_string()),
            ..Track::default()
        };

        assert!(track.read_flac().is_ok());

        // Assert that the track fields are populated correctly
        assert_eq!(
            track.album_artist,
            Some(vec!["Various Artists".to_string()])
        );
        assert_eq!(track.album_artist_sort, None);
        assert_eq!(
            track.album_title,
            Some(vec!["The Many Loves of Antonín Dvořák (CD1)".to_string()])
        );
        assert_eq!(track.album_title_sort, None);
        assert_eq!(track.disc_number, Some(vec!["01".to_string()]));
        assert_eq!(track.disc_count, Some(vec!["03".to_string()]));
        assert_eq!(
            track.artist,
            Some(vec!["Czech Philharmonic Orchestra, Karel Šejna".to_string()])
        );
        assert_eq!(track.artist_sort, None);
        assert_eq!(
            track.title,
            Some(vec![
                "Slavonic Dances, Series II, Op. 72 (B 147, 1886–87) No. 7 in C major. Presto"
                    .to_string()
            ])
        );
        assert_eq!(track.title_sort, None);
        assert_eq!(track.number, Some(vec!["1".to_string()]));
        assert_eq!(track.count, Some(vec!["05".to_string()]));
        assert_eq!(track.genre, Some(vec!["Classical".to_string()]));
        assert_eq!(track.composer, Some(vec!["Someone".to_string()]));
        assert_eq!(track.composer_sort, None);
        assert_eq!(track.date, Some(vec!["1959".to_string()]));
        assert_eq!(track.comments, Some(vec!["Recorded: 18th June 1959. The Dvořák Hall of Rudolfinum, Prague. First release: 1960".to_string()]));
    }

    #[test]
    fn test_read_flac_without_path() {
        let mut track = Track::default();

        assert!(track.read_flac().is_err());
    }
}
