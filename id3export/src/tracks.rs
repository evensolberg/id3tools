use common::{need_split, FileTypes};
use id3::{Tag, TagLike};
use metaflac::block;
use mp4ameta::{Data, Fourcc, Tag as Mp4Tag};
use serde::Serialize;
use std::error::Error;

macro_rules! mp3_tags {
    ($tags:ident, $field:ident, $self_ref:ident, $self_field:ident) => {
        if let Some(field) = $tags.$field() {
            $self_ref.$self_field = Some(field.join("; "));
        }
    };
}

macro_rules! mp3_tag {
    ($tags:ident, $field:ident, $self_ref:ident, $self_field:ident) => {
        if let Some(field) = $tags.$field() {
            $self_ref.$self_field = Some(field.to_string());
        }
    };
    ($tags:ident, $field:literal, $self_ref:ident, $self_field:ident) => {
        if let Some(field) = $tags.get($field) {
            $self_ref.$self_field = Some(field.to_string());
        }
    };
}

macro_rules! mp3_tag_string {
    ($tags:ident, $field:ident, $self_ref:ident, $self_field:ident) => {
        if let Some(field) = $tags.$field() {
            $self_ref.$self_field = Some(field.to_string());
        }
    };
}

macro_rules! ape_tags {
    ($tags:ident, $items:literal, $self_ref:ident, $self_field:ident) => {
        let field = $tags.items($items);
        if !field.is_empty() {
            // Collect the values into a new vector and then flatten the vector.
            let mut values = Vec::new();
            for item in field {
                match &item.value {
                    ape::ItemValue::Text(value) => {
                        values.push(value.to_string());
                    }
                    _ => {
                        log::debug!("Unexpected item type for {}: {item:?}", $items);
                    }
                }
            }
            $self_ref.$self_field = flatten_vec(values);
        }
    };
}

macro_rules! mp4_tags {
    ($tags:ident, $binding:literal, $self_ref:ident, $self_field:ident) => {
        let binding = Fourcc(*$binding);
        let field = $tags.data_of(&binding);
        let mut gather = Vec::new();
        for value in field {
            match value {
                Data::Utf8(s) => gather.push(s.to_string()),
                Data::Utf16(s) => gather.push(s.to_string()),
                _ => (),
            }
        }
        $self_ref.$self_field = Some(gather.join("; "));
    };
}

