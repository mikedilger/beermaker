use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A category of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaltCategory {
    /// Base malt with diastatic power
    Base,

    /// Crystal malt or Caramel malt
    Crystal,

    /// Roasted malt
    Roasted,

    /// Special malt
    Special,
}

/// A pH category of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MaltAcidCategory {
    /// No acid effect
    None,

    /// Crystal malts
    Crystal,

    /// Light malts
    Light,

    /// Dark malts
    Dark,

    /// Acidulated malt
    Acidulated,
}

/// A type of Malt
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Malt {
    /// Gladfield base malt, German Pilsner Malt
    GladfieldGermanPilsner,

    /// Gladfield Wheat
    GladfieldWheat,

    /// Weyermann Munich Type I
    WeyermannMunich1,

    /// Weyermann Munich Type II (Dark)
    WeyermannMunich2,

    /// Weyermann Vienna
    WeyermannVienna,

    /// Weyermann Caramunich Type II
    WeyermannCaramunich2,

    /// Weyermann Acidulated
    WeyermannAcidulated,

    /// Weyermann Melanoidin
    WeyermannMelanoidin,

    /// Weyermann Wheat Malt (Pale)
    WeyermannWheatPale,

    /// Oat Hulls
    OatHulls,

    /// Rice Hulls
    RiceHulls,
}

impl Malt {
    /// Category of malt
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn category(&self) -> MaltCategory {
        match *self {
            Malt::GladfieldGermanPilsner => MaltCategory::Base,
            Malt::GladfieldWheat => MaltCategory::Base,
            Malt::WeyermannMunich1 => MaltCategory::Base,
            Malt::WeyermannMunich2 => MaltCategory::Base,
            Malt::WeyermannVienna => MaltCategory::Base,
            Malt::WeyermannCaramunich2 => MaltCategory::Crystal,
            Malt::WeyermannAcidulated => MaltCategory::Special,
            Malt::WeyermannMelanoidin => MaltCategory::Crystal,
            Malt::WeyermannWheatPale => MaltCategory::Base,
            Malt::OatHulls => MaltCategory::Special,
            Malt::RiceHulls => MaltCategory::Special,
        }
    }

    /// Acid Category of malt
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn acid_category(&self) -> MaltAcidCategory {
        match *self {
            Malt::GladfieldGermanPilsner => MaltAcidCategory::Light,
            Malt::GladfieldWheat => MaltAcidCategory::Light,
            Malt::WeyermannMunich1 => MaltAcidCategory::Light,
            Malt::WeyermannMunich2 => MaltAcidCategory::Light,
            Malt::WeyermannVienna => MaltAcidCategory::Light,
            Malt::WeyermannCaramunich2 => MaltAcidCategory::Crystal,
            Malt::WeyermannAcidulated => MaltAcidCategory::Acidulated,
            Malt::WeyermannMelanoidin => MaltAcidCategory::Crystal,
            Malt::WeyermannWheatPale => MaltAcidCategory::Light,
            Malt::OatHulls => MaltAcidCategory::None,
            Malt::RiceHulls => MaltAcidCategory::None,
        }
    }

    /// Range of wort color provided
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn ebc_range(&self) -> (Ebc, Ebc) {
        match *self {
            Malt::GladfieldGermanPilsner => (Ebc(3.0), Ebc(4.5)), // 2.03
            Malt::GladfieldWheat => (Ebc(3.2), Ebc(4.2)),
            Malt::WeyermannMunich1 => (Ebc(12.0), Ebc(18.0)), // 5.0 - 7.2
            Malt::WeyermannMunich2 => (Ebc(20.0), Ebc(25.0)), // 8.0 - 9.9
            Malt::WeyermannVienna => (Ebc(6.0), Ebc(9.0)),    // 2.7 - 3.8
            Malt::WeyermannCaramunich2 => (Ebc(110.0), Ebc(130.0)), // 41.9 - 49.5
            Malt::WeyermannAcidulated => (Ebc(2.0), Ebc(5.0)), // 1.2 - 2.3
            Malt::WeyermannMelanoidin => (Ebc(60.0), Ebc(80.0)), // 23.1 - 30.6
            Malt::WeyermannWheatPale => (Ebc(3.0), Ebc(5.0)), // 1.6 - 2.3
            Malt::OatHulls => (Ebc(0.0), Ebc(0.0)),
            Malt::RiceHulls => (Ebc(0.0), Ebc(0.0)),
        }
    }

