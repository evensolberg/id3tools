//! Contains a single function to build the main CLI for the `id3tag` program.
//! This is also used by the `id3cli-gen` program to generate the CLI completion tags for Fig, Bash, etc.
use clap::{Arg, Command};

/// Builds the CLI so the main file doesn't get cluttered. Note that the `<'static>` means it returns a global variable.
pub fn build_cli(version: &'static str) -> Command<'static> {
    // This is the heading under which all the tags settings are grouped
    // run the app with `-h` to see.
    let tags_name = "TAGS";
    let operations_name = "OPERATIONS";
    let images_name = "IMAGES";
    Command::new("id3tag")
        .about("A simple application for updating metadata (ID3) information in music files.")
        .version(version)
        .author(clap::crate_authors!("\n"))
        .long_about("A simple application for updating metadata (ID3) information in music files.")
        .override_usage("id3tag <FILE(S)> [OPTIONS] [TAGS]")
        .arg( // Files - the files to process
            Arg::new("files")
                .value_name("FILE(S)")
                .help("One or more file(s) to process.")
                .long_help("One or more files to process.  Wildcards and multiple_occurrences files (e.g. 2019*.flac 2020*.mp3) are supported. Use the ** glob to recurse (eg. **/*.mp3). Note: Case sensitive.")
                .takes_value(true)
                .multiple_occurrences(true)
                .required(true)
        )
        .arg( // Stop on error
            Arg::new("stop-on-error")
                .short('s')
                .long("stop-on-error")
                .multiple_occurrences(false)
                .help("Stop on error.")
                .long_help("Stop on error. If this flag isn't set, the application will attempt to continue in case of error.")
                .takes_value(false)
        )
        .arg( // Dry-run
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .help("Iterate through the files and produce output without actually processing anything.")
                .multiple_occurrences(false)
                .takes_value(false)
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .multiple_occurrences(false)
                .help("Print summary after all files are processed.")
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .help("Don't display detailed information about each file processed.")
                .multiple_occurrences(false)
                .takes_value(false)
        )
        .arg( // Don't export detail information
            Arg::new("single-thread")
                .short('1')
                .long("single-thread")
                .help("Run processing single-threaded. Takes longer, but has less impact on the system.")
                .multiple_occurrences(false)
                .takes_value(false)
        )
        .arg( // Config file
            Arg::new("config-file")
                .short('c')
                .long("config-file")
                .help("The name of the config file to be read.")
                .long_help("The name of the config file to be read. Note that this is specified WITHOUT the '=', eg. -c myconfig.toml")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .default_missing_value("~/.config/id3tag/config.toml")
                .display_order(1)
        )
        .arg( // Log config
            Arg::new("log-config-file")
                .short('l')
                .long("log-config-file")
                .help("The name of the YAML file containing the logging settings.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .default_missing_value("~/.config/id3tag/id3tag-logs.yaml")
                .display_order(2)
        )
        //////////////////////////////////////////////
        // Options
        .arg( // Album artist
            Arg::new("album-artist")
                .long("album-artist")
                .visible_alias("aa")
                .help("The album artist(s).")
                .long_help("The name of the album artist(s). Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Album artist sort
            Arg::new("album-artist-sort")
                .long("album-artist-sort")
                .visible_alias("aas")
                .help("Album artist(s) sort name.")
                .long_help("The name on which the album artist(s) is sorted. Use quotation marks for multi-word entries. Example: Artist is 'Alicia Keys', but this value may be 'Keys, Alicia'. This is usually set to be the same for all tracks and discs for an album. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Album title
            Arg::new("album-title")
                .long("album-title")
                .visible_alias("at")
                .help("The album title.")
                .help("The title of the album. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Album title sort
            Arg::new("album-title-sort")
                .long("album-title-sort")
                .visible_alias("ats")
                .help("The album title sort name.")
                .long_help("The sorting title of the album. Use quotation marks for multi-word entries. Example: Title is 'The Division Bell', but the sorting title is 'Division Bell, The'. Not commonly used.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Disc number
            Arg::new("disc-number")
                .long("disc-number")
                .visible_alias("dn")
                .help("The disc number.")
                .long_help("The disc number for the disc being processed. This would take the form of 'DISCNUMBER (this value) of TOTALDISCS'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
            )
        .arg( // Determine disc number automagically
            Arg::new("disc-number-count")
                .long("disc-number-count")
                .visible_alias("dnc")
                .help("Determine the disc number and total number of discs based on the folder structure.")
                .long_help("Tries to determine disc number and total number of discs for the disc being processed based on whether we're in a subdirectory called 'CD xx' or 'Disc xx' etc. If not, assumes the disc number to be 1.")
                .takes_value(false)
                .multiple_occurrences(false)
                .require_equals(false)
                .conflicts_with("disc-number")
                .conflicts_with("disc-total")
                .help_heading(tags_name)
            )
        .arg( // Disc total
            Arg::new("disc-total")
                .long("disc-number-total")
                .visible_alias("dt")
                .help("The total number of discs for the album.")
                .long_help("The total number of discs that make up this album. This would take the form of 'DISCNUMBER of TOTALDISCS (this value)'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Track artist
            Arg::new("track-artist")
                .long("track-artist")
                .visible_alias("ta")
                .help("The track artist.")
                .long_help("The name of the track artist(s). Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Track artist
            Arg::new("track-album-artist")
                .long("track-album-artist")
                .visible_alias("taa")
                .help("Set album and track artist to be the same value.")
                .long_help("Sets both the album artist and track artist to the value provided.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .conflicts_with("track-artist")
                .conflicts_with("album-artist")
                .help_heading(tags_name)
        )
        .arg( // Track artist sort
            Arg::new("track-artist-sort")
                .long("track-artist-sort")
                .visible_alias("tas")
                .help("The track artist(s) sort name.")
                .help("The sort name of the track artist(s). Use quotation marks for multi-word entries. Example: Artist is 'Alicia Keys', but this value may be 'Keys, Alicia'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Track title
            Arg::new("track-title")
                .long("track-title")
                .visible_alias("tt")
                .help("The title of the track.")
                .long_help("The title of the track. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Track title sort
            Arg::new("track-title-sort")
                .long("track-title-sort")
                .visible_alias("tts")
                .help("The sort title of the track.")
                .help("The sort title of the track. Use quotation marks for multi-word entries. This is rarely used.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
        )
        .arg( // Track number
            Arg::new("track-number")
                .long("track-number")
                .visible_alias("tn")
                .help("The track number.")
                .help("The track number. Takes the form of 'TRACKNUMBER (this value) of TOTALTRACKS'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .help_heading(tags_name)
            )
        .arg( // Track total
            Arg::new("track-total")
                .long("track-number-total")
                .visible_alias("to")
                .help("The total number of tracks for the disc.")
                .help("The total number of tracks for the disc. Takes the form of 'TRACKNUMBER of TOTALTRACKS (this value)'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(tags_name)
        )
        .arg( // Track count
            Arg::new("track-count")
                .long("track-number-count")
                .visible_alias("tnc")
                .help("Use number of files as total number of tracks.")
                .help("Counts the number of files with the same extension in the same subdirectory, and uses it as the total number of tracks for the disc.")
                .takes_value(false)
                .multiple_occurrences(false)
                .conflicts_with("track-total")
                .help_heading(tags_name)
        )
        .arg( // Track genre
            Arg::new("track-genre")
                .long("track-genre")
                .visible_alias("tg")
                .help("The track music genre.")
                .long_help("The track music genre (eg. 'Rock', 'R&B', 'Classical'). This is usually set to the same value for all tracks on a disc or album. Use quotation marks for multi-word entries. Cannot be combined with '--track-genre-number'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(tags_name)
        )
        .arg( // Track genre number
            Arg::new("track-genre-number")
                .long("track-genre-number")
                .visible_alias("tgn")
                .help("The track music genre number.")
                .long_help("The track music genre number (eg. 'Rock'=17, 'R&B'=14, 'Classical'=32). This is usually set to the same value for all tracks on a disc or album. Cannot be combined with '--track-genre'. Whichever is passed LAST is used.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .conflicts_with("track-genre") // works both ways
                .validator(genre_number_validator).help_heading(tags_name)
        )
        .arg( // Track composer
            Arg::new("track-composer")
                .long("track-composer")
                .visible_alias("tc")
                .help("The composer(s) for the track.")
                .help("The composer(s) for the track. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(tags_name)
        )
        .arg( // Track composer sort
            Arg::new("track-composer-sort")
                .long("track-composer-sort")
                .visible_alias("tcs")
                .help("The sort composer(s) for the track.")
                .help("The sort composer(s) for the track. Use quotation marks for multi-word entries. For example, if the composer is 'Ludwig van Beethoven', this value could be 'Beethoven, Ludwig van'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(tags_name)
        )
        .arg( // Track date
            Arg::new("track-date")
                .long("track-date")
                .visible_alias("td")
                .help("The release date for the track.")
                .help("The release date for the track. This is usually the album release date. Can be a year or a date.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(tags_name)
        )
        .arg( // Track comments
            Arg::new("track-comments")
                .long("track-comments")
                .visible_alias("tm")
                .help("The comments for the track.")
                .help("The comments for the track. Use quotation marks for multi-word entries.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(tags_name)
        )
        .arg( // Front cover picture
            Arg::new("picture-front")
                .long("picture-front")
                .visible_alias("pf")
                .help("The front cover picture file name.")
                .long_help("The front cover picture file name. If not found, this is the output filename if one of the candidates is found. Default: 'cover-front.jpg'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(images_name)
                .default_missing_value("cover-front.jpg")
        )
        .arg( // Front cover picture candidate
            Arg::new("picture-front-candidate")
                .long("picture-front-candidate")
                .visible_alias("pfc")
                .help("The front cover picture candidate file name.")
                .long_help("The front cover picture candidate file name. Example: 'front.jpg' or 'folder.jpg'. Looks for the cover picture alongside the music first, then in the parent folder, then in any directories supplied using the `--picture-search-folder` argument.")
                .takes_value(true)
                .multiple_occurrences(true)
                .require_equals(false).help_heading(images_name)
                .requires("picture-front")
        )
        .arg( // Back cover picture
            Arg::new("picture-back")
                .long("picture-back")
                .visible_alias("pb")
                .help("The back cover picture file name.")
                .long_help("The back cover picture file name. If not found, this is the output filename if one of the candidates is found. Default: 'cover-back.jpg'.")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false).help_heading(images_name)
                .default_missing_value("cover-back.jpg")
        )
        .arg( // Front cover picture candidate
            Arg::new("picture-back-candidate")
                .long("picture-back-candidate")
                .visible_alias("pbc")
                .help("The back cover picture candidate file name.")
                .long_help("The back cover picture candidate file name. Example: 'back.jpg' or 'back-cover.jpg'. Looks for the cover picture alongside the music first, then in the parent folder, then in any directories supplied using the `--picture-search-folder` argument.")
                .takes_value(true)
                .multiple_occurrences(true)
                .require_equals(false).help_heading(images_name)
                .requires("picture-back")
        )
        .arg( // Picture search folder
            Arg::new("picture-search-folder")
                .long("picture-search-folder")
                .visible_alias("psf")
                .help("Folder(s) in which to look for the candidate front and back covers.")
                .long_help("The folder(s) to seach for the candidate cover images. Can be either relative to the music file ('../Artwork') or absolute ('/users/me/Documents/images').")
                .takes_value(true)
                .default_missing_value(".")
                .multiple_occurrences(true)
                .require_equals(false).help_heading(images_name)
        )
        .arg( // Picture max size
            Arg::new("picture-max-size")
                .long("picture-max-size")
                .visible_alias("pms")
                .help("Picture maximum size in pixels for the longest edge.")
                .long_help("The number of pixels for the longest edge of the cover picture. The default is '0', which means no maximum size.")
                .takes_value(true)
                .default_missing_value("0")
                .multiple_occurrences(false)
                .require_equals(false).help_heading(images_name)
        )
        .arg( // Tags (Hidden)
            Arg::new("tags")
                .long("tags")
                .short('t')
                .help("The tags you wish to set in the form `key1=value1, key2=value2`. Note the space between entries!")
                .takes_value(true)
                .multiple_occurrences(true)
                .require_equals(false)
                .hide(true).help_heading(tags_name)
        )
        .arg( // Rename file
            Arg::new("rename-file")
                .long("rename-file")
                .visible_alias("rf")
                .help("Renames the music file after setting the tags. Example: \"%dn-%tn %tt\"")
                .takes_value(true)
                .multiple_occurrences(false)
                .require_equals(false)
                .required(false)
                .validator(crate::shared::validate_file_rename_pattern)
                .hide(false).help_heading(operations_name)
                .display_order(1)
        )
}

/// Checks that the specified genre number is in the valid range (0..=191)
fn genre_number_validator(input: &str) -> Result<(), String> {
    let genre_num = input.parse::<u16>();

    genre_num.map_or_else(
        |_| {
            Err(String::from(
                "Unable to parse the input provided to --track-genre-number.",
            ))
        },
        |gn| {
            if gn <= 191 {
                Ok(())
            } else {
                Err(String::from("track-genre-number must be 0-191."))
            }
        },
    )
}

#[cfg(test)]
/// Test the CLI functions.
mod tests {
    use super::*;

    #[test]
    /// Test that the genre number validator returns OK if genre number is 0..=191, otherwise error.
    fn test_genre_numbervalidator() {
        // Check the valid range.
        for i in 0..=191 {
            assert_eq!(
                genre_number_validator(&i.to_string()),
                Ok(()),
                "genre_number_validator failed for {}",
                i
            );
        }

        // Check numbers outside the valid range.
        assert!(genre_number_validator("192").is_err());
        assert!(genre_number_validator("200").is_err());
        assert!(genre_number_validator("200_000").is_err());

        // Check that other types of input are rejected.
        assert!(genre_number_validator("wrong!").is_err());
    }

    // TODO: Create tests for the build_cli() function.
}
