//! Macros for inserting values into the `HashSet` used to tag files.

/// Insert tags into the new tags list. Replaces mucho repeated code.
///
/// This macro checks if the command line argument exists in `$cli`.
/// If it does, it inserts the value into `$nt` using the field specified by `$name` in `$t`.
/// If the command line argument does not exist, it checks if the "config-file" argument exists in `$cli`.
/// If it does, it inserts the value from `$cfg` using the field specified by `$name` in `$t`.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
/// - `$arg:expr` - The CLI parameter we're matching on
/// - `$name:ident` - The name of the variable in the `DefaultValues`
/// - `true|false` - Indicates whether to clone the value
///   - `true` - clones the value to be inserted. Use this if it is used later.
///   - `false` - moves the value to be inserted. This is the one you're most likely to use.
///
/// # Examples
///
/// ```
/// tag!(am, dv, nt, ot, "album-title", album_title, false);
/// tag!(am, dv, nt, ot, "disc-number", disc_number, true);
/// ```
#[macro_export]
macro_rules! tag {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $arg:expr, $name:ident, false) => {
        if $cli.contains_id($arg) {
            $nt.insert(
                $t.$name,
                $cli.get_one::<String>($arg)
                    .unwrap_or(&String::new())
                    .to_string(),
            );
        } else if $cli.contains_id("config-file") {
            if let Some(val) = &$cfg.$name {
                $nt.insert($t.$name, val.to_string());
            }
        }
    };
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $arg:expr, $name:ident, true) => {
        if $cli.contains_id($arg) {
            $nt.insert(
                $t.$name.clone(),
                $cli.get_one::<String>($arg)
                    .unwrap_or(&String::new())
                    .to_string(),
            );
        } else if $cli.contains_id("config-file") {
            if let Some(val) = &$cfg.$name {
                $nt.insert($t.$name.clone(), val.to_string());
            }
        }
    };
}

/// This macro is used to handle the insertion of picture metadata into the tag.
///
/// The macro checks if the command line interface contains the appropriate ID for the picture candidate.
/// If it does, it inserts the picture metadata into the tag.
/// If not, it checks if the configuration file contains the picture metadata.
/// If it does, it inserts the picture metadata into the tag.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
/// - `front|back` - Indicate which cover we're inserting
#[macro_export]
macro_rules! pic {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, front) => {
        if $cli.contains_id("picture-front-candidate") {
            $nt.insert(
                $t.picture_front.clone(),
                $cfg.picture_front
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_string(),
            );
        } else if $cli.contains_id("config-file") {
            if let Some(val) = &$cfg.picture_front {
                $nt.insert($t.picture_front, val.to_string());
            }
        }
    };
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, back) => {
        if $cli.contains_id("picture-back-candidate") {
            $nt.insert(
                $t.picture_back.clone(),
                $cfg.picture_back
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_string(),
            );
        } else if $cli.contains_id("config-file") {
            if let Some(val) = &$cfg.picture_back {
                $nt.insert($t.picture_back, val.to_string());
            }
        }
    };
}

/// Set the track and album artist to be the same value. This is just a convenience so we don't have to do both on the CLI.
///
/// This macro is used to handle the logic for setting the track artist and album artist values in the ID3 tag.
/// It takes in the command line interface (`$cli`), configuration (`$cfg`), ID3 tag (`$nt`), and track (`$t`) as input.
/// If the command line interface contains the "track-album-artist" option, it retrieves the value and sets it as the track artist and album artist in the ID3 tag.
/// If the "config-file" option is present in the command line interface, it checks if the `track_album_artist` value is specified in the configuration.
/// If it is, it sets the track artist and album artist in the ID3 tag to the specified value.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
#[macro_export]
macro_rules! track_album_artist {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident) => {
        if $cli.contains_id("track-album-artist") {
            let taa = $cli
                .get_one::<String>("track-album-artist")
                .unwrap_or(&String::new())
                .to_string();
            $nt.insert($t.track_artist.clone(), taa.clone());
            $nt.insert($t.album_artist.clone(), taa);
        } else if $cli.contains_id("config-file") {
            if let Some(val) = &$cfg.track_album_artist {
                let taa = val.to_string();
                $nt.insert($t.track_artist.clone(), taa.clone());
                $nt.insert($t.album_artist.clone(), taa);
            }
        }
    };
}

