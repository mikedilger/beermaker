use crate::mash::MashRest;
use crate::prelude::*;
use crate::style::Style;
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Recipe for beer
// Recipe2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe2 {
    /// The name of the recipe
    pub name: String,

    /// The style of beer
    pub style: Style,

    /// Water profile requirements, Cl to SO4 ratio
    pub chloride_sulfate_ratio_range: Option<Range<f32>>,

    // TBD water alkalinity requirements

    // TBD water hardness requirements

    // TBD pH requirements

    /// The malts that will be mashed, in proportion to all malts and sugars
    /// by weight. The actual weights are calculated.
    pub malts: Vec<MaltProportion>,

    /// The mash rests.
    ///
    /// For single infusion, just list that one.
    pub mash_rests: Vec<MashRest>,

    /// Final mash thickness in liters of liquor per kilogram of grist.
    /// Typical range is 2.4 - 3.1.
    pub mash_thickness: f32,

    /// The sugars added after mashing, in proportion to all malts and sugars
    /// by weight. The actual weights are calculated.
    pub sugars: Vec<SugarProportion>,

    /// Original gravity target
    pub original_gravity: SpecificGravity,

    /// The IBU target
    pub ibu: Ibu,

    /// The proportional hops additions added during the boil
    pub hops: Vec<HopsProportion>,

    /// Boil length
    pub boil_length: Minutes,

    /// Whether or not to use a fining agent
    pub fining_desired: bool,

    /// The yeast to ferment with
    pub yeast: Yeast,

    /// If a partial boil dilution is allowable. If false, and the wort
    /// can't fit into the boil kettle, an error will be generated.
    pub allow_partial_boil_dilution: bool,

    /// The temperature to ferment at
    pub ferment_temperature: Celsius,
}

impl Recipe2 {
    // NOTE: we order these functions based on dependencies.
    // Each function only depends on things above it.

    /// Ferment losses, as a fraction
    pub(crate) fn ferment_loss_fraction(&self) -> f32 {

        // Start out based on gravity. More sugar leads to
        // more yeast
        let mut fraction: f32 = (self.original_gravity.0 - 1.0) * 2.2;

        // Next, reduce this under low flocculation conditions
        if ! self.fining_desired {
            fraction = fraction *  match self.yeast.flocculation() {
                Flocculation::Low => 0.94,
                Flocculation::LowMedium => 0.97,
                _ => 1.0,
            };
        }

        // TBD:
        // belgian yeast strains create more krausen
        // hot break and cold break mean more solids left behind
        // more for heavy dry hopping

        fraction
    }
}
