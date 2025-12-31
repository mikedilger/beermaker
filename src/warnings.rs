use crate::prelude::*;
use std::fmt;
use std::ops::Range;

/// A warning related to a Process
#[derive(Debug, Clone)]
pub enum Warning {
    /// There is too much chloride, not enough sulfate, and there is no salt
    /// available to correct this.
    SulfateChlorideRatioLow {
        /// The current ratio of sulfate to chloride
        current_ratio: f32,
    },

    /// There is too much sulfate, not enough chloride, and there is no salt
    /// available to correct this.
    SulfateChlorideRatioHigh {
        /// The current ratio of sulfate to chloride
        current_ratio: f32,
    },

    /// None of the fermenters are large enough
    FermentersTooSmall {
        /// How much space is needed in the fermenter
        needed: Liters,
    },

    /// Excess dilution required, as the kettle is not large enough.
    ExcessDilutionRequired {
        /// The dilution ratio required
        dilution_ratio: f32,

        /// The maximum dilution ratio specified in the recipe
        maximum: f32,
    },

    /// Diastatic power of malts is too low
    LowDiastaticPower {
        /// The fraction of base malts the recipe supplies
        fraction_base_malts: f32,
    },

    /// ExcessMalt
    ExcessMalt {
        /// The malt in excess
        malt: Malt,

        /// The percent usage of the malt
        percent: f32,

        /// The maximium recommended percent usage of the malt
        max_recommended_percent: f32,
    },

    /// Boil Kettle is too small
    BoilKettleTooSmall {
        /// amount needed
        needed: Liters,

        /// amount available
        available: Liters,
    },

    /// Too much mash, sparge volume is negative, mash is too thin
    TooMuchMash {
        /// How overfull is the mash?
        overfull: Liters,

        /// What was the requested mash thickness?
        mash_thickness: f32,
    },

    /// Room temperature is unusual
    UnusualRoomTemperature(Celsius),

    /// Infusion temperature is above boiling
    ImpossibleInfusionTemperature(Celsius),

    /// Infusion temperature is unusual
    UnusualInfusionTemperature(Celsius),

    /// Fermentation temperature is unusual
    UnusualFermentationTemperature(Celsius),

    /// Fermentation temperature is too hot for the yeast
    TooHot {
        /// Fermentation temp
        ferment_temp: Celsius,

        /// The max the yeast should be at
        yeast_max: Celsius,
    },

    /// Fermentation temperature is too cold for the yeast
    TooCold {
        /// Fermentation temp
        ferment_temp: Celsius,

        /// The min the yeast should be at
        yeast_min: Celsius,
    },

    /// Yeast cannot tolerate the alcohol
    TooMuchAlcohol {
        /// abv
        abv: Abv,

        /// Yeast max
        yeast_max: Abv,
    },

    /// Mash pH out of range
    MashPhOutOfRange(usize, Ph),

    /// Original Gravity out of range for the style
    OriginalGravityOutOfRange {
        /// Original gravity
        gravity: SpecificGravity,

        /// range acceptable for the style
        range: Range<SpecificGravity>,
    },

    /// Final Gravity out of range for the style
    FinalGravityOutOfRange {
        /// Final gravity
        gravity: SpecificGravity,

        /// range acceptable for the style
        range: Range<SpecificGravity>,
    },

    /// ABV out of range for the style
    AbvOutOfRange {
        /// Alcohol by volume
        abv: Abv,

        /// range acceptable for the style
        range: Range<Abv>,
    },

    /// IBU out of range for the style
    IbuOutOfRange {
        /// IBU bitterness
        ibu: Ibu,

        /// range acceptable for the style
        range: Range<Ibu>,
    },

    /// SRM out of range for the style
    SrmOutOfRange {
        /// SRM color value
        srm: Srm,

        /// range acceptable for the style
        range: Range<Srm>,
    },

    /// Acidity Needed Cancelling
    AcidityNeededCancelling,
}

