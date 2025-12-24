use serde::{Deserialize, Serialize};
use std::fmt;

/// Fermentation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Fermentation {
    /// Ale
    Ale,

    /// Lager
    Lager,

    /// Either
    Either,

    /// Wild
    Wild,
}

impl fmt::Display for Fermentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Ale => write!(f, "Top-fermented"),
            Self::Lager => write!(f, "Botom-fermented"),
            Self::Either => write!(f, "Either"),
            Self::Wild => write!(f, "Wild"),
        }
    }
}