/// Count the number of discs
///
/// This macro is used to handle the logic for inserting disc number and disc count tags into the ID3 tag.
/// It checks if the "disc-number-count" value is provided through the command line or if the disc count is enabled in the config file.
/// If either condition is true, it retrieves the disc number and disc count from the given file name and inserts them into the ID3 tag.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
/// - `$fname:ident` - The name of the variable containing the music file name
#[macro_export]
macro_rules! disc_number_count {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $fname:ident) => {
        if $cli.value_source("disc-number-count") == Some(clap::parser::ValueSource::CommandLine)
            || ($cli.contains_id("config-file") && $cfg.disc_count.unwrap_or(false))
        {
            let disc_num = disc_number($fname)?;
            let disc_count = disc_count($fname)?;
            $nt.insert($t.disc_number.clone(), format!("{disc_num:0>2}"));
            $nt.insert($t.disc_number_total.clone(), format!("{disc_count:0>2}"));
        }
    };
}

/// Count the number of tracks
///
/// Macro to insert the total track number count into the given tag.
///
/// This macro checks if the command line argument `track-count` is present or if the `config-file` option is enabled and the `track_count` configuration is set to `true`.
/// If either condition is true, it counts the number of files using the `common::count_files` function and inserts the count into the specified tag.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
/// - `$fname:ident` - The name of the variable containing the music file name
///
/// # Example
///
/// ```
/// macro_rules! track_number_count {
///     ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $fname:ident) => {
///         if $cli.contains_id("track-count")
///             || ($cli.contains_id("config-file") && $cfg.track_count.unwrap_or(false))
///         {
///             let file_count = common::count_files($fname)?;
///             $nt.insert($t.track_number_total, file_count);
///         }
///     };
/// }
/// ```
#[macro_export]
macro_rules! track_number_count {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident, $fname:ident) => {
        if $cli.contains_id("track-count")
            || ($cli.contains_id("config-file") && $cfg.track_count.unwrap_or(false))
        {
            let file_count = common::count_files($fname)?;
            $nt.insert($t.track_number_total, file_count);
        }
    };
}

/// Set the genre of the track using the numerical value instead of the string. This just means the numerical value is used to look up the string value.
///
/// This macro is used to handle the logic for inserting track genre numbers and names into a hashmap.
/// It takes four parameters: `$cli`, `$cfg`, `$nt`, and `$t`.
///
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
///
/// If the command line interface contains the "track-genre-number" identifier, it inserts the track genre number and name into the hashmap.
/// If the command line interface does not contain the "track-genre-number" identifier but the configuration file contains the "config-file" identifier, it inserts the track genre number and name from the configuration file into the hashmap.
///
/// # Example
///
/// ```
/// let cli = CommandLineInterface::new();
/// let cfg = ConfigurationFile::new();
/// let mut track_numbers = HashMap::new();
/// let track = Track::new();
///
/// track_genre_num!(cli, cfg, track_numbers, track);
/// ```
///
#[macro_export]
macro_rules! track_genre_num {
    ($cli:ident, $cfg:ident, $nt:ident, $t:ident) => {
        if $cli.contains_id("track-genre-number") {
            $nt.insert(
                $t.track_genre.clone(),
                genre_name(*$cli.get_one::<u16>("track-genre-number").unwrap_or(&0))?,
            );
        } else if $cli.contains_id("config-file") {
            if let Some(val) = &$cfg.track_genre_number {
                $nt.insert($t.track_genre.clone(), genre_name(*val)?);
            }
        }
    };
}
