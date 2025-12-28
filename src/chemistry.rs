use crate::prelude::Ppm;
use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;

/// Chemical Element
// just the ones involved in mash chemistry
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, EnumIter)]
pub enum Element {
    Hydrogen,
    Carbon,
    Oxygen,
    Sodium,
    Magnesium,
    Sulfur,
    Chlorine,
    Calcium,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Hydrogen => write!(f, "H"),
            Self::Carbon => write!(f, "C"),
            Self::Oxygen => write!(f, "O"),
            Self::Sodium => write!(f, "Na"),
            Self::Magnesium => write!(f, "Mg"),
            Self::Sulfur => write!(f, "S"),
            Self::Chlorine => write!(f, "Cl"),
            Self::Calcium => write!(f, "Ca"),
        }
    }
}

impl Element {
    /// Atomic weight, relative isotopic average, dimensionless
    #[must_use]
    pub fn atomic_weight(&self) -> f32 {
        match *self {
            Self::Hydrogen => 1.008_0,
            Self::Carbon => 12.011,
            Self::Oxygen => 15.999,
            Self::Sodium => 22.990,
            Self::Magnesium => 24.305,
            Self::Sulfur => 32.06,
            Self::Chlorine => 35.45,
            Self::Calcium => 40.078,
        }
    }
}

/// Ion
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Ion {
    /// H+
    Hydrogen,

    /// OH-
    Hydroxide,

    /// Water isn't strictly an ion, but a lot of salts are hydrated
    /// and this is as good a place as any to put it.
    Water,

    /// HCO3-
    Bicarbonate,

    /// Na+2
    Sodium,

    /// Mg+2
    Magnesium,

    /// SO4-2
    Sulfate,

    /// Ch-2
    Chloride,

    /// Ca+2
    Calcium,
}

impl fmt::Display for Ion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Hydrogen => write!(f, "H⁺"),
            Self::Hydroxide => write!(f, "OH⁻"),
            Self::Water => write!(f, "H₂O"),
            Self::Bicarbonate => write!(f, "HCO₃⁻"),
            Self::Sodium => write!(f, "Na⁺²"),
            Self::Magnesium => write!(f, "Mg⁺²"),
            Self::Sulfate => write!(f, "SO₄⁻²"),
            Self::Chloride => write!(f, "Cl⁻²"),
            Self::Calcium => write!(f, "Ca⁺²"),
        }
    }
}

impl Ion {
    /// Atomic charge
    #[must_use]
    pub fn charge(&self) -> i8 {
        match *self {
            Self::Hydrogen => 1,
            Self::Hydroxide => -1,
            Self::Water => 0,
            Self::Bicarbonate => -1,
            Self::Sodium => 2,
            Self::Magnesium => 2,
            Self::Sulfate => -2,
            Self::Chloride => -2,
            Self::Calcium => 2,
        }
    }

    /// Atomic components
    #[must_use]
    #[rustfmt::skip]
    pub fn atoms(&self) -> &[Element] {
        match *self {
            Self::Hydrogen => &[
                Element::Hydrogen
            ],
            Self::Hydroxide => &[
                Element::Oxygen,
                Element::Hydrogen
            ],
            Self::Water => &[
                Element::Hydrogen,
                Element::Hydrogen,
                Element::Oxygen
            ],
            Self::Bicarbonate => &[
                Element::Hydrogen,
                Element::Carbon,
                Element::Oxygen,
                Element::Oxygen,
                Element::Oxygen
            ],
            Self::Sodium => &[
                Element::Sodium
            ],
            Self::Magnesium => &[
                Element::Magnesium
            ],
            Self::Sulfate => &[
                Element::Sulfur,
                Element::Oxygen,
                Element::Oxygen,
                Element::Oxygen,
                Element::Oxygen
            ],
            Self::Chloride => &[
                Element::Chlorine
            ],
            Self::Calcium => &[
                Element::Calcium
            ],
        }
    }

    /// Molecular weight
    #[must_use]
    pub fn molecular_weight(&self) -> f32 {
        let mut weight: f32 = 0.0;
        for atom in self.atoms() {
            weight += atom.atomic_weight();
        }
        weight
    }
}

/// Milli-equivalents per liter
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct MEqL(pub f32);

impl fmt::Display for MEqL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} mEq/L", self.0)
    }
}

impl MEqL {
    /// Convert Ppm(=mg/L) to mEq/L
    #[must_use]
    pub fn from_ppm(ppm: Ppm, ion: Ion) -> MEqL {
        let valence = f32::from(ion.charge().abs());
        let factor = valence / ion.molecular_weight();
        MEqL(ppm.0 * factor)
    }

    /// Convert mEq/L to Ppm(=mg/L)
    #[must_use]
    pub fn to_ppm(self, ion: Ion) -> Ppm {
        let valence = f32::from(ion.charge().abs());
        let factor = valence / ion.molecular_weight();
        Ppm(self.0 / factor)
    }
}
