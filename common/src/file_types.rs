//! Defines an enum with the types of files we can process.

use serde::Serialize;
use std::fmt;

/// The types of files we can process
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum FileTypes {
    Ape,
    Dsf,
    Flac,
    MP3,
    M4A,

    #[default]
    Unknown,
}

impl FileTypes {
    /// Returns the file type of the file.
    #[must_use]
    pub fn from_filename(filename: &str) -> Self {
        let file_type = infer::get_from_path(filename);
        match file_type {
            Ok(Some(ft)) => match ft.mime_type() {
                "audio/x-ape" => Self::Ape,
                "audio/x-dsf" => Self::Dsf,
                "audio/x-flac" => Self::Flac,
                "audio/mpeg" => Self::MP3,
                "video/mp4" | "audio/m4a" => Self::M4A,
                _ => Self::Unknown,
            },
            Ok(None) => Self::Unknown,
            Err(e) => {
                log::warn!("Unable to read {filename}: {e}");
                Self::Unknown
            }
        }
    }
}

impl fmt::Display for FileTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filetype = (match self {
            Self::Ape => "APE",
            Self::Dsf => "DSF",
            Self::Flac => "FLAC",
            Self::MP3 => "MP3",
            Self::M4A => "M4A",
            Self::Unknown => "Unknown",
        })
        .to_string();

        write!(f, "{filetype}")
    }
}

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    fn test_from_filename() {
        // Skip if testdata is not available (e.g. in CI without LFS files)
        if !std::path::Path::new("../testdata/sample.ape").exists() {
            return;
        }
        assert_eq!(
            FileTypes::from_filename("../testdata/sample.ape"),
            FileTypes::Ape
        );
        assert_eq!(
            FileTypes::from_filename("../testdata/sample.dsf"),
            FileTypes::Dsf
        );
        assert_eq!(
            FileTypes::from_filename("../testdata/sample.flac"),
            FileTypes::Flac
        );
        assert_eq!(
            FileTypes::from_filename("../testdata/sample.mp3"),
            FileTypes::MP3
        );
        assert_eq!(
            FileTypes::from_filename("../testdata/sample.mp4"),
            FileTypes::M4A
        );
    }

    #[test]
    ///
    fn test_fmt() {
        assert_eq!(format!("{}", FileTypes::Ape), "APE");
        assert_eq!(format!("{}", FileTypes::Dsf), "DSF");
        assert_eq!(format!("{}", FileTypes::Flac), "FLAC");
        assert_eq!(format!("{}", FileTypes::MP3), "MP3");
        assert_eq!(format!("{}", FileTypes::M4A), "M4A");
        assert_eq!(format!("{}", FileTypes::Unknown), "Unknown");
    }
}