/// A struct to hold track information.
#[derive(Serialize, Default, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Track {
    /// Path to the audio file.
    pub path: Option<String>,

    /// File format.
    pub file_format: Option<FileTypes>,

    /// File size in bytes
    pub file_size: Option<u64>,

    /// album artist
    pub album_artist: Option<String>,

    /// default name on which album artist is sorted. Example: Artist is "Alicia Keys", but artist_sort may be "Keys, Alicia".
    pub album_artist_sort: Option<String>,

    /// Album title.
    pub album_title: Option<String>,

    /// Album title sort.
    pub album_title_sort: Option<String>,

    /// Disc number, usually 1.
    pub disc_number: Option<String>,

    /// Total number of discs that comprise album, usually 1.
    pub disc_count: Option<String>,

    /// Track artist.
    pub artist: Option<String>,

    /// Track artist sort.
    pub artist_sort: Option<String>,

    /// Track title.
    pub title: Option<String>,

    /// Track title sort.
    pub title_sort: Option<String>,

    /// Track number.
    pub track_number: Option<String>,

    /// Total number of tracks.
    pub track_count: Option<String>,

    /// Track's genre.
    pub genre: Option<String>,

    /// Track's composer(s).
    pub composer: Option<String>,

    /// Track's composer sort.
    pub composer_sort: Option<String>,

    /// Track date(s).
    pub date: Option<String>,

    /// Track comments.
    pub comments: Option<String>,

    /// Duration in milliseconds
    pub duration_ms: Option<u64>,

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

    /// Track MD5 sum.
    pub md5: Option<String>,
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

    // May move the following functions out of the trait so the trait stays simple.

    fn read_flac(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn read_mp3(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn read_mp4(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn read_ape(&mut self) -> Result<(), Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn read_dsf(&mut self) -> Result<(), Box<dyn Error>>
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

        let metadata = std::fs::metadata(self.path.as_ref().unwrap_or(&String::new()));
        self.file_size = Some(metadata.unwrap().len());

        let file_type = FileTypes::from_filename(self.path.as_ref().unwrap_or(&String::new()));
        log::debug!("File type: {file_type}");
        match file_type {
            FileTypes::Flac => self.read_flac()?,
            FileTypes::MP3 => self.read_mp3()?,
            FileTypes::M4A => self.read_mp4()?,
            FileTypes::Ape => self.read_ape()?,
            FileTypes::Dsf => self.read_dsf()?,
            _ => {
                self.file_format = Some(FileTypes::Unknown);
            }
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

        self.file_format = Some(FileTypes::Flac);

        for block in tags.blocks() {
            match block {
                block::Block::VorbisComment(vc) => {
                    let vcc = &vc.comments;
                    log::debug!("Vorbis comments: {vcc:?}");

                    // While there are native functions for some of these (e.g. vc.album_artist()), they
                    // don't return the values in the format expected, so the values would need to be converted.
                    // It is just easier to do it this way. This may change in the future.
                    self.album_artist =
                        flatten_vec(vcc.get("ALBUMARTIST").cloned().unwrap_or_default());
                    self.album_artist_sort =
                        flatten_vec(vcc.get("ALBUMARTISTSORT").cloned().unwrap_or_default());
                    self.album_title = flatten_vec(vcc.get("ALBUM").cloned().unwrap_or_default());
                    self.album_title_sort =
                        flatten_vec(vcc.get("ALBUMSORT").cloned().unwrap_or_default());
                    self.disc_number =
                        flatten_vec(vcc.get("DISCNUMBER").cloned().unwrap_or_default());
                    self.disc_count =
                        flatten_vec(vcc.get("DISCTOTAL").cloned().unwrap_or_default());
                    self.artist = flatten_vec(vcc.get("ARTIST").cloned().unwrap_or_default());
                    self.artist_sort =
                        flatten_vec(vcc.get("ARTISTSORT").cloned().unwrap_or_default());
                    self.title = flatten_vec(vcc.get("TITLE").cloned().unwrap_or_default());
                    self.title_sort =
                        flatten_vec(vcc.get("TITLESORT").cloned().unwrap_or_default());
                    self.track_number =
                        flatten_vec(vcc.get("TRACKNUMBER").cloned().unwrap_or_default());
                    self.track_count =
                        flatten_vec(vcc.get("TRACKTOTAL").cloned().unwrap_or_default());
                    self.genre = flatten_vec(vcc.get("GENRE").cloned().unwrap_or_default());
                    self.composer = flatten_vec(vcc.get("COMPOSER").cloned().unwrap_or_default());
                    self.composer_sort =
                        flatten_vec(vcc.get("COMPOSERSORT").cloned().unwrap_or_default());
                    self.date = flatten_vec(vcc.get("DATE").cloned().unwrap_or_default());
                    self.comments = flatten_vec(vcc.get("COMMENT").cloned().unwrap_or_default());
                    log::debug!("Track after comments: {self:?}");
                }
                block::Block::StreamInfo(si) => {
                    log::debug!("StreamInfo: {si:?}");

                    self.duration_ms =
                        Some(duration_from_samples(si.total_samples, si.sample_rate));

                    self.bits_per_sample = Some(si.bits_per_sample);
                    self.channels = Some(si.num_channels);
                    self.sample_rate = Some(si.sample_rate);

                    // For some reason, doing a straight "String::from_utf8(si.md5.clone())?"
                    // results in "invalid utf-8 sequence of 1 bytes from index 0" in my tests.
                    // This is a workaround. ¯\_(ツ)_/¯
                    if let Err(e) = utf8_to_string(&si.md5) {
                        log::error!("Error converting MD5: {e}");
                        self.md5 = None;
                    } else {
                        let md5 = utf8_to_string(&si.md5)?;
                        if md5 == "00000000000000000000000000000000" {
                            self.md5 = None;
                        } else {
                            self.md5 = Some(md5);
                        }
                        log::debug!("MD5: {:?}", self.md5);
                    }

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
        let meta = match mp3_metadata::read_from_file(self.path.as_ref().unwrap_or(&String::new()))
        {
            Ok(m) => m,
            Err(e) => {
                let msg = format!("{:?}", e);
                log::error!("Error reading MP3: {msg}");
                return Err(msg.into());
            }
        };

        self.file_format = Some(FileTypes::MP3);
        self.duration_ms = Some(meta.duration.as_millis() as u64);

        let frames = meta.frames;

        if frames.is_empty() {
            return Err("No frames found".into());
        }

        self.bitrate = Some(frames[0].bitrate as u32);
        self.channels = if frames[0].ms_stereo {
            Some(2)
        } else {
            Some(1)
        };

        self.bits_per_sample = Some(16); // MP3 is always 16-bit

        // This should probably be something like the mode of the sample rates, but this is fine for now.
        self.sample_rate = Some(frames[0].sampling_freq as u32);

        // Use a different crate to get the metadata
        let tag = Tag::read_from_path(self.path.as_ref().unwrap_or(&String::new()))?;
        mp3_tag!(tag, "TPE2", self, album_artist);
        mp3_tag!(tag, "TSO2", self, album_artist_sort);
        mp3_tag!(tag, album, self, album_title);
        mp3_tag!(tag, "TSOA", self, album_title_sort);
        mp3_tag_string!(tag, disc, self, disc_number);
        mp3_tag_string!(tag, total_discs, self, disc_count);
        mp3_tags!(tag, artists, self, artist);
        mp3_tag!(tag, "TSOP", self, artist_sort);
        mp3_tag!(tag, title, self, title);
        mp3_tag!(tag, "TSOT", self, title_sort);
        mp3_tag_string!(tag, track, self, track_number);
        mp3_tag_string!(tag, total_tracks, self, track_count);
        mp3_tags!(tag, genres, self, genre);
        mp3_tag!(tag, "TCOM", self, composer);
        mp3_tag!(tag, "TSOC", self, composer_sort);
        mp3_tag!(tag, "TDRL", self, date);
        mp3_tag!(tag, "COMM", self, comments);

        Ok(())
    }

    /// Builds a `Track` struct from an MP4 file.
    fn read_mp4(&mut self) -> Result<(), Box<dyn Error>> {
        let tags = Mp4Tag::read_from_path(self.path.as_ref().unwrap_or(&String::new()))?;
        let audio = tags.audio_info();

        self.file_format = Some(FileTypes::M4A);

        self.duration_ms = Some(audio.duration.unwrap_or_default().as_millis() as u64);
        self.channels = Some(
            audio
                .channel_config
                .unwrap_or(mp4ameta::ChannelConfig::Stereo)
                .channel_count(),
        );
        self.bits_per_sample = Some(16);
        self.sample_rate = Some(
            audio
                .sample_rate
                .unwrap_or(mp4ameta::SampleRate::Hz44100)
                .hz(),
        );
        self.bitrate = Some(audio.avg_bitrate.unwrap_or(0));

        // Gather all the Utf8 and Utf16 into a single string.
        mp4_tags!(tags, b"aART", self, album_artist);
        mp4_tags!(tags, b"soaa", self, album_artist_sort);
        mp4_tags!(tags, b"\xa9alb", self, album_title);
        mp4_tags!(tags, b"soal", self, album_title_sort);
        mp4_tags!(tags, b"\xa9ART", self, artist);
        mp4_tags!(tags, b"soar", self, artist_sort);
        mp4_tags!(tags, b"\xa9nam", self, title);
        mp4_tags!(tags, b"sonm", self, title_sort);
        mp4_tags!(tags, b"\xa9gen", self, genre);
        mp4_tags!(tags, b"\xa9wrt", self, composer);
        mp4_tags!(tags, b"soco", self, composer_sort);
        mp4_tags!(tags, b"\xa9day", self, date);
        mp4_tags!(tags, b"\xa9cmt", self, comments);

        let default_value = 0;
        self.track_number = Some(tags.track_number().unwrap_or(default_value).to_string());
        self.track_count = Some(tags.total_tracks().unwrap_or(default_value).to_string());
        self.disc_number = Some(tags.disc_number().unwrap_or(default_value).to_string());
        self.disc_count = Some(tags.total_discs().unwrap_or(default_value).to_string());
        Ok(())
    }

    /// Reads an APE file. Unfortunately, the `ape` crate currently does not provide a way to read the file's duration, bitrate, etc.
    fn read_ape(&mut self) -> Result<(), Box<dyn Error>> {
        let tags = ape::read_from_path(self.path.as_ref().unwrap_or(&String::new()))?;
        log::debug!("APE tags: {tags:?}");

        self.file_format = Some(FileTypes::Ape);

        ape_tags!(tags, "ALBUMARTIST", self, album_artist);
        ape_tags!(tags, "ALBUMARTISTSORT", self, album_artist_sort);
        ape_tags!(tags, "ALBUM", self, album_title);
        ape_tags!(tags, "ALBUMSORT", self, album_title_sort);
        ape_tags!(tags, "DISCNUMBER", self, disc_number);
        ape_tags!(tags, "DISCTOTAL", self, disc_count);
        ape_tags!(tags, "ARTIST", self, artist);
        ape_tags!(tags, "ARTISTSORT", self, artist_sort);
        ape_tags!(tags, "TITLE", self, title);
        ape_tags!(tags, "TITLESORT", self, title_sort);
        ape_tags!(tags, "TRACKNUMBER", self, track_number);
        ape_tags!(tags, "TRACKTOTAL", self, track_count);
        ape_tags!(tags, "GENRE", self, genre);
        ape_tags!(tags, "COMPOSER", self, composer);
        ape_tags!(tags, "COMPOSERSORT", self, composer_sort);
        ape_tags!(tags, "DATE", self, date);
        ape_tags!(tags, "COMMENT", self, comments);
        Ok(())
    }

    fn read_dsf(&mut self) -> Result<(), Box<dyn Error>> {
        let newpath = String::new();
        let filepath = std::path::Path::new(self.path.as_ref().unwrap_or(&newpath));
        let dsf_file = dsf::DsfFile::open(filepath)?;
        log::debug!("DSF file metadata: {dsf_file}");

        self.file_format = Some(FileTypes::Dsf);

        // Get the basic file information first.
        let format = dsf_file.fmt_chunk();
        self.sample_rate = Some(format.sampling_frequency());
        self.channels = Some(format.channel_num() as u8);
        self.bits_per_sample = Some(format.bits_per_sample() as u8);
        self.duration_ms = Some(duration_from_samples(
            format.sample_count(),
            format.sampling_frequency(),
        ));

        // Get the ID3 tag.
        let tag = if dsf_file.id3_tag().is_some() {
            <std::option::Option<id3::Tag> as Clone>::clone(dsf_file.id3_tag()).unwrap()
        } else {
            log::warn!("No ID3 tag found");
            return Err("No ID3 tag found".into());
        };

        log::debug!("Tag: {tag:?}");
        for frame in tag.frames() {
            log::debug!("  {} = {}", frame.id(), frame.content());
        }

        tag.frames().for_each(|frame| {
            log::debug!("Frame: {frame:?}",);
            match frame.id() {
                "TPE2" => self.album_artist = Some(frame.content().to_string()),
                "TSO2" => self.album_artist_sort = Some(frame.content().to_string()),
                "TALB" => self.album_title = Some(frame.content().to_string()),
                "TSOA" => self.album_title_sort = Some(frame.content().to_string()),
                "TPOS" => {
                    let pos = frame.content().to_string();
                    if need_split(&pos) {
                        if let Ok((disc_num, disc_count)) = common::split_val(&pos) {
                            self.disc_number = Some(disc_num.to_string());
                            self.disc_count = Some(disc_count.to_string());
                        } else {
                            self.disc_number = Some(pos);
                        }
                    } else {
                        self.disc_number = Some(pos);
                    }
                }
                "TPOS-T" => self.disc_count = Some(frame.content().to_string()),
                "TPE1" => self.artist = Some(frame.content().to_string()),
                "TSOP" => self.artist_sort = Some(frame.content().to_string()),
                "TIT2" => self.title = Some(frame.content().to_string()),
                "TSOT" => self.title_sort = Some(frame.content().to_string()),
                "TRCK" => {
                    let pos = frame.content().to_string();
                    if need_split(&pos) {
                        if let Ok((track_num, track_count)) = common::split_val(&pos) {
                            self.track_number = Some(track_num.to_string());
                            self.track_count = Some(track_count.to_string());
                        } else {
                            self.track_number = Some(pos);
                        }
                    } else {
                        self.track_number = Some(pos);
                    }
                }
                "TRCK-T" => self.track_count = Some(frame.content().to_string()),
                "TCON" => self.genre = Some(frame.content().to_string()),
                "TCOM" => self.composer = Some(frame.content().to_string()),
                "TSOC" => self.composer_sort = Some(frame.content().to_string()),
                "TDRC" => self.date = Some(frame.content().to_string()),
                "COMM" => self.comments = Some(frame.content().to_string()),
                _ => log::debug!("Unknown frame: {}", frame.id()),
            }
        });

        Ok(())
    }
}

/// Converts samples to a duration in milliseconds using the sample rate.
///
/// # Arguments
///
/// * `samples` - Number of samples.
/// * `sample_rate` - Sample rate in Hz.
///
/// # Returns
///
/// An u64 representing the duration of the track in milliseconds.
///
///
#[allow(clippy::cast_precision_loss)]
fn duration_from_samples(samples: u64, sample_rate: u32) -> u64 {
    ((samples as f64 / f64::from(sample_rate)) * 1000.0).trunc() as u64
}

/// Converts an utf8 hex string to a `String`.
///
/// # Arguments
///
/// * `utf8` - A slice of bytes representing an utf8 hex string.
///
/// # Returns
///
/// A `Result` containing the `String` representation of the utf8 hex string.
///
/// # Examples
///
/// ```
/// let utf8 = b"48656c6c6f2c20576f726c6421";
/// let string = utf8_to_string(utf8).unwrap();
/// assert_eq!(string, "Hello, World!");
/// ```
///
/// # Errors
///
/// Returns an error if the utf8 hex string is invalid.
fn utf8_to_string(utf8: &[u8]) -> Result<String, Box<dyn Error>> {
    log::debug!("UTF8: {utf8:?}");

    let hex: String = utf8
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("");

    log::debug!("utf8 to string: {hex:?}");

    Ok(hex)
}

/// Flatten a `Vec<String>` into a single `String`.
/// The strings are separated by a semicolon and a space.
///
/// # Arguments
///
/// * `vec` - A `Vec<String>` to flatten.
///
/// # Returns
///
/// A `String` with the contents of the `Vec<String>`.
///
/// # Examples
///
/// ```
/// let vec = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
/// let string = flatten_vec(vec);
/// assert_eq!(string, "One; Two; Three");
/// ```
///
/// # Notes
///
/// The function trims the resulting string.
fn flatten_vec(vec: Vec<String>) -> Option<String> {
    if vec.is_empty() {
        return None;
    }

    Some(vec.join("; "))
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
        assert_eq!(track.track_number, None);
        assert_eq!(track.track_count, None);
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

        assert_eq!(track.album_artist, Some("Various Artists".to_string()));

        // MP3
    }

    #[test]
    fn test_read_flac_with_path() {
        let mut track = Track {
            path: Some("../t_flac/CD 1 - Stuff/01-01 Slavonic Dances, Series II, Op 72 (B 147, 1886–87) No 7 in C major Presto.flac".to_string()),
            ..Track::default()
        };

        if let Err(err) = track.read_flac() {
            panic!("Error reading FLAC: {err}");
        }

        // Assert that the track fields are populated correctly
        assert_eq!(track.album_artist, Some("Various Artists".to_string()));
        assert_eq!(track.album_artist_sort, None);
        assert_eq!(
            track.album_title,
            Some("The Many Loves of Antonín Dvořák (CD1)".to_string())
        );
        assert_eq!(track.album_title_sort, None);
        assert_eq!(track.disc_number, Some("01".to_string()));
        assert_eq!(track.disc_count, Some("03".to_string()));
        assert_eq!(
            track.artist,
            Some("Czech Philharmonic Orchestra, Karel Šejna".to_string())
        );
        assert_eq!(track.artist_sort, None);
        assert_eq!(
            track.title,
            Some(
                "Slavonic Dances, Series II, Op. 72 (B 147, 1886–87) No. 7 in C major. Presto"
                    .to_string()
            )
        );
        assert_eq!(track.title_sort, None);
        assert_eq!(track.track_number, Some("1".to_string()));
        assert_eq!(track.track_count, Some("05".to_string()));
        assert_eq!(track.genre, Some("Classical".to_string()));
        assert_eq!(track.composer, Some("Someone".to_string()));
        assert_eq!(track.composer_sort, None);
        assert_eq!(track.date, Some("1959".to_string()));
        assert_eq!(track.comments, Some("Recorded: 18th June 1959. The Dvořák Hall of Rudolfinum, Prague. First release: 1960".to_string()));
    }

    #[test]
    fn test_read_flac_without_path() {
        let mut track = Track::default();

        assert!(track.read_flac().is_err());
    }
}
