use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum Malt {
    /// Dingemans Special-B
    DingemansSpecialB,

    /// Gladfield Ale Malt
    GladfieldAle,

    /// Gladfield American Ale Malt
    GladfieldAmericanAle,

    /// Gladfield Biscuit Malt
    GladfieldBiscuit,

    /// Gladfield Brown Malt
    GladfieldBrown,

    /// Gladfield Crystal Dark
    GladfieldCrystalDark,

    /// Gladfield base malt, German Pilsner Malt
    GladfieldGermanPilsner,

    /// Gladfield Light Lager Malt
    GladfieldLagerLight,

    /// Gladfield Munich
    GladfieldMunich,

    /// Gladfield Pilsner Malt
    GladfieldPilsner,

    /// Gladfield Shepherds Delight
    GladfieldShepherdsDelight,

    /// Gladfield Vienna
    GladfieldVienna,

    /// Gladfield Wheat
    GladfieldWheat,

    /// Oat Hulls
    OatHulls,

    /// Rice Hulls
    RiceHulls,

    /// Weyermann Acidulated
    WeyermannAcidulated,

    /// Weyermann Carafa Special II
    WeyermannCarafaSpecial2,

    /// Weyermann Caramunich Type II
    WeyermannCaramunich2,

    /// Weyermann Melanoidin
    WeyermannMelanoidin,

    /// Weyermann Munich Type I
    WeyermannMunich1,

    /// Weyermann Munich Type II (Dark)
    WeyermannMunich2,

    /// Weyermann Pilsner
    WeyermannPilsner,

    /// Weyermann Vienna
    WeyermannVienna,

    /// Weyermann Wheat Malt (Pale)
    WeyermannWheatPale,
}

impl Malt {
    /// Category of malt
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn category(&self) -> MaltCategory {
        match *self {
            Malt::DingemansSpecialB => MaltCategory::Crystal,
            Malt::GladfieldAle => MaltCategory::Base,
            Malt::GladfieldAmericanAle => MaltCategory::Base,
            Malt::GladfieldBiscuit => MaltCategory::Roasted,
            Malt::GladfieldBrown => MaltCategory::Roasted,
            Malt::GladfieldCrystalDark => MaltCategory::Crystal,
            Malt::GladfieldGermanPilsner => MaltCategory::Base,
            Malt::GladfieldLagerLight => MaltCategory::Base,
            Malt::GladfieldMunich => MaltCategory::Base,
            Malt::GladfieldPilsner => MaltCategory::Base,
            Malt::GladfieldShepherdsDelight => MaltCategory::Special,
            Malt::GladfieldVienna => MaltCategory::Base,
            Malt::GladfieldWheat => MaltCategory::Base,
            Malt::OatHulls => MaltCategory::Special,
            Malt::RiceHulls => MaltCategory::Special,
            Malt::WeyermannAcidulated => MaltCategory::Special,
            Malt::WeyermannCarafaSpecial2 => MaltCategory::Roasted,
            Malt::WeyermannCaramunich2 => MaltCategory::Crystal,
            Malt::WeyermannMelanoidin => MaltCategory::Crystal,
            Malt::WeyermannMunich1 => MaltCategory::Base,
            Malt::WeyermannMunich2 => MaltCategory::Base,
            Malt::WeyermannPilsner => MaltCategory::Base,
            Malt::WeyermannVienna => MaltCategory::Base,
            Malt::WeyermannWheatPale => MaltCategory::Base,
        }
    }

