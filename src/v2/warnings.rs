use crate::prelude::*;
use std::fmt;

/// A warning related to a Process
#[derive(Debug, Clone, Copy)]
pub enum Warning {
    /// There is too much sulfate, not enough chloride, and there is no salt
    /// available to correct this.
    ChlorideSulfateRatioLow {
        /// The current ratio of chloride to sulfate
        current_ratio: f32,
    },

    /// There is too much chloride, not enough sulfate, and there is no salt
    /// available to correct this.
    ChlorideSulfateRatioHigh {
        /// The current ratio of chloride to sulfate
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
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Self::ChlorideSulfateRatioLow { current_ratio } => {
                write!(
                    f,
                    "Chloride-Sulfate ratio is too low ({current_ratio}), and \
                           we have no salt to fix it with."
                )
            }
            Self::ChlorideSulfateRatioHigh { current_ratio } => {
                write!(
                    f,
                    "Chloride-Sulfate ratio is too high ({current_ratio}), and \
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
        }
    }
}

impl Warning {
    /// If this warning is an error.
    ///
    /// Errors mean the process cannot work. Warnings just mean that the output
    /// might not be as great as it could be.
    pub fn is_error(&self) -> bool {
        match *self {
            Self::FermentersTooSmall { .. } => true,
            Self::BoilKettleTooSmall { .. } => true,
            Self::TooMuchMash { .. } => true,
            _ => false,
        }
    }
}
