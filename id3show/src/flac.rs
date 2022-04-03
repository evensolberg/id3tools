//! Show FLAC metadata.

use metaflac::Tag;
use std::error::Error;

/// Performs the actual processing of FLAC files.
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
    let tags = Tag::read_from_path(&filename)?;

    // Output existing blocks
    for block in tags.blocks() {
        match block {
            metaflac::Block::StreamInfo(si) => {
                if show_detail {
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
            }
            metaflac::Block::Application(app) => {
                if show_detail {
                    log::info!("  Application Block:");
                    log::info!("    Application IDs: {:?}", app.id);
                    log::info!("    Application Data: {:?}", app.data);
                }
            }
            metaflac::Block::CueSheet(cs) => {
                if show_detail {
                    log::info!("  Cue Sheet:");
                    log::info!("    Catalog number: {}", cs.catalog_num);
                    log::info!("    Lead-ins: {}", cs.num_leadin);
                    log::info!("    Is CD: {}", cs.is_cd);
                    log::info!("    Tracks: {:?}", cs.tracks);
                }
            }
            metaflac::Block::Padding(pad) => {
                if show_detail {
                    log::info!("  Padding:");
                    log::info!("    Padding Size: {}", pad);
                }
            }
            metaflac::Block::Picture(pic) => {
                if show_detail {
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
            }
            metaflac::Block::SeekTable(st) => {
                if show_detail {
                    log::info!("  Seek Table:");
                    log::info!("    Number of Seek Points: {:?}", st.seekpoints.len());
                }
            }
            metaflac::Block::VorbisComment(vc) => {
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
            metaflac::Block::Unknown(uk) => {
                if show_detail {
                    log::info!("  Unknown Block:");
                    log::info!("    Block Code: {}", uk.0);
                    log::info!("    Block Data: {:?}", uk.1);
                }
            }
        }
    }

    // Return safely
    Ok(())
}
