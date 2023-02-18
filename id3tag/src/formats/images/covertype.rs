//! Define the `CoverType` enum and associated function(s)
use crate::default_values::DefaultValues;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Default, Copy, Clone)]
/// The types of covers we deal with - `Front`, `Back`, `FrontCandidate` and `BackCandidate`
pub enum CoverType {
    #[default]
    Front,
    Back,
    FrontCandidate,
    BackCandidate,
}

/// Implements the `Display` trait for the `CoverType` enum
impl Display for CoverType {
    /// Display function for the `CoverType`.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Front => write!(f, "front"),
            Self::Back => write!(f, "back"),
            Self::FrontCandidate => write!(f, "front candidate"),
            Self::BackCandidate => write!(f, "back candidate"),
        }
    }
}

/// Returns the cover name from the config, depending on the type we ask for
///
/// # Arguments
///
/// `cover_type: CoverType` - the type of cover we're looking for, i.e., `FrontCover` or `BackCover`
/// `cfg: &DefaultValues` - the program configuration as collected from the CLI and config file.
///
/// # Returns
///
/// A `String` containing the name of the cover file as specified by the config.
///
/// # Errors
///
///  None.
///
/// # Panics
///
/// None.
pub fn cover_filename_from_config(cover_type: CoverType, cfg: &DefaultValues) -> String {
    match cover_type {
        CoverType::Front | CoverType::FrontCandidate => cfg
            .picture_front
            .as_ref()
            .unwrap_or(&"front-cover.jpg".to_string())
            .clone(),
        CoverType::Back | CoverType::BackCandidate => cfg
            .picture_back
            .as_ref()
            .unwrap_or(&"back-cover.jpg".to_string())
            .clone(),
    }
}
