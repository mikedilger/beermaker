use serde::{Deserialize, Serialize};
use std::fmt;

/// Origin
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StyleOrigin {
    /// American (North)
    American,

    /// Belgian
    Belgian,

    /// Bohemian (Czech)
    Bohemian,

    /// British
    British,

    /// European
    European,

    /// Finland
    Finnish,

    /// French
    French,

    /// German
    German,

    /// Irish
    Irish,

    /// Mexican
    Mexican,

    /// Oceania
    Oceania,

    /// Other
    Other,

    /// Sweden
    Swedish,
}

impl fmt::Display for StyleOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::American => write!(f, "American"),
            Self::Belgian => write!(f, "Belgian"),
            Self::Bohemian => write!(f, "Bohemian"),
            Self::British => write!(f, "British"),
            Self::European => write!(f, "European"),
            Self::Finnish => write!(f, "Finnish"),
            Self::French => write!(f, "French"),
            Self::German => write!(f, "German"),
            Self::Irish => write!(f, "Irish"),
            Self::Mexican => write!(f, "Mexican"),
            Self::Oceania => write!(f, "Oceania"),
            Self::Other => write!(f, "Other"),
            Self::Swedish => write!(f, "Swedish"),
        }
    }
}
