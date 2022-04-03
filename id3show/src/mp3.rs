use id3::Tag;
use std::error::Error;

/// Performs the actual processing of MP3 files.
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    // Reat the tag - bomb out if it doesn't work.
    let tag = Tag::read_from_path(&filename)?;

    log::debug!("Tag = {:?}", tag);
    log::debug!("Frames:");
    for item in tag.frames() {
        match item.content() {
            id3::Content::Text(t) => {
                log::info!("  {} = {} (Text)", item.id(), t);
            }
            id3::Content::ExtendedText(et) => {
                log::info!("  {} = {} (Extended Text)", item.id(), et);
            }
            id3::Content::Link(l) => {
                log::info!("  {} = {} (Link)", item.id(), l);
            }
            id3::Content::ExtendedLink(el) => {
                log::info!("  {} = {} (Extended Link)", item.id(), el);
            }
            id3::Content::Comment(co) => {
                log::info!("  {} = {} (Comment)", item.id(), co);
            }
            id3::Content::Popularimeter(pm) => {
                if show_detail {
                    log::info!("  Popularimeter: {}", pm);
                    log::info!("    User: {}", pm.user);
                    log::info!("    Rating: {}", pm.rating);
                    log::info!("    Counter: {}", pm.counter);
                }
            }
            id3::Content::Lyrics(l) => {
                if show_detail {
                    log::info!("  Lyrics:");
                    log::info!("    Language: {}", l.lang);
                    log::info!("    Description: {}", l.description);
                    log::info!("    Text: {}", l.text);
                }
            }
            id3::Content::SynchronisedLyrics(sl) => {
                if show_detail {
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
            }
            id3::Content::Picture(p) => {
                if show_detail {
                    log::info!("  Picture:");
                    log::info!("    Mime Type: {}", p.mime_type);
                    log::info!("    Picture Type: {:?}", p.picture_type);
                    log::info!("    Description: {}", p.description);
                    log::info!("    Picture Data: {} bytes", p.data.len());
                }
            }
            id3::Content::EncapsulatedObject(eo) => {
                if show_detail {
                    log::info!("  Encapsulated Object:");
                    log::info!("    Mime Type: {}", eo.mime_type);
                    log::info!("    Filename: {}", eo.filename);
                    log::info!("    Description: {}", eo.description);
                    log::info!("    Object Data: {} bytes", eo.data.len());
                }
            }
            id3::Content::Chapter(c) => {
                if show_detail {
                    log::info!("  Chapter:");
                    log::info!("    Element ID: {}", c.element_id);
                    log::info!("    Start Time: {}", c.start_time);
                    log::info!("    End Time: {}", c.end_time);
                    log::info!("    Start Offset: {}", c.start_offset);
                    log::info!("    End Offset: {}", c.end_offset);
                    log::info!("    Frame Count: {}", c.frames.len());
                }
            }
            id3::Content::MpegLocationLookupTable(mllt) => {
                if show_detail {
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
            }
            id3::Content::Unknown(uk) => {
                if show_detail {
                    log::info!("  Unknown:");
                    log::info!("    Version: {}", uk.version);
                    log::info!("    Data: {} bytes", uk.data.len());
                }
            }
            _ => {
                return Err(format!("Unknown content type in file {}", filename).into());
            }
        }
    }

    // return safely
    Ok(())
}
