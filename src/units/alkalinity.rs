use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Alkalinity as `CaCO3` ppm units
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct CaCO3(pub f32);

impl fmt::Display for CaCO3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} ppm Alkalinity as CaCO3", self.0)
    }
}

/// Alkalinity as `HCO3` ppm units
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct HCO3(pub f32);

impl fmt::Display for HCO3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} ppm Alkalinity as HCO3", self.0)
    }
}

const CACO3_PER_HCO3: f32 = 1.22;

impl From<CaCO3> for HCO3 {
    fn from(v: CaCO3) -> Self {
        HCO3(v.0 * CACO3_PER_HCO3)
    }
}

impl From<HCO3> for CaCO3 {
    fn from(v: HCO3) -> Self {
        CaCO3(v.0 / CACO3_PER_HCO3)
    }
}
