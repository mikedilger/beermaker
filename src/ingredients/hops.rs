use serde::{Deserialize, Serialize};
use std::fmt;

/// Typical usage of a hops variety
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HopsUsage {
    Bittering,
    Finishing,
    DualPurpose,
}

/// A variety of Hops
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Hops {
    /// Cascade
    Cascade,

    /// Chinook,
    Chinook,

    /// Citra
    Citra,

    /// East Kent Goldings
    EastKentGoldings,

    /// Fuggle
    Fuggle,

    /// Hallertau Mittelfruh, a noble hop
    HallertauMittelfruh,

    /// Magnum
    Magnum,

    /// Mosaic
    Mosaic,

    /// Nelson Sauvin
    NelsonSauvin,

    /// Saaz (Czech)
    Saaz,

    /// Simcoe
    Simcoe,

    /// Target
    Target,

    /// Tettnang
    Tettnang,

    /// Williamette
    Williamette,
}

impl fmt::Display for Hops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hops::Cascade => write!(f, "[Cascade]"),
            Hops::Chinook => write!(f, "[Chinook]"),
            Hops::Citra => write!(f, "[Citra]"),
            Hops::EastKentGoldings => write!(f, "[East Kent Goldings]"),
            Hops::Fuggle => write!(f, "[Fuggle]"),
            Hops::HallertauMittelfruh => write!(f, "[Hallertau Mittelfruh]"),
            Hops::Magnum => write!(f, "[Magnum]"),
            Hops::Mosaic => write!(f, "[Mosaic]"),
            Hops::NelsonSauvin => write!(f, "[Nelson Sauvin]"),
            Hops::Saaz => write!(f, "[Saaz]"),
            Hops::Simcoe => write!(f, "[Simcoe]"),
            Hops::Target => write!(f, "[Target]"),
            Hops::Tettnang => write!(f, "[Tettnang]"),
            Hops::Williamette => write!(f, "[Williamette]"),
        }
    }
}

impl Hops {
    /// Typical usage of the hop variety
    #[must_use]
    pub fn usage(&self) -> HopsUsage {
        match *self {
            Hops::Cascade => HopsUsage::Finishing,
            Hops::Chinook => HopsUsage::DualPurpose,
            Hops::Citra => HopsUsage::DualPurpose,
            Hops::EastKentGoldings => HopsUsage::DualPurpose,
            Hops::Fuggle => HopsUsage::Finishing,
            Hops::HallertauMittelfruh => HopsUsage::Finishing,
            Hops::Magnum => HopsUsage::Bittering,
            Hops::Mosaic => HopsUsage::DualPurpose,
            Hops::NelsonSauvin => HopsUsage::DualPurpose,
            Hops::Saaz => HopsUsage::Finishing,
            Hops::Simcoe => HopsUsage::DualPurpose,
            Hops::Target => HopsUsage::DualPurpose,
            Hops::Tettnang => HopsUsage::Finishing,
            Hops::Williamette => HopsUsage::Finishing,
        }
    }

    /// AA%, The alpha acid level measured in fraction (percentage/100)
    #[must_use]
    pub fn alpha_acid(&self) -> f32 {
        match *self {
            Hops::Cascade => f32::midpoint(4.0, 7.0),
            Hops::Chinook => f32::midpoint(12.0, 14.0),
            Hops::Citra => 0.133,
            Hops::EastKentGoldings => 0.055,
            Hops::Fuggle => f32::midpoint(3.5, 6.5),
            Hops::HallertauMittelfruh => 0.0375,
            Hops::Magnum => f32::midpoint(12.0, 14.0),
            Hops::Mosaic => f32::midpoint(11.5, 13.5),
            Hops::NelsonSauvin => f32::midpoint(12.0, 13.0),
            Hops::Saaz => f32::midpoint(2.0, 4.5),
            Hops::Simcoe => f32::midpoint(12.0, 14.0),
            Hops::Target => 0.115,
            Hops::Tettnang => f32::midpoint(3.0, 5.0),
            Hops::Williamette => 0.054,
        }
    }
}