    /// Acid Category of malt
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn acid_category(&self) -> MaltAcidCategory {
        match *self {
            Malt::DingemansSpecialB => MaltAcidCategory::Crystal,
            Malt::GladfieldAle => MaltAcidCategory::Light,
            Malt::GladfieldAmericanAle => MaltAcidCategory::Light,
            Malt::GladfieldBiscuit => MaltAcidCategory::Crystal,
            Malt::GladfieldBrown => MaltAcidCategory::Dark,
            Malt::GladfieldCrystalDark => MaltAcidCategory::Crystal,
            Malt::GladfieldGermanPilsner => MaltAcidCategory::Light,
            Malt::GladfieldLagerLight => MaltAcidCategory::Light,
            Malt::GladfieldMunich => MaltAcidCategory::Light,
            Malt::GladfieldPilsner => MaltAcidCategory::Light,
            Malt::GladfieldShepherdsDelight => MaltAcidCategory::Dark,
            Malt::GladfieldVienna => MaltAcidCategory::Light,
            Malt::GladfieldWheat => MaltAcidCategory::Light,
            Malt::OatHulls => MaltAcidCategory::None,
            Malt::RiceHulls => MaltAcidCategory::None,
            Malt::WeyermannAcidulated => MaltAcidCategory::Acidulated,
            Malt::WeyermannCarafaSpecial2 => MaltAcidCategory::Dark,
            Malt::WeyermannCaramunich2 => MaltAcidCategory::Crystal,
            Malt::WeyermannMelanoidin => MaltAcidCategory::Crystal,
            Malt::WeyermannMunich1 => MaltAcidCategory::Light,
            Malt::WeyermannMunich2 => MaltAcidCategory::Light,
            Malt::WeyermannPilsner => MaltAcidCategory::Light,
            Malt::WeyermannVienna => MaltAcidCategory::Light,
            Malt::WeyermannWheatPale => MaltAcidCategory::Light,
        }
    }

    /// Range of wort color provided
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn ebc_range(&self) -> (Ebc, Ebc) {
        match *self {
            Malt::DingemansSpecialB => (Ebc(300.0), Ebc(300.0)),
            Malt::GladfieldAle => (Ebc(5.0), Ebc(6.0)),
            Malt::GladfieldAmericanAle => (Ebc(4.5), Ebc(5.5)),
            Malt::GladfieldBiscuit => (Ebc(40.0), Ebc(80.0)),
            Malt::GladfieldBrown => (Ebc(150.0), Ebc(200.0)),
            Malt::GladfieldCrystalDark => (Ebc(200.0), Ebc(200.0)),
            Malt::GladfieldGermanPilsner => (Ebc(3.0), Ebc(4.5)), // 2.03
            Malt::GladfieldLagerLight => (Ebc(2.5), Ebc(2.9)),
            Malt::GladfieldMunich => (Ebc(14.0), Ebc(17.0)),
            Malt::GladfieldPilsner => (Ebc(3.2), Ebc(4.0)),
            Malt::GladfieldShepherdsDelight => (Ebc(320.), Ebc(320.)),
            Malt::GladfieldVienna => (Ebc(6.5), Ebc(8.5)),
            Malt::GladfieldWheat => (Ebc(3.2), Ebc(4.2)),
            Malt::OatHulls => (Ebc(0.0), Ebc(0.0)),
            Malt::RiceHulls => (Ebc(0.0), Ebc(0.0)),
            Malt::WeyermannAcidulated => (Ebc(2.0), Ebc(5.0)), // 1.2 - 2.3
            Malt::WeyermannCarafaSpecial2 => (Ebc(1100.0), Ebc(1200.0)),
            Malt::WeyermannCaramunich2 => (Ebc(110.0), Ebc(130.0)), // 41.9 - 49.5
            Malt::WeyermannMelanoidin => (Ebc(60.0), Ebc(80.0)),    // 23.1 - 30.6
            Malt::WeyermannMunich1 => (Ebc(12.0), Ebc(18.0)),       // 5.0 - 7.2
            Malt::WeyermannMunich2 => (Ebc(20.0), Ebc(25.0)),       // 8.0 - 9.9
            Malt::WeyermannPilsner => (Ebc(2.5), Ebc(4.5)),
            Malt::WeyermannVienna => (Ebc(6.0), Ebc(9.0)), // 2.7 - 3.8
            Malt::WeyermannWheatPale => (Ebc(3.0), Ebc(5.0)), // 1.6 - 2.3
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
            Malt::DingemansSpecialB => 15.0,
            Malt::GladfieldAle => 100.0,
            Malt::GladfieldAmericanAle => 100.0,
            Malt::GladfieldBiscuit => 15.0,
            Malt::GladfieldBrown => 15.0,
            Malt::GladfieldCrystalDark => 25.0,
            Malt::GladfieldGermanPilsner => 100.0,
            Malt::GladfieldLagerLight => 100.0,
            Malt::GladfieldMunich => 100.0,
            Malt::GladfieldPilsner => 100.0,
            Malt::GladfieldShepherdsDelight => 10.0,
            Malt::GladfieldVienna => 100.0,
            Malt::GladfieldWheat => 70.0,
            Malt::OatHulls => 5.0,
            Malt::RiceHulls => 8.0,           // 3-8%
            Malt::WeyermannAcidulated => 5.0, // 10.0 in sour beers
            Malt::WeyermannCarafaSpecial2 => 5.0,
            Malt::WeyermannCaramunich2 => 10.0,
            Malt::WeyermannMelanoidin => 20.0,
            Malt::WeyermannMunich1 => 100.0,
            Malt::WeyermannMunich2 => 100.0,
            Malt::WeyermannPilsner => 100.0,
            Malt::WeyermannVienna => 100.0,
            Malt::WeyermannWheatPale => 80.0,
        }
    }

