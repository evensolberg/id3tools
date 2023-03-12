use id3::frame;
use id3::Content;
use id3::Tag;
use mp3_metadata::MP3Metadata;
use std::error::Error;

/// Look into an `Option<String>` and output the `String` if there is one.
macro_rules! opt {
    ($var:ident, $field:ident, $title:literal) => {
        if let Some(f) = &$var.$field {
            log::info!($title, f);
        }
    };
}

/// Output the content of a `Vec<String>` as a comma-separated list.
macro_rules! string_vec {
    ($var:ident, $field:ident, $title:literal) => {
        if !$var.$field.is_empty() {
            let concated = if $var.$field.len() == 1 {
                $var.$field[0].clone()
            } else {
                $var.$field
                    .iter()
                    .fold(", ".to_string(), |i, j| (i.to_string() + &*j))
            };
            log::info!($title, concated);
        }
    };
}

/// Output the content of a `Vec<String>` as a comma-separated list.
macro_rules! url_vec {
    ($var:ident, $field:ident, $title:literal) => {
        if !$var.$field.is_empty() {
            let concated = if $var.$field.len() == 1 {
                format!("{:?}", $var.$field[0])
            } else {
                $var.$field
                    .iter()
                    .fold(", ".to_string(), |i, j| i + &url_to_string(&*j))
            };
            log::info!($title, concated);
        }
    };
}

/// Output the content of a `Vec<Genre>` as a comma-separated list.
macro_rules! genre_vec {
    ($var:ident, $field:ident, $title:literal) => {
        if !$var.$field.is_empty() {
            let concated = if $var.$field.len() == 1 {
                genre(&$var.$field[0])
            } else {
                $var.$field
                    .iter()
                    .fold(", ".to_string(), |i, j| (i + &genre(&*j)))
            };
            log::info!($title, concated);
        }
    };
}

/// Performs the actual processing of MP3 files.
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    let meta = open_mp3(filename)?;
    if show_detail {
        show_optional_audio_tags(&meta);
        show_frame_data(&meta);
    }

    // Read the tag - bomb out if it doesn't work.
    let tag = Tag::read_from_path(filename)?;

    for item in tag.frames() {
        match item.content() {
            Content::Text(t) => {
                log::info!("  {} = {} (Text)", item.id(), t);
            }
            Content::ExtendedText(et) => {
                log::info!("  {} = {} (Extended Text)", item.id(), et);
            }
            Content::Link(l) => {
                log::info!("  {} = {} (Link)", item.id(), l);
            }
            Content::ExtendedLink(el) => {
                log::info!("  {} = {} (Extended Link)", item.id(), el);
            }
            Content::Comment(co) => {
                log::info!("  {} = {} (Comment)", item.id(), co);
            }
            Content::Popularimeter(pm) => {
                if show_detail {
                    show_popularimeter(pm);
                }
            }
            Content::Lyrics(l) => {
                if show_detail {
                    show_lyrics(l);
                }
            }
            Content::SynchronisedLyrics(sl) => {
                if show_detail {
                    show_synchronised_lyrics(sl);
                }
            }
            Content::Picture(p) => {
                if show_detail {
                    show_picture(p);
                }
            }
            Content::EncapsulatedObject(eo) => {
                if show_detail {
                    show_encapsulated_object(eo);
                }
            }
            Content::Chapter(c) => {
                if show_detail {
                    show_chapter(c);
                }
            }
            Content::MpegLocationLookupTable(mllt) => {
                if show_detail {
                    show_mpeg_location_lookup_table(mllt);
                }
            }
            Content::Unknown(uk) => {
                if show_detail {
                    show_unknown(uk);
                }
            }
            _ => {
                return Err(format!("Unknown content type in file {filename}").into());
            }
        }
    }

    // return safely
    Ok(())
}

/// Show the `frame::Popularimeter` fields
fn show_popularimeter(pm: &frame::Popularimeter) {
    log::info!("  Popularimeter: {}", pm);
    log::info!("    User: {}", pm.user);
    log::info!("    Rating: {}", pm.rating);
    log::info!("    Counter: {}", pm.counter);
}

/// Show the `frome::Lyrics` fields
fn show_lyrics(l: &frame::Lyrics) {
    log::info!("  Lyrics:");
    log::info!("    Language: {}", l.lang);
    log::info!("    Description: {}", l.description);
    log::info!("    Text: {}", l.text);
}

