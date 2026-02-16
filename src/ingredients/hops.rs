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

    /// Challenger,
    Challenger,

    /// Chinook,
    Chinook,

    /// Citra
    Citra,

    /// East Kent Goldings
    EastKentGoldings,

    /// Fuggles
    Fuggles,

    /// Galaxy
    Galaxy,

    /// Hallertau Mittelfruh, a noble hop
    HallertauMittelfruh,

    /// Magnum
    Magnum,

    /// Mosaic
    Mosaic,

    /// Nelson Sauvin
    // NZ, bright peel-like bitterness, sweet, tropical
    NelsonSauvin,

    /// Northdown
    Northdown,

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
    // Superdelic (NZ)
    // Nectaron (NZ, sweet fruit, pineapple, passion fruit)
    // Motueka (NZ, lime zest, lemon, mint, herbal, floral, papaya,
    // lychee, guava)

    // Riwaka (NZ, thiols, diesel, dank, passion fruit, kumquat,
    //    grapefruit, lime), hard to grow, fussy, low yield, so most
    //    expensive hop in world
    // Waimea (arm pit?)
    // Waiiti (NZ, apricot peach)
}

impl fmt::Display for Hops {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hops::Cascade => write!(f, "[Cascade]"),
            Hops::Challenger => write!(f, "[Challenger]"),
            Hops::Chinook => write!(f, "[Chinook]"),
            Hops::Citra => write!(f, "[Citra]"),
            Hops::EastKentGoldings => write!(f, "[East Kent Goldings]"),
            Hops::Fuggles => write!(f, "[Fuggles]"),
            Hops::Galaxy => write!(f, "[Galaxy]"),
            Hops::HallertauMittelfruh => write!(f, "[Hallertau Mittelfruh]"),
            Hops::Magnum => write!(f, "[Magnum]"),
            Hops::Mosaic => write!(f, "[Mosaic]"),
            Hops::NelsonSauvin => write!(f, "[Nelson Sauvin]"),
            Hops::Northdown => write!(f, "[Northdown]"),
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
            Hops::Challenger => HopsUsage::DualPurpose,
            Hops::Chinook => HopsUsage::DualPurpose,
            Hops::Citra => HopsUsage::DualPurpose,
            Hops::EastKentGoldings => HopsUsage::DualPurpose,
            Hops::Fuggles => HopsUsage::Finishing,
            Hops::Galaxy => HopsUsage::DualPurpose,
            Hops::HallertauMittelfruh => HopsUsage::Finishing,
            Hops::Magnum => HopsUsage::Bittering,
            Hops::Mosaic => HopsUsage::DualPurpose,
            Hops::NelsonSauvin => HopsUsage::DualPurpose,
            Hops::Northdown => HopsUsage::DualPurpose,
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
            Hops::Cascade => f32::midpoint(0.04, 0.07),
            Hops::Challenger => f32::midpoint(0.065, 0.09),
            Hops::Chinook => f32::midpoint(0.12, 0.14),
            Hops::Citra => 0.133,
            Hops::EastKentGoldings => 0.055,
            Hops::Fuggles => f32::midpoint(0.035, 0.065),
            Hops::Galaxy => f32::midpoint(0.11, 0.16),
            Hops::HallertauMittelfruh => 0.0375,
            Hops::Magnum => f32::midpoint(0.12, 0.14),
            Hops::Mosaic => f32::midpoint(0.115, 0.135),
            Hops::NelsonSauvin => f32::midpoint(0.12, 0.13),
            Hops::Northdown => f32::midpoint(0.06, 0.09),
            Hops::Saaz => f32::midpoint(0.02, 0.045),
            Hops::Simcoe => f32::midpoint(0.12, 0.14),
            Hops::Target => 0.115,
            Hops::Tettnang => f32::midpoint(0.03, 0.05),
            Hops::Williamette => 0.054,
        }
    }
}