    /// Points per pound per gallon
    // taken from brewersfriend.com
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn ppg(&self) -> f32 {
        match *self {
            Malt::DingemansSpecialB => 33.1,
            Malt::GladfieldAle => 37.4,
            Malt::GladfieldAmericanAle => 37.3,
            Malt::GladfieldBiscuit => 35.0,
            Malt::GladfieldBrown => 34.0,
            Malt::GladfieldCrystalDark => 35.4,
            Malt::GladfieldGermanPilsner => 36.3,
            Malt::GladfieldLagerLight => 35.0,
            Malt::GladfieldMunich => 36.8,
            Malt::GladfieldPilsner => 37.7,
            Malt::GladfieldShepherdsDelight => 35.0,
            Malt::GladfieldVienna => 37.4,
            Malt::GladfieldWheat => 38.8,
            Malt::OatHulls => 0.0,
            Malt::RiceHulls => 0.0,
            Malt::WeyermannAcidulated => 27.0,
            Malt::WeyermannCarafaSpecial2 => 29.9,
            Malt::WeyermannCaramunich2 => 34.0,
            Malt::WeyermannMelanoidin => 34.5,
            Malt::WeyermannMunich1 => 38.0,
            Malt::WeyermannMunich2 => 37.0,
            Malt::WeyermannPilsner => 36.0,
            Malt::WeyermannVienna => 37.0,
            Malt::WeyermannWheatPale => 36.0,
        }
    }

    /// Percent protein from malt spec
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn percent_protein(&self) -> Option<f32> {
        match *self {
            Malt::DingemansSpecialB => None,
            Malt::GladfieldAle => None,
            Malt::GladfieldAmericanAle => None,
            Malt::GladfieldBiscuit => None,
            Malt::GladfieldBrown => None,
            Malt::GladfieldCrystalDark => None,
            Malt::GladfieldGermanPilsner => None,
            Malt::GladfieldLagerLight => None,
            Malt::GladfieldMunich => None,
            Malt::GladfieldPilsner => None,
            Malt::GladfieldShepherdsDelight => None,
            Malt::GladfieldVienna => None,
            Malt::GladfieldWheat => None,
            Malt::OatHulls => Some(0.0),                // presume none
            Malt::RiceHulls => Some(0.0),               // presume none
            Malt::WeyermannAcidulated => Some(0.0),     // presume none
            Malt::WeyermannCarafaSpecial2 => Some(0.0), // presume none
            Malt::WeyermannCaramunich2 => Some(0.0),    // presume none
            Malt::WeyermannMelanoidin => Some(0.0),     // presume none
            Malt::WeyermannMunich1 => Some(f32::midpoint(9.0, 12.5)),
            Malt::WeyermannMunich2 => Some(f32::midpoint(9.0, 12.5)),
            Malt::WeyermannPilsner => Some(f32::midpoint(9.0, 12.0)),
            Malt::WeyermannVienna => Some(f32::midpoint(9.0, 12.5)),
            Malt::WeyermannWheatPale => Some(f32::midpoint(10.0, 13.0)),
        }
    }

