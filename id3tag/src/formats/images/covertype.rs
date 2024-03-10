//! Define the `CoverType` enum and associated function(s)
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Default, Copy, Clone)]
/// The types of covers we deal with - `Front`, `Back`
pub enum CoverType {
    #[default]
    Front,
    Back,
}

/// Implements the `Display` trait for the `CoverType` enum
impl Display for CoverType {
    /// Display function for the `CoverType`.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Front => write!(f, "front"),
            Self::Back => write!(f, "back"),
        }
    }
}
