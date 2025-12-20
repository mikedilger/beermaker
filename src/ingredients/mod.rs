use serde::{Deserialize, Serialize};

mod water;
pub use water::{WaterAdjustment, WaterProfile};

mod salt;
pub use salt::{Ion, Salt};

mod acid;
pub use acid::Acid;

mod malt;
pub use malt::{Malt, MaltAcidCategory, MaltCategory};

mod sugar;
pub use sugar::Sugar;

mod hops;
pub use hops::Hops;

mod yeast;
pub use yeast::{Flocculation, Yeast};

use crate::units::prelude::*;

/// A dose of Water
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WaterDose {
    /// How much
    pub volume: Liters,

    /// At what temperature
    pub temp: Celsius,
}

/// A concentration of a Salt
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SaltConcentration {
    /// Which salt to use
    pub salt: Salt,

    /// The concentration
    pub ppm: Ppm,
}

/// A dose of Salt
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SaltDose {
    /// Which salt to use
    pub salt: Salt,

    /// How many milligrams
    pub mg: Milligrams,
}

/// A concentration of Ions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IonConcentration {
    /// Which ion
    pub ion: Ion,

    /// The concentration
    pub ppm: Ppm,
}

/// A concentration of an Acid/Base
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AcidConcentration {
    /// Which acid,
    pub acid: Acid,

    /// The concentration
    pub ppm: Ppm,
}

/// A dose of Acid/Base
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AcidDose {
    /// Which acid/base to use
    pub acid: Acid,

    /// How many milligrams
    pub mg: Milligrams,
}

/// A proportion of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MaltProportion {
    /// Which malt to use
    pub malt: Malt,

    /// Proportion to use in relation to all other malts and sugars
    pub proportion: f32,
}

/// A dose of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MaltDose {
    /// Which malt to use
    pub malt: Malt,

    /// How much
    pub weight: Kilograms,
}

/// A proportion of Sugar
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SugarProportion {
    /// Which sugar to use
    pub sugar: Sugar,

    /// Proportion to use in relation to all other malts and sugars
    pub proportion: f32,
}

/// A dose of Sugar
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SugarDose {
    /// Which sugar to use
    pub sugar: Sugar,

    /// How much
    pub weight: Kilograms,
}

/// A proportion of Hops
///
/// where the sum of proportions provides the recipe specified IBUs
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HopsProportion {
    /// Which hops to use
    pub hops: Hops,

    /// Proportion to use in relation to all other hops
    pub proportion: f32,

    /// How long before the end of the boil to add them
    pub timing: Minutes,
}

/// A dose of Hops
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HopsDose {
    /// Which hops to use
    pub hops: Hops,

    /// How much
    pub weight: Grams,

    /// How long before the end of the boil to add them
    pub timing: Minutes,
}
