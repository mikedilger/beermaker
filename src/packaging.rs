use crate::ingredients::Sugar;
use crate::units::prelude::*;
use serde::{Deserialize, Serialize};

/// The kind of packaging that is used
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Packaging {
    /// Packaged in bottles of given size, primed with the given sugar
    Bottle(Liters, Sugar),

    /// Packaged in a keg if given size
    Keg(Liters),
}