    /// Kolbach index (percent of protein that is soluble)
    /// Alco called the Soluble Nitrogen Ratio
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn kolbach_index(&self) -> Option<f32> {
        match *self {
            Malt::DingemansSpecialB => None,
            Malt::GladfieldAle => Some(38.0), // spec 35 - 41
            Malt::GladfieldAmericanAle => Some(38.0), // spec 35 - 41
            Malt::GladfieldBiscuit => Some(37.0), // of zero
            Malt::GladfieldBrown => Some(37.0), // of zero
            Malt::GladfieldCrystalDark => None,
            Malt::GladfieldGermanPilsner => Some(41.0), // spec 37 - 42
            Malt::GladfieldLagerLight => Some(39.0),    // spec 35 - 41
            Malt::GladfieldMunich => Some(40.0),        // spec 37 - 43
            Malt::GladfieldPilsner => Some(39.0),       // spec 35 - 41
            Malt::GladfieldShepherdsDelight => None,
            Malt::GladfieldVienna => Some(40.0), // spec 37 - 43
            Malt::GladfieldWheat => Some(34.0),  // spec 33 - 40
            Malt::OatHulls => Some(37.0),        // of zero
            Malt::RiceHulls => Some(37.0),       // of zero
            Malt::WeyermannAcidulated => Some(37.0), // of zero
            Malt::WeyermannCarafaSpecial2 => None,
            Malt::WeyermannCaramunich2 => Some(37.0), // of zero
            Malt::WeyermannMelanoidin => Some(37.0),  // of zero
            Malt::WeyermannMunich1 => Some(f32::midpoint(37.0, 46.0)),
            Malt::WeyermannMunich2 => Some(f32::midpoint(37.0, 46.0)),
            Malt::WeyermannPilsner => Some(f32::midpoint(36.0, 42.5)),
            Malt::WeyermannVienna => Some(f32::midpoint(37.0, 45.5)),
            Malt::WeyermannWheatPale => Some(f32::midpoint(37.5, 47.0)),
        }
    }

    /// FAN value (mg/L, congress mash at 1.040 or 10 Plato)
    /// from malt specification, if it has one
    #[must_use]
    pub fn fan_from_spec(&self) -> Option<Ppm> {
        match *self {
            Malt::DingemansSpecialB => Some(Ppm(0.0)),      // assume 0
            Malt::GladfieldAle => Some(Ppm(130.0)),         // min spec 120.0
            Malt::GladfieldAmericanAle => Some(Ppm(135.0)), // min spec 120.0
            Malt::GladfieldBiscuit => Some(Ppm(0.0)),       // assume 0
            Malt::GladfieldBrown => Some(Ppm(0.0)),         // assume 0
            Malt::GladfieldCrystalDark => Some(Ppm(0.0)),   // assume 0
            Malt::GladfieldGermanPilsner => Some(Ppm(140.0)), // min spec 135.0
            Malt::GladfieldLagerLight => Some(Ppm(130.0)),  // min spec 120.0
            Malt::GladfieldMunich => Some(Ppm(125.0)),      // min spec 120
            Malt::GladfieldPilsner => Some(Ppm(130.0)),     // min spec 120.0
            Malt::GladfieldShepherdsDelight => Some(Ppm(0.0)), // assume 0
            Malt::GladfieldVienna => Some(Ppm(140.0)),      // min spec 120
            Malt::GladfieldWheat => Some(Ppm(85.0)),        // no spec
            Malt::OatHulls => Some(Ppm(0.0)),               // assume 0
            Malt::RiceHulls => Some(Ppm(0.0)),              // assume 0
            Malt::WeyermannAcidulated => Some(Ppm(0.0)),    // assume 0
            Malt::WeyermannCarafaSpecial2 => Some(Ppm(0.0)), // assume 0
            Malt::WeyermannCaramunich2 => Some(Ppm(0.0)),   // assume 0
            Malt::WeyermannMelanoidin => Some(Ppm(0.0)),    // assume 0
            Malt::WeyermannMunich1 => None,
            Malt::WeyermannMunich2 => None,
            Malt::WeyermannPilsner => None,
            Malt::WeyermannVienna => None,
            Malt::WeyermannWheatPale => None,
        }
    }

