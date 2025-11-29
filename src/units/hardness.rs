use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

pub use crate::units::concentration::Ppm;

/// German water hardness degrees
/// dH (german degrees of hardness)    1 dH = 17.8 mg/L `CaCO3`
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Dh(pub f32);

impl fmt::Display for Dh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °dH", self.0)
    }
}

const PPM_PER_DH: f32 = 17.8;

impl From<Ppm> for Dh {
    fn from(v: Ppm) -> Self {
        Dh(v.0 / PPM_PER_DH)
    }
}

impl From<Dh> for Ppm {
    fn from(v: Dh) -> Self {
        Ppm(v.0 * PPM_PER_DH)
    }
}

// French water hardness degrees
// fH (french degrees of hardness)    1 fH = 10 mg/L CaCO3
// pub struct Fh(pub f32);

// Water hardness in Grains per gallon
// grains per gallon (gpg)            1 gpg = 17.12 mg/L
// pub struct Gpg(pub f32);

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_hardness_conversions() {
        let a = Ppm(130.0);
        let b = Into::<Ppm>::into(Into::<Dh>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
    }
}
