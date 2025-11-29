use super::WaterProfile;
use crate::units::concentration::Ppm;
use serde::{Deserialize, Serialize};

/// Water balance (what we need, or have too much of)
#[allow(clippy::doc_markdown)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WaterBalance {
    /// ca defecit or surplus
    pub ca: Ppm,

    /// mg defecit or surplus
    pub mg: Ppm,

    /// na defecit or surplus
    pub na: Ppm,

    /// so4 defecit or surplus
    pub so4: Ppm,

    /// cl defecit or surplus
    pub cl: Ppm,

    /// alkalinity defecit or surplus
    pub alkalinity_caco3: Ppm,
}

/// Water ratio (what fraction of target are we at)
#[allow(clippy::doc_markdown)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WaterRatio {
    /// ca fraction of target
    pub ca: f32,

    /// mg fraction of target
    pub mg: f32,

    /// na fraction of target
    pub na: f32,

    /// so4 fraction of target
    pub so4: f32,

    /// cl fraction of target
    pub cl: f32,

    /// alkalinity fraction of target
    pub alkalinity_caco3: f32,
}

/// Compare a water profile, to a target water profile to see how
/// they compare by amount and by ratio.
#[must_use]
pub fn compare_to_target(source: WaterProfile, target: WaterProfile) -> (WaterBalance, WaterRatio) {
    let balance = WaterBalance {
        ca: source.ca - target.ca,
        mg: source.mg - target.mg,
        na: source.na - target.na,
        so4: source.so4 - target.so4,
        cl: source.cl - target.cl,
        alkalinity_caco3: source.alkalinity_caco3 - target.alkalinity_caco3,
    };

    let ratio = WaterRatio {
        ca: source.ca.0 / target.ca.0,
        mg: source.mg.0 / target.mg.0,
        na: source.na.0 / target.na.0,
        so4: source.so4.0 / target.so4.0,
        cl: source.cl.0 / target.cl.0,
        alkalinity_caco3: source.alkalinity_caco3.0 / target.alkalinity_caco3.0,
    };

    (balance, ratio)
}