/// Show the `frame::Picture` fields
fn show_picture(p: &frame::Picture) {
    log::info!("  Picture:");
    log::info!("    Mime Type: {}", p.mime_type);
    log::info!("    Picture Type: {:?}", p.picture_type);
    log::info!("    Description: {}", p.description);
    log::info!("    Picture Data: {} bytes", p.data.len());
}

/// Show the `frame::SynchronisedLyrics` fields
fn show_synchronised_lyrics(sl: &frame::SynchronisedLyrics) {
    log::info!("  Synchronised Lyrics:");
    log::info!("    Language: {}", sl.lang);
    log::info!("    Timestamp Format: {:?}", sl.timestamp_format);
    log::info!("    Content Type: {:?}", sl.content_type);
    log::info!("    Description: {}", sl.description);
    log::info!("    Content:");
    for (line_num, text) in &sl.content {
        log::info!("      {}: {}", *line_num, text);
    }
}

/// Show the `frame::EncapsulatedObject` fields
fn show_encapsulated_object(eo: &frame::EncapsulatedObject) {
    log::info!("  Encapsulated Object:");
    log::info!("    Mime Type: {}", eo.mime_type);
    log::info!("    Filename: {}", eo.filename);
    log::info!("    Description: {}", eo.description);
    log::info!("    Object Data: {} bytes", eo.data.len());
}

/// Show the `frame::Chapter` fields
fn show_chapter(c: &frame::Chapter) {
    log::info!("  Chapter:");
    log::info!("    Element ID: {}", c.element_id);
    log::info!("    Start Time: {}", c.start_time);
    log::info!("    End Time: {}", c.end_time);
    log::info!("    Start Offset: {}", c.start_offset);
    log::info!("    End Offset: {}", c.end_offset);
    log::info!("    Frame Count: {}", c.frames.len());
}

/// Show the `frame::MpegLocationLookupTable` fields
fn show_mpeg_location_lookup_table(mllt: &frame::MpegLocationLookupTable) {
    log::info!("  MPEG Location Lookup Table:");
    log::info!(
        "    Frames Between Reference: {}",
        mllt.frames_between_reference
    );
    log::info!(
        "    Bytes Between Reference: {}",
        mllt.bytes_between_reference
    );
    log::info!(
        "    Millis Between Reference: {}",
        mllt.millis_between_reference
    );
    log::info!("    Bits for Bytes: {}", mllt.bits_for_bytes);
    log::info!("    Bits for Millis: {}", mllt.bits_for_millis);
    log::info!("    References Count: {}", mllt.references.len());
}

/// Show the `Content::Unknown` fields
fn show_unknown(uk: &frame::Unknown) {
    log::info!("  Unknown:");
    log::info!("    Version: {}", uk.version);
    log::info!("    Data: {} bytes", uk.data.len());
}

/// Show the MP3 Frame information
fn show_frame_data(meta: &MP3Metadata) {
    log::info!("Frame information:");
    log::info!("  Duration: {:.1} seconds", meta.duration.as_secs_f64());
    log::info!("  Number of Frames: {}", meta.frames.len());

    let f = &meta.frames[0];
    log::info!("  Frame #0:");
    log::info!("    Size: {}", f.size);
    log::info!("    Version: {}", mp3_version(f.version));
    log::info!("    Layer: {}", mp3_layer(f.layer));
    log::info!("    CRC Info: {}", mp3_crc(f.crc));
    log::info!("    Bitrate: {} kb/sec", f.bitrate);
    log::info!("    Sampling frequency: {} hz", f.sampling_freq);
    log::info!("    Padding: {}", f.padding);
    log::info!("    Private bit: {}", f.private_bit);
    log::info!("    Channel type: {}", mp3_channeltype(f.chan_type));
    log::info!("    Intensity stereo: {}", f.intensity_stereo);
    log::info!("    MS Stereo: {}", f.ms_stereo);
    log::info!("    Copyright: {}", mp3_copyright(f.copyright));
    log::info!("    Status: {}", mp3_status(f.status));
    log::info!("    Emphasis: {}", mp3_emphasis(f.emphasis));
    if let Some(dur) = f.duration {
        log::info!("    Duration: {:.1}", dur.as_secs_f64());
    }
    log::info!("    Position: {:.1}", f.position.as_secs_f64());
    log::info!("    Offset: {}", f.offset);
}

