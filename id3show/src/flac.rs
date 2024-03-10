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

    // Placeholder for duration
    let mut duration = String::new();

    // Output existing blocks
    for block in tags.blocks() {
        match block {
            metaflac::Block::StreamInfo(si) => {
                if show_detail {
                    show_streaminfo(si);
                }
                duration = calc_duration_string(si.total_samples, si.sample_rate)?;
            }
            metaflac::Block::Application(app) => {
                if show_detail {
                    show_application(app);
                }
            }
            metaflac::Block::CueSheet(cs) => {
                if show_detail {
                    show_cuesheet(cs);
                };
            }
            metaflac::Block::Padding(pad) => {
                if show_detail {
                    show_padding(*pad);
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
                show_vorbis_comment(vc, &duration, show_detail);
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
    println!("  Stream Info:");
    println!("    Min Block Size: {}", si.min_block_size);
    println!("    Max Block Size: {}", si.max_block_size);
    println!("    Min Frame Size: {}", si.min_frame_size);
    println!("    Max Frame Size: {}", si.max_frame_size);
    println!("    Sample Rate: {}", si.sample_rate);
    println!("    Channels: {}", si.num_channels);
    println!("    Bits Per Sample: {}", si.bits_per_sample);
    println!("    Total Samples: {}", si.total_samples);
    println!("    MD5: {:?}", si.md5);
}

/// Show the `block::Application` fields
fn show_application(app: &block::Application) {
    println!("  Application Block:");
    println!("    Application IDs: {:?}", app.id);
    println!("    Application Data: {:?}", app.data);
}

/// Show the `block::CueSheet` fields
fn show_cuesheet(cs: &block::CueSheet) {
    println!("  Cue Sheet:");
    println!("    Catalog number: {}", cs.catalog_num);
    println!("    Lead-ins: {}", cs.num_leadin);
    println!("    Is CD: {}", cs.is_cd);
    println!("    Tracks: {:?}", cs.tracks);
}

/// Show the `block::Padding` fields
fn show_padding(pad: u32) {
    println!("  Padding:");
    println!("    Padding Size: {pad}");
}

/// Show the `block::Picture` fields
fn show_picture(pic: &block::Picture) {
    println!("  Picture:");
    println!("    Picture Type: {:?}", pic.picture_type);
    println!("    MIME Type: {}", pic.mime_type);
    println!("    Description: {}", pic.description);
    println!("    Width: {}", pic.width);
    println!("    Height: {}", pic.height);
    println!("    Color Depth: {}", pic.depth);
    println!("    Color Count: {}", pic.num_colors);
    println!("    Picture size: {} bytes", pic.data.len());
}

/// Show the `block::SeekTable` fields
fn show_seektable(st: &block::SeekTable) {
    println!("  Seek Table:");
    println!("    Number of Seek Points: {:?}", st.seekpoints.len());
}

/// Show the `block::VorbisComment` fields
fn show_vorbis_comment(vc: &block::VorbisComment, duration: &str, show_detail: bool) {
    println!("  Vorbis Comments:");
    if show_detail {
        println!("    Vendor: {}", vc.vendor_string);
    }
    for (key, values) in &vc.comments {
        for value in values {
            println!("    {key} = {value}");
        }
    }
    println!("    Duration = {duration} mm:ss");
}

/// Show the `block::Unknown` fields
fn show_unknown(uk: &(u8, Vec<u8>)) {
    println!("  Unknown Block:");
    println!("    Block Code: {}", uk.0);
    println!("    Block Data: {:?}", uk.1);
}

fn calc_duration_seconds(samples: u64, sample_rate: u32) -> Result<f64, Box<dyn Error>> {
    if sample_rate == 0 {
        return Err("Sample rate is zero".into());
    }

    Ok(samples as f64 / sample_rate as f64)
}

fn calc_duration_string(samples: u64, sample_rate: u32) -> Result<String, Box<dyn Error>> {
    let duration = calc_duration_seconds(samples, sample_rate)?;
    let hours = (duration / 3600.0) as u32;
    let minutes = (duration / 60.0) as u32;
    let seconds = (duration % 60.0) as u32;
    if hours > 0 {
        return Ok(format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds));
    }
    Ok(format!("{:0>2}:{:0>2}", minutes, seconds))
}