impl fmt::Display for Warning {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::SulfateChlorideRatioLow { current_ratio } => {
                write!(
                    f,
                    "Sulfate/Chloride ratio is too low ({current_ratio}), and \
                           we have no salt to fix it with."
                )
            }
            Self::SulfateChlorideRatioHigh { current_ratio } => {
                write!(
                    f,
                    "Sulfate/Chloride ratio is too high ({current_ratio}), and \
                           we have no salt to fix it with."
                )
            }
            Self::FermentersTooSmall { needed } => {
                write!(
                    f,
                    "You don't have a fermenter big enough. You need {needed}."
                )
            }
            Self::ExcessDilutionRequired {
                dilution_ratio,
                maximum,
            } => {
                write!(
                    f,
                    "Excess partial-boil dilution is required. The recipe allows \
                           up to {maximum} but we had to use {dilution_ratio}."
                )
            }
            Self::LowDiastaticPower {
                fraction_base_malts,
            } => {
                write!(f, "Not enough base malt: {fraction_base_malts} < 0.7")
            }
            Self::ExcessMalt {
                malt,
                percent,
                max_recommended_percent,
            } => {
                write!(
                    f,
                    "Too much {malt}. You are using {percent}% but the recommended \
                     maximum is {max_recommended_percent}%",
                )
            }
            Self::BoilKettleTooSmall { needed, available } => {
                write!(
                    f,
                    "Kettle is overfull. Kettle can hold {available} but we need {needed}"
                )
            }
            Self::TooMuchMash {
                overfull,
                mash_thickness,
            } => {
                write!(
                    f,
                    "Too much mash by {overfull}. This means the mash thickness of \
                     {mash_thickness} is too thin and not achievable."
                )
            }
            Self::UnusualRoomTemperature(c) => write!(f, "Unusual room temp: {c}"),
            Self::ImpossibleInfusionTemperature(c) => {
                write!(f, "Infusion temp is above boiling!: {c}")
            }
            Self::UnusualInfusionTemperature(c) => write!(f, "Unusual infusion temp: {c}"),
            Self::UnusualFermentationTemperature(c) => write!(f, "Unusual fermentation temp: {c}"),
            Self::TooHot {
                ferment_temp,
                yeast_max,
            } => {
                write!(
                    f,
                    "The fermentation temp of {ferment_temp} is hotter than the yesat maximum of {yeast_max}"
                )
            }
            Self::TooCold {
                ferment_temp,
                yeast_min,
            } => {
                write!(
                    f,
                    "The fermentation temp of {ferment_temp} is colder than the yesat minimum of {yeast_min}"
                )
            }
            Self::TooMuchAlcohol { abv, yeast_max } => {
                write!(
                    f,
                    "The ABV of {abv} is too high, the yeast can only tolerate up to {yeast_max}"
                )
            }
            Self::MashPhOutOfRange(step, ph) => {
                write!(f, "Mash {ph} in step {step} is out of pH range 5.2..5.6")
            }
            Self::OriginalGravityOutOfRange { gravity, range } => {
                write!(
                    f,
                    "Original Gravity {gravity} out of range {}..{} for the style.",
                    range.start, range.end
                )
            }
            Self::FinalGravityOutOfRange { gravity, range } => {
                write!(
                    f,
                    "Final Gravity {gravity} out of range {}..{} for the style.",
                    range.start, range.end
                )
            }
            Self::AbvOutOfRange { abv, range } => {
                write!(
                    f,
                    "ABV {abv} out of range {}..{} for the style.",
                    range.start, range.end
                )
            }
            Self::IbuOutOfRange { ibu, range } => {
                write!(
                    f,
                    "IBU {ibu} out of range {}..{} for the style.",
                    range.start, range.end
                )
            }
            Self::SrmOutOfRange { srm, range } => {
                write!(
                    f,
                    "SRM {srm} out of range {}..{} for the style.",
                    range.start, range.end
                )
            }
            Self::AcidityNeededCancelling => {
                write!(
                    f,
                    "Acidity needed cancelling with Baking Soda, but the acidity was not required \
                     in the first place. Please adjust the recipe."
                )
            }
        }
    }
}

impl Warning {
    /// If this warning is an error.
    ///
    /// Errors mean the process cannot work. Warnings just mean that the output
    /// might not be as great as it could be.
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(
            *self,
            Self::FermentersTooSmall { .. }
                | Self::BoilKettleTooSmall { .. }
                | Self::TooMuchMash { .. }
                | Self::ImpossibleInfusionTemperature(_)
        )
    }
}
