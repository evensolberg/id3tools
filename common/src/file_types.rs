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
        if let Ok(ft_ok) = file_type {
            if let Some(ft) = ft_ok {
                if ft.mime_type() == "audio/x-ape" {
                    return Self::Ape;
                } else if ft.mime_type() == "audio/x-dsf" {
                    return Self::Dsf;
                } else if ft.mime_type() == "audio/x-flac" {
                    return Self::Flac;
                } else if ft.mime_type() == "audio/mpeg" {
                    return Self::MP3;
                } else if ft.mime_type() == "video/mp4" || ft.mime_type() == "audio/m4a" {
                    return Self::M4A;
                }
                return Self::Unknown;
            }
            return Self::Unknown;
        }
        Self::Unknown
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