    /// Free Amino Nitrogen (FAN) contribution in mg/L at 1.040 SG.
    ///
    /// We first try the malt specification, then we estimate based on malt
    /// specified protein and Kolbach index. For non-base malts and wherever
    /// data is missing, this is zero.
    #[must_use]
    pub fn fan(&self) -> Ppm {
        if let Some(fan) = self.fan_from_spec() {
            fan
        } else if matches!(self, Malt::WeyermannWheatPale) {
            Ppm(85.0) // wheat has less
        } else {
            if let Some(pp) = self.percent_protein()
                && let Some(ki) = self.kolbach_index()
            {
                return Ppm(pp * ki * 0.32);
            }

            Ppm(0.0)
        }
    }
}

impl fmt::Display for Malt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Malt::DingemansSpecialB => write!(f, "[Dingemans Special B]"),
            Malt::GladfieldAle => write!(f, "[Gladfield Ale Malt]"),
            Malt::GladfieldAmericanAle => write!(f, "[Gladfield American Ale Malt]"),
            Malt::GladfieldBiscuit => write!(f, "[Gladfield Biscuit Malt]"),
            Malt::GladfieldBrown => write!(f, "[Gladfield Brown Malt]"),
            Malt::GladfieldCrystalDark => write!(f, "[Gladfield Dark Crystal]"),
            Malt::GladfieldGermanPilsner => write!(f, "[Gladfield German Pilsner Malt]"),
            Malt::GladfieldLagerLight => write!(f, "[Gladfield Lager Light Malt]"),
            Malt::GladfieldMunich => write!(f, "[Gladfield Munich Malt]"),
            Malt::GladfieldPilsner => write!(f, "[Gladfield Pilsner Malt]"),
            Malt::GladfieldShepherdsDelight => write!(f, "[Gladfield Shepherds Delight]"),
            Malt::GladfieldVienna => write!(f, "[Gladfield Vienna Malt]"),
            Malt::GladfieldWheat => write!(f, "[Gladfield Wheat Malt]"),
            Malt::OatHulls => write!(f, "[Oat Hulls]"),
            Malt::RiceHulls => write!(f, "[Rice Hulls]"),
            Malt::WeyermannAcidulated => write!(f, "[Weyermann Acidulated Malt]"),
            Malt::WeyermannCarafaSpecial2 => write!(f, "[Weyermann Carafa Special II]"),
            Malt::WeyermannCaramunich2 => write!(f, "[Weyermann CaraMunich Malt II]"),
            Malt::WeyermannMelanoidin => write!(f, "[Weyermann Melanoidin Malt]"),
            Malt::WeyermannMunich1 => write!(f, "[Weyermann Munich Malt I]"),
            Malt::WeyermannMunich2 => write!(f, "[Weyermann Munich Malt II]"),
            Malt::WeyermannPilsner => write!(f, "[Weyermann Pilsner Malt]"),
            Malt::WeyermannVienna => write!(f, "[Weyermann Vienna Malt]"),
            Malt::WeyermannWheatPale => write!(f, "[Weyermann Wheat Malt Pale]"),
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

    #[test]
    fn test_malt_fan() {
        use strum::IntoEnumIterator;

        for malt in Malt::iter() {
            println!("{}: FAN={}", malt, malt.fan());
        }
    }
}
