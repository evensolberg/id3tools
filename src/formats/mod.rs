use std::error::Error;

use crate::{args, default_values::DefaultValues, shared};

mod ape;
mod flac;
mod mp3;
mod mp4;

/// The types of files we can process
#[derive(Debug, Copy, Clone)]
pub enum FileTypes {
    Ape,
    Flac,
    MP3,
    MP4,
    Unknown,
}

/// Performs the actual file processing
///
/// Parameters:
///
/// - `file_type: args::FileType` -- the type of file to process (`Flac`, `MP3` or `MP4`)
/// - `filename: &str` -- the name of the file
/// - `config: &DefaultValuess` -- The default config values to use (stop on error, etc)
/// - `cli_args: &clap::ArgMatches` -- The config values and options supplied from the CLI
/// - `counts: &mut shared::Counts` -- A struct for various file counters (skipped, processed, total)
///
/// Returns:
///
/// - `Ok()` if everything goes well.
/// - `Box<dyn Error>` if we run into problems
pub fn process_file(
    file_type: FileTypes,
    filename: &str,
    config: &DefaultValues,
    cli_args: &clap::ArgMatches,
    counts: &mut shared::Counts,
) -> Result<(), Box<dyn Error>> {
    match file_type {
        FileTypes::Ape => log::debug!("Processing APE."),
        FileTypes::Flac => log::debug!("Processing FLAC."),
        FileTypes::MP3 => log::debug!("Processing MP3."),
        FileTypes::MP4 => log::debug!("Processing MP4."),
        FileTypes::Unknown => return Err(format!("Unknown file type: {}", filename).into()),
    }

    let new_tags_result = args::parse_options(filename, file_type, config, cli_args);
    log::debug!("new_tags_result: {:?}", new_tags_result);
    let new_tags;
    match new_tags_result {
        Ok(res) => {
            new_tags = res;
            log::debug!("New tags: {:?}", new_tags);

            log::debug!("Processing file.");
            let proc_res = match file_type {
                FileTypes::Ape => ape::process_ape(filename, &new_tags, config),
                FileTypes::Flac => flac::process_flac(filename, &new_tags, config),
                FileTypes::MP3 => mp3::process_mp3(filename, &new_tags, config),
                FileTypes::MP4 => mp4::process_mp4(filename, &new_tags, config),
                FileTypes::Unknown => {
                    return Err("We should never get here. That's a problem.".into())
                }
            };

            match proc_res {
                Ok(_) => counts.processed_file_count += 1,
                Err(err) => {
                    if config.stop_on_error.unwrap_or(true) {
                        return Err(format!(
                            "Unable to process {}. Error: {}",
                            filename,
                            err.to_string()
                        )
                        .into());
                    } else {
                        log::error!("Unable to process {}. Error: {}", filename, err.to_string());
                        counts.skipped_file_count += 1;
                    }
                }
            } // match flag::process_flac
        } // Ok(_)
        Err(err) => {
            if config.stop_on_error.unwrap_or(true) {
                return Err(format!(
                    "Unable to parse tags for {}. Error: {}",
                    filename,
                    err.to_string()
                )
                .into());
            } else {
                log::error!(
                    "Unable to parse tags for {}. Error: {}",
                    filename,
                    err.to_string()
                );
                counts.skipped_file_count += 1;
            }
        } // Err(err)
    } // match new_tags_result

    // return safely
    Ok(())
}
