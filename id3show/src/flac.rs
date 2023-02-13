//! Show FLAC metadata.

use metaflac::block;
use metaflac::Tag;
use std::error::Error;

/// Shows the metadata contents of the file provided.
///
/// **Parameters:**
///
/// - `filename: &str` -- The name of the file to be processed, eg. "somefile.flac".
/// - `new_tags: &HashMap<String, String>` -- A set of new tags in Key/Value form, eg. _key = ALBUMARTIST_, _value = "The Tragically Hip"_
/// - `config: &DefaultValues` -- A struct containing default values read from a config file and the CLI
///
/// **Returns:**
///
/// `Result<(), Box<dyn Error>>` -- Nothing except `Ok` if things go well, otherwise an error.
///
/// **Example:**
///
/// `flac::process("somefile.flac", &my_tags, &my_config)?;`
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<(), Box<dyn Error>> {
    let tags = Tag::read_from_path(filename)?;

    // Output existing blocks
    for block in tags.blocks() {
        match block {
            metaflac::Block::StreamInfo(si) => {
                if show_detail {
                    show_streaminfo(si);
                }
            }
            metaflac::Block::Application(app) => {
                if show_detail {
                    show_application(app);
                }
            }
            metaflac::Block::CueSheet(cs) => {
                if show_detail {
                    show_cuesheet(cs)
                };
            }
            metaflac::Block::Padding(pad) => {
                if show_detail {
                    show_padding(*pad)
                };
            }
            metaflac::Block::Picture(pic) => {
                if show_detail {
                    show_picture(pic);
                }
            }
            metaflac::Block::SeekTable(st) => {
                if show_detail {
                    show_seektable(st);
                }
            }
            metaflac::Block::VorbisComment(vc) => {
                show_vorbis_comment(vc, show_detail);
            }
            metaflac::Block::Unknown(uk) => {
                if show_detail {
                    show_unknown(uk);
                }
            }
        }
    }

    // Return safely
    Ok(())
}

/// Show the `block::StreamInfo` fields
fn show_streaminfo(si: &block::StreamInfo) {
    log::info!("  Stream Info:");
    log::info!("    Min Block Size: {}", si.min_block_size);
    log::info!("    Max Block Size: {}", si.max_block_size);
    log::info!("    Min Frame Size: {}", si.min_frame_size);
    log::info!("    Max Frame Size: {}", si.max_frame_size);
    log::info!("    Sample Rate: {}", si.sample_rate);
    log::info!("    Channels: {}", si.num_channels);
    log::info!("    Bits Per Sample: {}", si.bits_per_sample);
    log::info!("    Total Samples: {}", si.total_samples);
    log::info!("    MD5: {:?}", si.md5);
}

/// Show the `block::Application` fields
fn show_application(app: &block::Application) {
    log::info!("  Application Block:");
    log::info!("    Application IDs: {:?}", app.id);
    log::info!("    Application Data: {:?}", app.data);
}

/// Show the `block::CueSheet` fields
fn show_cuesheet(cs: &block::CueSheet) {
    log::info!("  Cue Sheet:");
    log::info!("    Catalog number: {}", cs.catalog_num);
    log::info!("    Lead-ins: {}", cs.num_leadin);
    log::info!("    Is CD: {}", cs.is_cd);
    log::info!("    Tracks: {:?}", cs.tracks);
}

/// Show the `block::Padding` fields
fn show_padding(pad: u32) {
    log::info!("  Padding:");
    log::info!("    Padding Size: {}", pad);
}

/// Show the `block::Picture` fields
fn show_picture(pic: &block::Picture) {
    log::info!("  Picture:");
    log::info!("    Picture Type: {:?}", pic.picture_type);
    log::info!("    MIME Type: {}", pic.mime_type);
    log::info!("    Description: {}", pic.description);
    log::info!("    Width: {}", pic.width);
    log::info!("    Height: {}", pic.height);
    log::info!("    Color Depth: {}", pic.depth);
    log::info!("    Color Count: {}", pic.num_colors);
    log::info!("    Picture size: {} bytes", pic.data.len());
}

/// Show the `block::SeekTable` fields
fn show_seektable(st: &block::SeekTable) {
    log::info!("  Seek Table:");
    log::info!("    Number of Seek Points: {:?}", st.seekpoints.len());
}

/// Show the `block::VorbisComment` fields
fn show_vorbis_comment(vc: &block::VorbisComment, show_detail: bool) {
    log::info!("  Vorbis Comments:");
    if show_detail {
        log::info!("    Vendor: {}", vc.vendor_string);
    }
    for (key, values) in &vc.comments {
        for value in values {
            log::info!("  {} = {}", key, value);
        }
    }
}

/// Show the `block::Unknown` fields
fn show_unknown(uk: &(u8, Vec<u8>)) {
    log::info!("  Unknown Block:");
    log::info!("    Block Code: {}", uk.0);
    log::info!("    Block Data: {:?}", uk.1);
}