/// Show optional audio tags
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn show_optional_audio_tags(m: &MP3Metadata) {
    let t = &m.optional_info;
    log::info!("  Optional Audio Tags:");
    log::info!("    Number of entries: {}", t.len());
    let mut en = 0;
    for e in &m.optional_info {
        en += 1;
        log::info!("    Entry #{en}");
        log::info!("      Position: {}", e.position);
        log::info!("      Version: {}.{}", e.major_version, e.minor_version);
        opt!(e, album_movie_show, "      Album/movie/show title: {}");
        opt!(e, bpm, "      Beats per Minute: {}");
        string_vec!(e, composers, "      Composers: {}");
        genre_vec!(e, content_type, "      Content type: {}");
        opt!(e, copyright, "     Copyright: {}");
        opt!(e, date, "      Date: {}");
        opt!(e, playlist_delay, "      Playlist delay: {} ms");
        opt!(e, encoded_by, "      Encoded by: {}");
        string_vec!(e, text_writers, "      Lyricist(s)/text writer(s): {}");
        opt!(e, file_type, "      File type: {}");
        opt!(e, time, "      Time (HHMM): {}");
        opt!(
            e,
            content_group_description,
            "      Content group description: {}"
        );
        opt!(
            e,
            subtitle_refinement_description,
            "      Subtitle/Description refinement: {}"
        );
        opt!(e, title, "      Title/Song name/Content description: {}");
        opt!(e, initial_key, "      Initial key: {}");
        opt!(e, language, "      Language: {}");
        opt!(e, length, "      Length: {} ms");
        opt!(e, media_type, "      Media type: {}");
        opt!(
            e,
            original_album_move_show_title,
            "      Original album/movie/show title: {}"
        );
        opt!(e, original_filename, "      Original file name: {}");
        string_vec!(
            e,
            original_text_writers,
            "      Original lyricist(s)/text writer(s): {}"
        );
        string_vec!(
            e,
            original_artists,
            "      Original artist(s)/performer(s): {}"
        );
        opt!(e, original_release_year, "      Original release year: {}");
        opt!(e, file_owner, "      File owner: {}");
        string_vec!(e, performers, "      Performer(s): {}");
        opt!(e, band, "      Band: {}");
        opt!(e, conductor, "      Conductor: {}");
        opt!(e, interpreted, "      Interpreted/Remixed by: {}");
        opt!(e, part_of_a_set, "      Part of Set: {}");
        opt!(e, publisher, "      Publisher: {}");
        opt!(e, track_number, "      Track number/Position in Set: {}");
        opt!(e, recording_dates, "      Recording dates: {}");
        opt!(
            e,
            internet_radio_station_name,
            "      Internet radio station name: {}"
        );
        opt!(
            e,
            internet_radio_station_owner,
            "      INternet radio station owner: {}"
        );
        opt!(e, size, "      Size: {} bytes (excluding ID3v2 tag)");
        opt!(
            e,
            international_standard_recording_code,
            "      International Standard Recording Code (ISRC): {}"
        );
        opt!(
            e,
            soft_hard_setting,
            "      Software/Hardware encoding settings: {}"
        );
        opt!(e, year, "      Recording year: {}");
        opt!(e, involved_people, "      Involved people list: {}");
        url_vec!(e, commercial_info_url, "      Commercial info URLs: {}");
        opt!(e, copyright_info_url, "      Copyright info URLs: {:?}");
        opt!(
            e,
            official_webpage,
            "      Official audio file webpage: {:?}"
        );
        url_vec!(
            e,
            official_artist_webpage,
            "      Official artist/performer webpage(s): {}"
        );
        opt!(
            e,
            official_audio_source_webpage,
            "      Official audio source webpage: {:?}"
        );
        opt!(
            e,
            official_internet_radio_webpage,
            "      Official internet radio webpage: {:?}"
        );
        opt!(e, payment_url, "      Payment URL: {:?}");
        opt!(
            e,
            publishers_official_webpage,
            "      Publisher's official webpage: {:?}"
        );
    }
}

/// Return the `String` version of the MP3 `Version` enum
fn mp3_version(v: mp3_metadata::Version) -> String {
    match v {
        mp3_metadata::Version::MPEG1 => String::from("MPEG1"),
        mp3_metadata::Version::MPEG2 => String::from("MPEG2"),
        mp3_metadata::Version::MPEG2_5 => String::from("MPEG2.5"),
        mp3_metadata::Version::Reserved => String::from("Reserved"),
        mp3_metadata::Version::Unknown => String::from("Unknown"),
    }
}

