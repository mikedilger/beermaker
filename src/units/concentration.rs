use super::prelude::*;
use crate::ingredients::{MaltDose, SugarDose};
use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Parts per million
/// Same as mg/kg which in water is also mg/L
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Ppm(pub f32);

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} ppm", self.0)
    }
}

/// Specific Gravity
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct SpecificGravity(pub f32);

impl fmt::Display for SpecificGravity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} s.g.", self.0)
    }
}

/// Plato (approx same as Brix).  Percentage of dissolved sugar.
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Plato(pub f32);

impl fmt::Display for Plato {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}% plato", self.0)
    }
}

impl From<SpecificGravity> for Plato {
    fn from(sg: SpecificGravity) -> Plato {
        let sg = sg.0;
        Plato(-616.868 + 1111.14 * sg - 630.272 * sg.powi(2) + 135.997 * sg.powi(3))
    }
}

impl From<Plato> for SpecificGravity {
    fn from(plato: Plato) -> SpecificGravity {
        let plato = plato.0;
        SpecificGravity(1.000 + (plato / (258.6 - ((plato / 258.2) * 227.1))))
    }
}

/// Brix
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Div)]
pub struct Brix(pub f32);

impl fmt::Display for Brix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}% brix", self.0)
    }
}

impl From<SpecificGravity> for Brix {
    fn from(sg: SpecificGravity) -> Brix {
        Brix(((182.4601 * sg.0 - 775.6821) * sg.0 + 1262.7794) * sg.0 - 669.5622)
    }
}

impl From<Brix> for SpecificGravity {
    fn from(b: Brix) -> SpecificGravity {
        SpecificGravity(1.000 + (b.0 / (258.6 - ((b.0 / 258.2) * 227.1))))
    }
}

impl SpecificGravity {
    /// Compute the specific gravity of the given malts and sugars
    /// in the given volume of waters, at the given mash efficiency.
    #[must_use]
    pub fn from_recipe(
        malts: &[MaltDose],
        sugars: &[SugarDose],
        volume: Gallons,
        mash_efficiency: f32,
    ) -> Self {
        let mut points: f32 = 0.0;

        for malt_dose in malts {
            let pounds: Pounds = malt_dose.weight.into();
            let pts = malt_dose.malt.ppg() * pounds.0 * mash_efficiency;
            let points_added = pts / volume.0;
            points += points_added;
        }

        for sugar_dose in sugars {
            let pounds: Pounds = sugar_dose.weight.into();
            let pts = sugar_dose.sugar.ppg() * pounds.0;
            let points_added = pts / volume.0;
            points += points_added;
        }

        SpecificGravity(1.0 + points / 1000.0)
    }
}

/// Alcohol by volume, fraction (not percent)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Div)]
pub struct Abv(pub f32);

impl fmt::Display for Abv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p: Percent = (*self).into();
        write!(f, "{p} ABV")
    }
}

impl Abv {
    /// Compute Abv from original and final gravity.
    ///
    /// `dilution_fraction` must be 1.0 or greater.
    #[must_use]
    pub fn from_gravity(
        original_gravity: SpecificGravity,
        final_gravity: SpecificGravity,
        dilution_fraction: f32,
    ) -> Abv {
        let og = original_gravity.0;
        let fg = final_gravity.0;
        let abv = (76.08 * (og - fg) / (1.775 - og)) * (fg / 0.794) / 100.0;
        Abv(abv / dilution_fraction)
    }
}

/// A concentration by percentage
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Percent(pub f32);

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0)
    }
}

impl From<Percent> for Abv {
    fn from(p: Percent) -> Abv {
        Abv(p.0 / 100.0)
    }
}

impl From<Abv> for Percent {
    fn from(a: Abv) -> Percent {
        Percent(a.0 * 100.0)
    }
}

/// 1 PPG = 8.3454 PKL
/// via the conversion of pounds to kilograms 2.204623
/// and the conversion of gallons to liters 3.7854

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_concentration_conversions() {
        let a = SpecificGravity(1.031);
        let b = Into::<SpecificGravity>::into(Into::<Plato>::into(a));
        println!("  a={a} b={b}");
        assert!(approx_eq!(f32, a.0, b.0, epsilon = 0.0005));

        let a = Plato(7.1);
        let b = Into::<Plato>::into(Into::<SpecificGravity>::into(a));
        println!("  a={a} b={b}");
        assert!(approx_eq!(f32, a.0, b.0, epsilon = 0.001));

        let a = Brix(8.422);
        let b = Into::<Brix>::into(Into::<SpecificGravity>::into(a));
        println!("  a={a} b={b}");
        assert!(approx_eq!(f32, a.0, b.0, epsilon = 0.0005));
    }
}
