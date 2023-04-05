//! Macros for inserting values into the `HashSet` used to tag files.

/// Insert tags into the new tags list. Replaces mucho repeated code.
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
/// `tag!(am, dv, nt, ot, "album-title", album_title, false);`
/// `tag!(am, dv, nt, ot, "disc-number", disc_number, true);`
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

/// Insert a picture
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
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
/// - `$fname:ident` - The name of the variable containing the music file name
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
/// # Arguments
///
/// - `$cli:ident` - The name of the variable that holds the `clap::ArgMatches`
/// - `$cfg:ident` - The name of the variable that contains the `DefaultValues`
/// - `$nt:ident` - The name of the variable that contains the new tags `HashSet`
/// - `$t:ident` - The name of the variable that contains the existing tags `HashSet`
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
