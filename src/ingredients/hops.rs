use serde::{Deserialize, Serialize};
use std::fmt;

/// A type of Hops
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Hops {
    /// Cascade
    Cascade,

    /// Citra
    Citra,

    /// East Kent Goldings
    EastKentGoldings,

    /// Fuggle
    Fuggle,

    /// Hallertau Mittelfruh, a noble hop
    HallertauMittelfruh,

    /// Saaz (Czech)
    Saaz,

    /// Tettnang
    Tettnang,
}

impl Hops {
    /// AA%, The alpha acid level measured in fraction (percentage/100)
    #[must_use]
    pub fn alpha_acid(&self) -> f32 {
        match *self {
            Hops::Cascade => f32::midpoint(4.0, 7.0),
            Hops::Citra => 0.133,
            Hops::EastKentGoldings => 0.055,
            Hops::Fuggle => f32::midpoint(3.5, 6.5),
            Hops::HallertauMittelfruh => 0.0375,
            Hops::Saaz => f32::midpoint(2.0, 4.5),
            Hops::Tettnang => f32::midpoint(3.0, 5.0),
        }
    }
}

impl fmt::Display for Hops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hops::Cascade => write!(f, "[Cascade]"),
            Hops::Citra => write!(f, "[Citra]"),
            Hops::EastKentGoldings => write!(f, "[East Kent Goldings]"),
            Hops::Fuggle => write!(f, "[Fuggle]"),
            Hops::HallertauMittelfruh => write!(f, "[Hallertau Mittelfruh]"),
            Hops::Saaz => write!(f, "[Saaz]"),
            Hops::Tettnang => write!(f, "[Tettnang]"),
        }
    }
}