/// Return the `String` version of the MP3 `Layer` enum
fn mp3_layer(l: mp3_metadata::Layer) -> String {
    match l {
        mp3_metadata::Layer::Layer1 => String::from("Layer1"),
        mp3_metadata::Layer::Layer2 => String::from("Layer2"),
        mp3_metadata::Layer::Layer3 => String::from("Layer3"),
        mp3_metadata::Layer::Reserved => String::from("Reserved"),
        mp3_metadata::Layer::Unknown => String::from("Unknown"),
    }
}

/// Return the `String` version of the MP3 `CRC` enum
fn mp3_crc(c: mp3_metadata::CRC) -> String {
    match c {
        mp3_metadata::CRC::Added => String::from("Added"),
        mp3_metadata::CRC::NotAdded => String::from("Not Ndded"),
    }
}

/// Return the `String` version of the MP3 `ChannelType` enum
fn mp3_channeltype(t: mp3_metadata::ChannelType) -> String {
    match t {
        mp3_metadata::ChannelType::Stereo => String::from("Stereo"),
        mp3_metadata::ChannelType::JointStereo => String::from("Joint Stereo"),
        mp3_metadata::ChannelType::DualChannel => String::from("Dual Channel"),
        mp3_metadata::ChannelType::SingleChannel => String::from("Single Channel"),
        mp3_metadata::ChannelType::Unknown => String::from("Unknown"),
    }
}

/// Return the `String` version of the MP3 `Copyright` enum
fn mp3_copyright(c: mp3_metadata::Copyright) -> String {
    match c {
        mp3_metadata::Copyright::None => String::from("None"),
        mp3_metadata::Copyright::Some => String::from("Some"),
    }
}

/// Return the `String` version of the MP3 `Status` enum
fn mp3_status(s: mp3_metadata::Status) -> String {
    match s {
        mp3_metadata::Status::Copy => String::from("Copy"),
        mp3_metadata::Status::Original => String::from("Original"),
        mp3_metadata::Status::Unknown => String::from("Unknown"),
    }
}

/// Return the `String` version of the MP3 `Emphasis` enum
fn mp3_emphasis(e: mp3_metadata::Emphasis) -> String {
    match e {
        mp3_metadata::Emphasis::None => String::from("None"),
        mp3_metadata::Emphasis::MicroSeconds => String::from("Microseconds"),
        mp3_metadata::Emphasis::Reserved => String::from("Reserved"),
        mp3_metadata::Emphasis::CCITT => String::from("CCITT"),
        mp3_metadata::Emphasis::Unknown => String::from("Unknown"),
    }
}

/// Open an MP3 file for reading using the `mp3_metadata` crate and return the metadata as a result if OK.
fn open_mp3(filename: &str) -> Result<MP3Metadata, Box<dyn Error>> {
    let meta_res = mp3_metadata::read_from_file(filename);
    match meta_res {
        Ok(r) => Ok(r),
        Err(e) => {
            Err(format!("Unable to open {filename} for to read stream info. Error: {e}").into())
        }
    }
}

