//! Show FLAC metadata.

use anyhow::{bail, Result};
use metaflac::block;
use metaflac::Tag;

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
/// `Result<()>` -- Nothing except `Ok` if things go well, otherwise an error.
///
/// **Example:**
///
/// `flac::process("somefile.flac", &my_tags, &my_config)?;`
pub fn show_metadata(filename: &str, show_detail: bool) -> Result<()> {
    let tags = Tag::read_from_path(filename)?;

    // Placeholder for duration
    let mut duration = String::new();

    // Output existing blocks
    for block in tags.blocks() {
        match block {
            metaflac::Block::StreamInfo(si) => {
                if show_detail {
                    // Fetch file size here — only needed for the bitrate calculation in
                    // show_audio_info, so keep it out of the non-detail path.
                    let file_size = std::fs::metadata(filename)?.len();
                    // Attempt the user-friendly summary first; keep the raw dump visible
                    // even if the summary fails (e.g. corrupt file with sample_rate == 0).
                    let audio_result = show_audio_info(si, file_size);
                    show_streaminfo(si);
                    audio_result?;
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
                }
            }
            metaflac::Block::Padding(pad) => {
                if show_detail {
                    show_padding(*pad);
                }
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
                // Duration is shown in the Audio Info block in detail mode; print
                // it here only in non-detail mode to avoid duplication.
                show_vorbis_comment(vc, &duration, show_detail, !show_detail);
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

/// Show a user-friendly "Audio Info:" summary block.
///
/// Surfaces channels, sample rate, bit depth, encoded bitrate, and duration in
/// one place. Only called when `--show-detail` is active; appears before the raw
/// "Stream Info:" dump so the output mirrors the FLAC block order conceptually.
fn show_audio_info(si: &block::StreamInfo, file_size: u64) -> Result<()> {
    let duration_secs = calc_duration_seconds(si.total_samples, si.sample_rate)?;
    let duration_str = format_duration(duration_secs);
    let bitrate = calc_bitrate_kbps(file_size, duration_secs);

    println!("  Audio Info:");
    println!("    Channels    = {}", si.num_channels);
    println!("    Sample Rate = {} Hz", si.sample_rate);
    println!("    Bit Depth   = {} bits", si.bits_per_sample);
    println!("    Bitrate     = {} kbps", bitrate);
    println!("    Duration    = {duration_str}");
    Ok(())
}

/// Compute the container bitrate in kbps from the on-disk file size and duration.
///
/// **Note:** `file_size` is the total bytes on disk, which includes all FLAC metadata
/// blocks (Vorbis Comments, embedded artwork, SeekTable, Padding). The returned value
/// is therefore the *container* bitrate, not the audio-stream-only bitrate. For files
/// with large embedded artwork the figure will be noticeably higher than the pure audio
/// bitrate; this is an accepted trade-off of using file size rather than parsing each
/// block's audio payload.
///
/// Returns 0 if `duration_secs` is zero or negative to avoid division by zero.
#[allow(
    clippy::cast_precision_loss, // u64 → f64: files > 2^53 bytes (~8 PB) lose low bits; acceptable for bitrate estimation
    clippy::cast_sign_loss,      // f64 → u32: f64 is a signed type; the guard above ensures a non-negative value
    clippy::cast_possible_truncation // f64 → u32: sub-kbps remainder is intentionally discarded
)]
fn calc_bitrate_kbps(file_size: u64, duration_secs: f64) -> u32 {
    if duration_secs <= 0.0 {
        return 0;
    }
    (file_size as f64 * 8.0 / duration_secs / 1000.0) as u32
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
fn show_vorbis_comment(vc: &block::VorbisComment, duration: &str, show_detail: bool, show_duration: bool) {
    println!("  Vorbis Comments:");
    if show_detail {
        println!("    Vendor: {}", vc.vendor_string);
    }
    for (key, values) in &vc.comments {
        for value in values {
            println!("    {key} = {value}");
        }
    }
    if show_duration {
        println!("    Duration = {duration} mm:ss");
    }
}

/// Show the `block::Unknown` fields
fn show_unknown(uk: &(u8, Vec<u8>)) {
    println!("  Unknown Block:");
    println!("    Block Code: {}", uk.0);
    println!("    Block Data: {:?}", uk.1);
}

#[allow(clippy::cast_precision_loss, clippy::cast_lossless)]
fn calc_duration_seconds(samples: u64, sample_rate: u32) -> Result<f64> {
    if sample_rate == 0 {
        bail!("Sample rate is zero");
    }

    Ok(samples as f64 / sample_rate as f64)
}

fn calc_duration_string(samples: u64, sample_rate: u32) -> Result<String> {
    Ok(format_duration(calc_duration_seconds(samples, sample_rate)?))
}

/// Format a duration given in seconds as `mm:ss` or `hh:mm:ss` (zero-padded).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn format_duration(secs: f64) -> String {
    let hours = (secs / 3600.0) as u32;
    let minutes = ((secs % 3600.0) / 60.0) as u32;
    let seconds = (secs % 60.0) as u32;
    if hours > 0 {
        format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    } else {
        format!("{minutes:0>2}:{seconds:0>2}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- calc_bitrate_kbps ---

    #[test]
    fn test_calc_bitrate_kbps_normal() {
        // 10_000_000 bytes (SI MB) × 8 bits / 100 s / 1_000 = 800 kbps
        assert_eq!(calc_bitrate_kbps(10_000_000, 100.0), 800);
    }

    #[test]
    fn test_calc_bitrate_kbps_zero_duration() {
        assert_eq!(calc_bitrate_kbps(10_000_000, 0.0), 0);
    }

    // --- calc_duration_seconds ---

    #[test]
    fn test_calc_duration_seconds_normal() {
        let result = calc_duration_seconds(44100, 44100).unwrap();
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calc_duration_seconds_zero_sample_rate() {
        assert!(calc_duration_seconds(44100, 0).is_err());
    }

    // --- calc_duration_string ---

    #[test]
    fn test_calc_duration_string_minutes() {
        // 2 minutes exactly → "02:00"
        let result = calc_duration_string(44100 * 120, 44100).unwrap();
        assert_eq!(result, "02:00");
    }

    #[test]
    fn test_calc_duration_string_hours() {
        // 1 hour, 1 minute, 1 second = 3661 s → "01:01:01"
        let result = calc_duration_string(44100 * 3661, 44100).unwrap();
        assert_eq!(result, "01:01:01");
    }
}
