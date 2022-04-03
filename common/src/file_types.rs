use std::fmt;

/// The types of files we can process
#[derive(Debug, Copy, Clone)]
pub enum FileTypes {
    Ape,
    Dsf,
    Flac,
    MP3,
    MP4,
    Unknown,
}

impl fmt::Display for FileTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filetype = (match self {
            FileTypes::Ape => "APE",
            FileTypes::Dsf => "DSF",
            FileTypes::Flac => "FLAC",
            FileTypes::MP3 => "MP3",
            FileTypes::MP4 => "MP4",
            FileTypes::Unknown => "Unknown",
        })
        .to_string();

        write!(f, "{}", filetype)
    }
}
