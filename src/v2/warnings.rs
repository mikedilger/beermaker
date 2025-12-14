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
        needed: Liters
    },
}


impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            Self::ChlorideSulfateRatioLow { current_ratio } => {
                write!(f, "Chloride-Sulfate ratio is too low ({current_ratio}), and \
                           we have no salt to fix it with.")
            },
            Self::ChlorideSulfateRatioHigh { current_ratio } => {
                write!(f, "Chloride-Sulfate ratio is too high ({current_ratio}), and \
                           we have no salt to fix it with.")
            },
            Self::FermentersTooSmall { needed } => {
                write!(f, "You don't have a fermenter big enough. You need {needed}.")
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
            _ => false,
        }
    }
}
