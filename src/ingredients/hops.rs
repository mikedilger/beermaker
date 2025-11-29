use serde::{Deserialize, Serialize};
use std::fmt;

/// A type of Hops
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Hops {
    /// Hallertau Mittelfruh, a noble hop
    HallertauMittelfruh,
}

impl Hops {
    /// AA%, The alpha acid level measured in fraction (percentage/100)
    #[must_use]
    pub fn alpha_acid(&self) -> f32 {
        match *self {
            Hops::HallertauMittelfruh => 0.0375,
        }
    }
}

impl fmt::Display for Hops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hops::HallertauMittelfruh => write!(f, "[Hallertau Mittelfruh]"),
        }
    }
}
