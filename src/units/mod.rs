use std::fmt;

/// Weight
pub mod weight;

/// Volume
pub mod volume;

/// Temperature
pub mod temperature;

/// Color (of beer)
pub mod color;

/// Concentration (of sugar or of ions)
pub mod concentration;

/// Water hardness
pub mod hardness;

/// Water alkalinity
pub mod alkalinity;

/// Time
pub mod time;

/// Prelude, for importing all of the units
pub mod prelude {
    pub use super::alkalinity::*;
    pub use super::color::*;
    pub use super::concentration::*;
    pub use super::hardness::*;
    pub use super::temperature::*;
    pub use super::time::*;
    pub use super::volume::*;
    pub use super::weight::*;
    pub use super::{Ibu, Ph};
}

use serde::{Deserialize, Serialize};

/// Acidity in pH
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Ph(pub f32);

impl fmt::Display for Ph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pH {:.1}", self.0)
    }
}

/// Bitterness in IBU
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Ibu(pub f32);

impl fmt::Display for Ibu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} IBU", self.0)
    }
}
