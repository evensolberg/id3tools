use std::fmt;

/// The types of files we can process
#[derive(Debug, Copy, Clone, Default)]
pub enum FileTypes {
    Ape,
    Dsf,
    Flac,
    MP3,
    MP4,

    #[default]
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

#[cfg(test)]
///
mod tests {
    use super::*;
    use assay::assay;

    #[assay]
    ///
    fn test_fmt() {
        assert_eq!(format!("{}", FileTypes::Ape), "APE");
        assert_eq!(format!("{}", FileTypes::Dsf), "DSF");
        assert_eq!(format!("{}", FileTypes::Flac), "FLAC");
        assert_eq!(format!("{}", FileTypes::MP3), "MP3");
        assert_eq!(format!("{}", FileTypes::MP4), "MP4");
        assert_eq!(format!("{}", FileTypes::Unknown), "Unknown");
    }
}