/// Returns the ID3 genre as a string
#[allow(clippy::too_many_lines)]
fn genre(g: &mp3_metadata::Genre) -> String {
    match g {
        mp3_metadata::Genre::Blues => String::from("Blues"),
        mp3_metadata::Genre::ClassicRock => String::from("Classic Rock"),
        mp3_metadata::Genre::Country => String::from("Country"),
        mp3_metadata::Genre::Dance => String::from("Dance"),
        mp3_metadata::Genre::Disco => String::from("Disco"),
        mp3_metadata::Genre::Funk => String::from("Funk"),
        mp3_metadata::Genre::Grunge => String::from("Grunge"),
        mp3_metadata::Genre::HipHop => String::from("Hip Hop"),
        mp3_metadata::Genre::Jazz => String::from("Jazz"),
        mp3_metadata::Genre::Metal => String::from("Metal"),
        mp3_metadata::Genre::NewAge => String::from("NewAge"),
        mp3_metadata::Genre::Oldies => String::from("Oldies"),
        mp3_metadata::Genre::Other => String::from("Other"),
        mp3_metadata::Genre::Pop => String::from("Pop"),
        mp3_metadata::Genre::RAndB => String::from("R&B"),
        mp3_metadata::Genre::Rap => String::from("Rap"),
        mp3_metadata::Genre::Reggae => String::from("Reggae"),
        mp3_metadata::Genre::Rock => String::from("Rock"),
        mp3_metadata::Genre::Techno => String::from("Techno"),
        mp3_metadata::Genre::Industrial => String::from("Industrial"),
        mp3_metadata::Genre::Alternative => String::from("Alternative"),
        mp3_metadata::Genre::Ska => String::from("Ska"),
        mp3_metadata::Genre::DeathMetal => String::from("Death Metal"),
        mp3_metadata::Genre::Pranks => String::from("Pranks"),
        mp3_metadata::Genre::Soundtrack => String::from("Soundtrack"),
        mp3_metadata::Genre::EuroTechno => String::from("Euro Techno"),
        mp3_metadata::Genre::Ambient => String::from("Ambient"),
        mp3_metadata::Genre::TripHop => String::from("Trip Hop"),
        mp3_metadata::Genre::Vocal => String::from("Vocal"),
        mp3_metadata::Genre::JazzFunk => String::from("Jazz Funk"),
        mp3_metadata::Genre::Fusion => String::from("Fusion"),
        mp3_metadata::Genre::Trance => String::from("Trance"),
        mp3_metadata::Genre::Classical => String::from("Classical"),
        mp3_metadata::Genre::Instrumental => String::from("Instrumental"),
        mp3_metadata::Genre::Acid => String::from("Acid"),
        mp3_metadata::Genre::House => String::from("House"),
        mp3_metadata::Genre::Game => String::from("Game"),
        mp3_metadata::Genre::SoundClip => String::from("Sound Clip"),
        mp3_metadata::Genre::Gospel => String::from("Gospel"),
        mp3_metadata::Genre::Noise => String::from("Noise"),
        mp3_metadata::Genre::AlternRock => String::from("Alternative Rock"),
        mp3_metadata::Genre::Bass => String::from("Bass"),
        mp3_metadata::Genre::Soul => String::from("Soul"),
        mp3_metadata::Genre::Punk => String::from("Punk"),
        mp3_metadata::Genre::Space => String::from("Space"),
        mp3_metadata::Genre::Meditative => String::from("Meditative"),
        mp3_metadata::Genre::InstrumentalPop => String::from("Instrumental Pop"),
        mp3_metadata::Genre::InstrumentalRock => String::from("Instrumental Rock"),
        mp3_metadata::Genre::Ethnic => String::from("Ethnic"),
        mp3_metadata::Genre::Gothic => String::from("Gothic"),
        mp3_metadata::Genre::Darkwave => String::from("Darkwave"),
        mp3_metadata::Genre::TechnoIndustrial => String::from("Techno Industrial"),
        mp3_metadata::Genre::Electronic => String::from("Electronic"),
        mp3_metadata::Genre::PopFolk => String::from("Pop Folk"),
        mp3_metadata::Genre::Eurodance => String::from("Eurodance"),
        mp3_metadata::Genre::Dream => String::from("Dream"),
        mp3_metadata::Genre::SouthernRock => String::from("Southern Rock"),
        mp3_metadata::Genre::Comedy => String::from("Comedy"),
        mp3_metadata::Genre::Cult => String::from("Cult"),
        mp3_metadata::Genre::Gangsta => String::from("Gangsta"),
        mp3_metadata::Genre::Top40 => String::from("Top 40"),
        mp3_metadata::Genre::ChristianRap => String::from("Christian Rap"),
        mp3_metadata::Genre::PopFunk => String::from("Pop Funk"),
        mp3_metadata::Genre::Jungle => String::from("Jungle"),
        mp3_metadata::Genre::NativeAmerican => String::from("Native American"),
        mp3_metadata::Genre::Cabaret => String::from("Cabaret"),
        mp3_metadata::Genre::NewWave => String::from("New Wave"),
        mp3_metadata::Genre::Psychadelic => String::from("Psychadelic"),
        mp3_metadata::Genre::Rave => String::from("Rave"),
        mp3_metadata::Genre::Showtunes => String::from("Showtunes"),
        mp3_metadata::Genre::Trailer => String::from("Trailer"),
        mp3_metadata::Genre::LoFi => String::from("Lo-Fi"),
        mp3_metadata::Genre::Tribal => String::from("Tribal"),
        mp3_metadata::Genre::AcidPunk => String::from("Acid Punk"),
        mp3_metadata::Genre::AcidJazz => String::from("Acid Jazz"),
        mp3_metadata::Genre::Polka => String::from("Polka"),
        mp3_metadata::Genre::Retro => String::from("Retro"),
        mp3_metadata::Genre::Musical => String::from("Musical"),
        mp3_metadata::Genre::RockAndRoll => String::from("Rock 'n' Roll"),
        mp3_metadata::Genre::HardRock => String::from("Hard Rock"),
        mp3_metadata::Genre::Folk => String::from("Folk"),
        mp3_metadata::Genre::FolkRock => String::from("Folk Rock"),
        mp3_metadata::Genre::NationalFolk => String::from("National Folk"),
        mp3_metadata::Genre::Swing => String::from("Swing"),
        mp3_metadata::Genre::FastFusion => String::from("Fast Fusion"),
        mp3_metadata::Genre::Bebob => String::from("Bebob"),
        mp3_metadata::Genre::Latin => String::from("Latin"),
        mp3_metadata::Genre::Revival => String::from("Revival"),
        mp3_metadata::Genre::Celtic => String::from("Celtic"),
        mp3_metadata::Genre::Bluegrass => String::from("Bluegrass"),
        mp3_metadata::Genre::Avantgarde => String::from("Avantgarde"),
        mp3_metadata::Genre::GothicRock => String::from("Gothic Rock"),
        mp3_metadata::Genre::ProgressiveRock => String::from("Progressive Rock"),
        mp3_metadata::Genre::PsychedelicRock => String::from("Psychedelic Rock"),
        mp3_metadata::Genre::SymphonicRock => String::from("Symphonic Rock"),
        mp3_metadata::Genre::SlowRock => String::from("Slow Rock"),
        mp3_metadata::Genre::BigBand => String::from("Big Band"),
        mp3_metadata::Genre::Chorus => String::from("Chorus"),
        mp3_metadata::Genre::EasyListening => String::from("Easy Listening"),
        mp3_metadata::Genre::Acoustic => String::from("Acoustic"),
        mp3_metadata::Genre::Humour => String::from("Humour"),
        mp3_metadata::Genre::Speech => String::from("Speech"),
        mp3_metadata::Genre::Chanson => String::from("Chanson"),
        mp3_metadata::Genre::Opera => String::from("Opera"),
        mp3_metadata::Genre::ChamberMusic => String::from("Chamber Music"),
        mp3_metadata::Genre::Sonata => String::from("Sonata"),
        mp3_metadata::Genre::Symphony => String::from("Symphony"),
        mp3_metadata::Genre::BootyBrass => String::from("BootyBrass"),
        mp3_metadata::Genre::Primus => String::from("Primus"),
        mp3_metadata::Genre::PornGroove => String::from("Porn Groove"),
        mp3_metadata::Genre::Satire => String::from("Satire"),
        mp3_metadata::Genre::SlowJam => String::from("Slow Jam"),
        mp3_metadata::Genre::Club => String::from("Club"),
        mp3_metadata::Genre::Tango => String::from("Tango"),
        mp3_metadata::Genre::Samba => String::from("Samba"),
        mp3_metadata::Genre::Folklore => String::from("Folklore"),
        mp3_metadata::Genre::Ballad => String::from("Ballad"),
        mp3_metadata::Genre::PowerBallad => String::from("Power Ballad"),
        mp3_metadata::Genre::RhytmicSoul => String::from("Rhytmic Soul"),
        mp3_metadata::Genre::Freestyle => String::from("Freestyle"),
        mp3_metadata::Genre::Duet => String::from("Duet"),
        mp3_metadata::Genre::PunkRock => String::from("Punk Rock"),
        mp3_metadata::Genre::DrumSolo => String::from("Drum Solo"),
        mp3_metadata::Genre::ACapela => String::from("A Capela"),
        mp3_metadata::Genre::EuroHouse => String::from("Euro House"),
        mp3_metadata::Genre::DanceHall => String::from("Dance Hall"),
        mp3_metadata::Genre::Something(s) => s.to_string(),
        mp3_metadata::Genre::Unknown => String::from("Unknown"),
    }
}

/// Returns the `Url` as a `String`
fn url_to_string(u: &mp3_metadata::Url) -> String {
    u.0.clone()
}
