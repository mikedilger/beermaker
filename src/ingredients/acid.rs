use serde::{Deserialize, Serialize};
use std::fmt;

/// A type of Acid or Base
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(clippy::doc_markdown)]
pub enum Acid {
    /// Lactic Acid
    LacticAcid,
}

impl fmt::Display for Acid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LacticAcid => write!(f, "[Lactic Acid]"),
        }
    }
}
