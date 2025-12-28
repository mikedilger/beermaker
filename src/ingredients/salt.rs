use crate::chemistry::Ion;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A type of Salt
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    #[rustfmt::skip]
    pub fn ions(&self) -> &[Ion] {
        match *self {
            Salt::Gypsum => &[
                Ion::Calcium,
                Ion::Sulfate,
                Ion::Water,
                Ion::Water,
            ],
            Salt::Epsom => &[
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
            Salt::TableSalt => &[
                Ion::Sodium,
                Ion::Chloride,
            ],
            Salt::CalciumChloride => &[
                Ion::Calcium,
                Ion::Chloride,
                Ion::Chloride,
                Ion::Water,
                Ion::Water,
            ],
            Salt::MagnesiumChloride => &[
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
            Salt::BakingSoda => &[
                Ion::Sodium,
                Ion::Bicarbonate,
            ],
            Salt::SlakedLime => &[
                Ion::Calcium,
                Ion::Hydroxide,
                Ion::Hydroxide,
            ],
            Salt::CausticSoda => &[
                Ion::Sodium,
                Ion::Hydroxide,
            ],
            Salt::SodiumSulfate => &[
                Ion::Sodium,
                Ion::Sodium,
                Ion::Sulfate,
            ],
        }
    }

    /// Molecular weight
    #[must_use]
    pub fn molecular_weight(&self) -> f32 {
        let mut weight: f32 = 0.0;
        for ion in self.ions() {
            weight += ion.molecular_weight();
        }
        weight
    }

    /// What fraction (by weight) of this salt is the ion
    // this handles the fact that the ion may appear more than once.
    #[must_use]
    pub fn ion_fraction(&self, target_ion: Ion) -> f32 {
        let mut numerator: f32 = 0.0;
        let mut denominator: f32 = 0.0;
        for ion in self.ions().to_owned().drain(..) {
            denominator += ion.molecular_weight();
            if ion == target_ion {
                numerator += ion.molecular_weight();
            }
        }
        numerator / denominator
    }
}
