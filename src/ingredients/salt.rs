use serde::{Deserialize, Serialize};
use std::fmt;

/// A type of Salt
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(clippy::doc_markdown)]
pub enum Salt {
    /// Gypsum (CaSO4), Calcium Sulfate
    Gypsum,

    /// Epsom Salt (MgSO4*7H2O), Magnesium Sulfate
    Epsom,

    /// Table Salt (NaCl), no iodine, Sodium Chloride
    TableSalt,

    /// Calcium chloride (dihydrate) (CaCl*2H2O)
    CalciumChloride,

    /// Magnesium Chloride (MgCl*6H20)
    MagnesiumChloride,

    /// Baking Soda (NaHCO3)
    BakingSoda,

    /// Slaked Lime (CaOH2)
    SlakedLime,

    /// Caustic Soda (lye) (NaOH)
    CausticSoda,

    /// Sodium Sulfate (Na2SO4)
    SodiumSulfate,
}

impl fmt::Display for Salt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Salt::Gypsum => write!(f, "[Gypsum]"),
            Salt::Epsom => write!(f, "[Epsom]"),
            Salt::TableSalt => write!(f, "[Table Salt]"),
            Salt::CalciumChloride => write!(f, "[Calcium Chloride]"),
            Salt::MagnesiumChloride => write!(f, "[Magnesium Chloride]"),
            Salt::BakingSoda => write!(f, "[Baking Soda]"),
            Salt::SlakedLime => write!(f, "[Slaked Lime]"),
            Salt::CausticSoda => write!(f, "[Caustic Soda]"),
            Salt::SodiumSulfate => write!(f, "[Sodium Sulfate]"),
        }
    }
}

impl Salt {
    /// What ions does this salt have?
    #[must_use]
    pub fn ions(&self) -> Vec<Ion> {
        match *self {
            Salt::Gypsum => vec![Ion::Calcium, Ion::Sulfate, Ion::Water, Ion::Water],
            Salt::Epsom => vec![
                Ion::Magnesium,
                Ion::Sulfate,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
            ],
            Salt::TableSalt => vec![Ion::Sodium, Ion::Chloride],
            Salt::CalciumChloride => vec![
                Ion::Calcium,
                Ion::Chloride,
                Ion::Chloride,
                Ion::Water,
                Ion::Water,
            ],
            Salt::MagnesiumChloride => vec![
                Ion::Magnesium,
                Ion::Chloride,
                Ion::Chloride,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
                Ion::Water,
            ],
            Salt::BakingSoda => vec![Ion::Sodium, Ion::Bicarbonate],
            Salt::SlakedLime => vec![Ion::Calcium, Ion::Hydroxide, Ion::Hydroxide],
            Salt::CausticSoda => vec![Ion::Sodium, Ion::Hydroxide],
            Salt::SodiumSulfate => vec![Ion::Sodium, Ion::Sodium, Ion::Sulfate],
        }
    }

    /// What fraction (by weight) of this salt is the ion
    #[must_use]
    pub fn ion_fraction(&self, ion: Ion) -> f32 {
        let mut numerator: f32 = 0.0;
        let mut denominator: f32 = 0.0;
        for i in self.ions() {
            denominator += ion.molecular_weight();
            if i == ion {
                numerator += ion.molecular_weight();
            }
        }
        numerator / denominator
    }
}

/// An ion
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Ion {
    /// H+
    Hydrogen,

    /// OH-
    Hydroxide,

    /// H2O
    Water,

    /// HCO3
    Bicarbonate,

    /// Na+2
    Sodium,

    /// Mg+2
    Magnesium,

    /// SO4-2
    Sulfate,

    /// Cl-2
    Chloride,

    /// Ca+2
    Calcium,
}

impl Ion {
    /// The molecular weight of the ion
    #[must_use]
    pub fn molecular_weight(self) -> f32 {
        match self {
            Self::Hydrogen => 1.008,
            Self::Hydroxide => {
                1.008 + 15.999 // OH
            }
            Self::Water => {
                1.008 * 2.0 + 15.999 // H2O
            }
            Self::Bicarbonate => {
                1.008 + 12.011 + 15.999 * 3.0 // HCO3
            }
            Self::Sodium => 22.990,
            Self::Magnesium => 24.305,
            Self::Sulfate => {
                32.06 + 15.999 * 4.0 // SO4
            }
            Self::Chloride => 35.45,
            Self::Calcium => 40.078,
        }
    }
}
