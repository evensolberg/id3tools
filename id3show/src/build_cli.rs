use clap::{Arg, Command}; // Command line

pub fn build_cli() -> Command {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .long_about("Show the metadata header of music files.")
        .arg(
            Arg::new("files")
                .value_name("FILE(S)")
                .help("One or more file(s) to process. Globs, wildcards and multiple files (e.g. *.mp3 Genesis/**/*.flac) are supported.")
                .num_args(1..),
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .help("Print summary detail for each session processed.")
                .num_args(0)
        )
        .arg( // Don't export detail information
            Arg::new("show-detail")
                .short('d')
                .long("show-detail")
                .help("Show detailed information about each file processed.")
                .num_args(0)
        )
        .arg( // Log config
            Arg::new("log-config-file")
                .short('l')
                .long("log-config-file")
                .help("The name of the YAML file containing the logging settings.")
                .num_args(0..)
                .default_missing_value("~/.config/id3tag/id3show-logs.yaml")
                .display_order(2)
        )
}