    /// Range of wort color provided
    #[must_use]
    pub fn ebc(&self) -> Ebc {
        (self.ebc_range().0 + self.ebc_range().1) / 2.0
    }

    /// Maximum amount you should have in a normal beer recipe
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn recommended_max_percent(&self) -> f32 {
        match *self {
            Malt::GladfieldGermanPilsner => 100.0,
            Malt::GladfieldWheat => 70.0,
            Malt::WeyermannMunich1 => 100.0,
            Malt::WeyermannMunich2 => 100.0,
            Malt::WeyermannVienna => 100.0,
            Malt::WeyermannCaramunich2 => 10.0,
            Malt::WeyermannAcidulated => 5.0, // 10.0 in sour beers
            Malt::WeyermannMelanoidin => 20.0,
            Malt::WeyermannWheatPale => 80.0,
            Malt::OatHulls => 5.0,
            Malt::RiceHulls => 8.0, // 3-8%
        }
    }

    /// Points per pound per gallon
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn ppg(&self) -> f32 {
        match *self {
            Malt::GladfieldGermanPilsner => 36.3,
            Malt::GladfieldWheat => 38.8,
            Malt::WeyermannMunich1 => 38.0,
            Malt::WeyermannMunich2 => 37.0,
            Malt::WeyermannVienna => 37.0,
            Malt::WeyermannCaramunich2 => 34.0,
            Malt::WeyermannAcidulated => 27.0,
            Malt::WeyermannMelanoidin => 34.5,
            Malt::WeyermannWheatPale => 36.0,
            Malt::OatHulls => 0.0,
            Malt::RiceHulls => 0.0,
        }
    }
}

impl fmt::Display for Malt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Malt::GladfieldGermanPilsner => write!(f, "[Gladfield German Pilsner Malt]"),
            Malt::GladfieldWheat => write!(f, "[Gladfield Wheat Malt]"),
            Malt::WeyermannMunich1 => write!(f, "[Weyermann Munich Malt I]"),
            Malt::WeyermannMunich2 => write!(f, "[Weyermann Munich Malt II]"),
            Malt::WeyermannVienna => write!(f, "[Weyermann Vienna Malt]"),
            Malt::WeyermannCaramunich2 => write!(f, "[Weyermann CaraMunich Malt II]"),
            Malt::WeyermannAcidulated => write!(f, "[Weyermann Acidulated Malt]"),
            Malt::WeyermannMelanoidin => write!(f, "[Weyermann Melanoidin Malt]"),
            Malt::WeyermannWheatPale => write!(f, "[Weyermann Wheat Malt Pale]"),
            Malt::OatHulls => write!(f, "[Oat Hulls]"),
            Malt::RiceHulls => write!(f, "[Rice Hulls]"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::units::color::Lovabond;
    use float_cmp::approx_eq;

    #[test]
    fn test_known_malt_lovabond_values() {
        let (low, high) = Malt::WeyermannMunich2.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            8.0,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            9.9,
            epsilon = 0.2
        ));

        let (low, high) = Malt::WeyermannCaramunich2.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            41.9,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            49.5,
            epsilon = 0.3
        ));

        let (low, high) = Malt::WeyermannAcidulated.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            1.2,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            2.3,
            epsilon = 0.2
        ));

        let (low, high) = Malt::WeyermannMelanoidin.ebc_range();
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(low).0,
            23.1,
            epsilon = 0.2
        ));
        assert!(approx_eq!(
            f32,
            Into::<Lovabond>::into(high).0,
            30.6,
            epsilon = 0.2
        ));
    }
}
