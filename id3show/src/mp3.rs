use id3::frame;
use id3::Content;
use id3::Tag;
use std::error::Error;

/// Performs the actual processing of MP3 files.
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    // Reat the tag - bomb out if it doesn't work.
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

/// Show the 'frame::MpegLocationLookupTable` fields
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
