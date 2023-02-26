//! Generate the logging configuration.

use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use std::error::Error;
use std::path::Path;

/// Creates a log configuration for the application.
///
/// # Errors
///
/// - `init_file()` failure
pub fn build_logger(config_filename: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&shellexpand::tilde(&config_filename).to_string()).to_owned();
    if path.exists() {
        log4rs::init_file(path, log4rs::config::Deserializers::default())?;
    } else {
        // Build a stdout logger.
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{date(%Y-%m-%d %H:%M:%S)} {highlight({level})} {message}{n}",
            )))
            .target(Target::Stdout)
            .build();

        // Logging to log file.
        let logfile = FileAppender::builder()
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            .encoder(Box::new(PatternEncoder::new(
                "{date(%Y-%m-%d %H:%M:%S)} {highlight({level})} {message}{n}",
            )))
            .build("./id3tag.log")?;

        // Log Info level output to file where trace is the default level
        // and the programmatically specified level to stdout.
        let config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Warn)))
                    .build("logfile", Box::new(logfile)),
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Info)))
                    .build("stdout", Box::new(stdout)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Info),
            )?;

        // Use this to change log levels at runtime.
        // This means you can change the default log level to trace
        // if you are trying to debug an issue and need more logs on then turn it off
        // once you are done.
        let _handle = log4rs::init_config(config)?;
    }

    Ok(())
}
