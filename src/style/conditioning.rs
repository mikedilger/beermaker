use serde::{Deserialize, Serialize};
use std::fmt;

/// Conditioning
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Conditioning {
    /// None
    None,

    /// Lagered
    Lagered,

    /// Aged
    Aged,
}

impl fmt::Display for Conditioning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::None => write!(f, "None"),
            Self::Lagered => write!(f, "Lagered"),
            Self::Aged => write!(f, "Aged"),
        }
    }
}
