use crate::mash::MashRest;
use crate::prelude::*;
use crate::style::Style;
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Recipe for beer
// Recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// The name of the recipe
    pub name: String,

    /// The style of beer
    pub style: Style,

    /// Water profile requirements, SO4 to Cl ratio
    pub sulfate_chloride_ratio_range: Range<f32>,

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

    /// How much dilution to allow when moving from boil kettle to
    /// fermenter. This is typically 1.0 (meaning none), but this can
    /// allow a larger batch to be brewed from a smaller kettle.
    ///
    /// This is expressed as a fraction, so 1.2 means up to 20% dilution.
    ///
    /// If this is too small, and the wort cannot fit in the kettle, an
    /// error (Warning) will be generated.
    ///
    /// Keep in mind that partial boils are done at higher gravity in order
    /// to hit the target original gravity, and this will affect the beer.
    pub max_partial_boil_dilution: f32,

    /// The temperature to ferment at
    pub ferment_temperature: Celsius,

    /// Target ABV. ABV is mainly determined by the original gravity and
    /// the attenuation of the yeast. If you set this, and the natural ABV
    /// is to high, the product will be diluted up to the maximum
    /// post ferment dilution level to try to achieve this ABV. Note that
    /// this is not the only way to lower ABV, and usually other techniques
    /// are used in low alcohol beers.
    pub target_abv: Option<Abv>,

    /// Maximum post-fermentation dilution to allow in persuit of the target
    /// ABV.
    ///
    /// This is expressed as a fraction, so 1.2 means up to 20% dilution.
    pub max_post_ferment_dilution: f32,

    /// Custom steps
    ///
    /// Custom steps will be printed per-section at the top of the section.
    pub custom_steps: Option<Steps>,
}

impl Recipe {
    // NOTE: we order these functions based on dependencies.
    // Each function only depends on things above it.

    /// Ferment losses, as a fraction
    pub(crate) fn ferment_loss_fraction(&self) -> f32 {
        // Start out based on gravity. More sugar leads to
        // more yeast
        let mut fraction: f32 = (self.original_gravity.0 - 1.0) * 2.2;

        // Next, reduce this under low flocculation conditions
        if !self.fining_desired {
            fraction *= match self.yeast.flocculation() {
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

    /// Estimated length of the fermentation
    ///
    /// Warning, this calculation is completely made up and not validated
    /// against anything.  But most calculators, websites, etc, just say
    /// something even less data-driven like "7 days".
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn fermentation_time(&self) -> Days {
        // Higher temperature ferments run faster
        // This fails however if you ferment at or above 35 C.
        let base = (35.0 - self.ferment_temperature.0) / 2.0;

        // If you ferment lower in the temp range of the given
        // yeast, you slow down the fermentation.
        let temp_range = self.yeast.temp_range();
        let temp_range_adjuster = (self.ferment_temperature.0 - temp_range.start.0)
            / (temp_range.end.0 - temp_range.start.0);
        let temp_range_multiplier = 1.2 - 0.4 * temp_range_adjuster;

        // TODO: only include if the style requires clearing?
        let floc_multiplier = match self.yeast.flocculation() {
            Flocculation::Low => 1.2,
            Flocculation::LowMedium => 1.1,
            Flocculation::Medium => 1.0,
            Flocculation::MediumHigh => 0.9,
            Flocculation::High => 0.8,
            Flocculation::VeryHigh => 0.75,
        };

        Days((base * temp_range_multiplier * floc_multiplier) as usize)
    }

    /// Estimated FAN requirement of yeast
    #[must_use]
    pub fn fan_requirement_of_yeast(&self) -> Ppm {
        self.yeast.fan_requirement() * ((self.original_gravity.0 - 1.0) / 0.050)
    }

    /// Diacetyl rest temperature
    #[must_use]
    pub fn diacetyl_rest_temperature(&self) -> Celsius {
        Celsius(self.ferment_temperature.0 * (5.0 / 6.0) + (20.0 / 3.0))
    }
}
