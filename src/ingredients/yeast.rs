use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Range;
use strum::EnumIter;

/// Flocculation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum Flocculation {
    /// Low
    Low,

    /// Low to Medium
    LowMedium,

    /// Medium
    Medium,

    /// Medium to High
    MediumHigh,

    /// High
    High,

    /// Very High
    VeryHigh,
}

/// A type of Yesat
#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum Yeast {
    /// Kveik Voss
    //   - Omega Yeast OYL-061, Lallemand Kveik Ale Yeast,
    //    Imperial Yeast Ale Loki,
    KveikVoss,

    /// Lutra Kveik
    // OYL-071
    // pseudolager, lager-like at 68-72C
    LutraKveik,

    /// Lallemand Munich Classic German Wheat-Style Ale Yeast
    LallemandMunichClassic,

    /// Safale T-58, Phenolic for English/Belgian
    /// high banana, clove, pepper, great with wheat beers
    SafaleT58,

    /// Safale W-68 (german style wheat beers), can't find in NZ
    /// Wyeast Strain 3068 from Weihenstephan Weizen beers,
    /// but behaves differently as it is dry.
    SafaleW68,

    /// Safale WB-06 (wheat)
    SafaleWB06,

    /// Saflager W-34/70
    SaflagerW3470,

    /// White labs Hefeweizen Ale Yeast, WLP300
    /// Wyeast Strain 3068 from Weihenstephan Weizen beers
    WLP300,

    /// White Labs Bavarian Weizen Ale Yeast, WLP351
    WLP351,

    /// White Labs Hefeweizen IV Ale Yeast, WLP380
    WLP380,

    /// White Labs Oktoberfest/Märzen WLP820
    WLP820,

    /// White Labs German Lager Yeast WLP830
    // I've heard this is strain 34/70, as is Wyeast 2206
    WLP830,

    /// White Labs German Bock Lager Yeast WLP833
    WLP833,

    /// White Labs German X Lager Yeast WLP835
    WLP835,

    /// White Labs Southern German Lager Yeast WLP838
    WLP838,
    // EC-1118 dry sparkling wine yeast
    // great for bottle conditioning

    // CBC-1
    // great for bottle conditioning
}

impl Yeast {
    /// Sources say it is between 5b and 20b depending on
    /// viability of the yeast.  Fermentis guarantees a
    /// minimum of 6b, many home brew tests show it is
    /// closer to 20b.  We pick a number in between.
    pub const CELLS_PER_GRAM_DRY: u64 = 13_000_000_000;

    /// The minimum recommended temperature to ferment at
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn temp_range(&self) -> Range<Celsius> {
        match *self {
            Yeast::KveikVoss => Celsius(25.0)..Celsius(40.0),
            Yeast::LutraKveik => Celsius(12.0)..Celsius(35.0),
            Yeast::LallemandMunichClassic => Celsius(17.0)..Celsius(25.0),
            Yeast::SafaleT58 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleW68 => Celsius(18.0)..Celsius(26.0),
            Yeast::SafaleWB06 => Celsius(18.0)..Celsius(26.0),
            Yeast::SaflagerW3470 => Celsius(12.0)..Celsius(18.0),
            Yeast::WLP300 => Celsius(20.0)..Celsius(22.0),
            Yeast::WLP351 => Celsius(19.0)..Celsius(21.0),
            Yeast::WLP380 => Celsius(19.0)..Celsius(21.0),
            Yeast::WLP820 => Celsius(11.0)..Celsius(14.0),
            Yeast::WLP830 => Celsius(10.0)..Celsius(13.0),
            Yeast::WLP833 => Celsius(9.0)..Celsius(13.0),
            Yeast::WLP835 => Celsius(10.0)..Celsius(12.0),
            Yeast::WLP838 => Celsius(10.0)..Celsius(13.0),
        }
    }

    /// The recommended fermentation temperature
    #[must_use]
    pub fn temp(&self) -> Celsius {
        let range = self.temp_range();
        Celsius(f32::midpoint(range.start.0, range.end.0))
    }

    /// Attenuation (apparent), fraction
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn attenuation_range(&self) -> Range<f32> {
        match *self {
            Yeast::KveikVoss => 0.76..0.82,
            Yeast::LutraKveik => 0.75..0.82,
            Yeast::LallemandMunichClassic => 0.76..0.83,
            Yeast::SafaleT58 => 0.72..0.78,
            Yeast::SafaleW68 => 0.78..0.84,
            Yeast::SafaleWB06 => 0.86..0.90,
            Yeast::SaflagerW3470 => 0.80..0.84,
            Yeast::WLP300 => 0.72..0.76,
            Yeast::WLP351 => 0.75..0.82,
            Yeast::WLP380 => 0.73..0.80,
            Yeast::WLP820 => 0.65..0.73,
            Yeast::WLP830 => 0.74..0.79,
            Yeast::WLP833 => 0.70..0.76,
            Yeast::WLP835 => 0.70..0.76,
            Yeast::WLP838 => 0.68..0.76,
        }
    }

    /// The expected apparent attenuation
    #[must_use]
    pub fn attenuation(&self) -> f32 {
        let range = self.attenuation_range();
        f32::midpoint(range.start, range.end)
    }

    /// Alcohol tolerance, fraction, range
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn alcohol_tolerance_range(&self) -> Range<f32> {
        match *self {
            Yeast::KveikVoss => 0.12..0.12,
            Yeast::LutraKveik => 0.15..0.15,
            Yeast::LallemandMunichClassic => 0.12..0.12,
            Yeast::SafaleT58 => 0.09..0.11,
            Yeast::SafaleW68 => 0.09..0.11, // unlisted on specs
            Yeast::SafaleWB06 => 0.09..0.11,
            Yeast::SaflagerW3470 => 0.09..0.11,
            Yeast::WLP300 => 0.08..0.12,
            Yeast::WLP351 => 0.15..0.15,
            Yeast::WLP380 => 0.05..0.10,
            Yeast::WLP820 => 0.05..0.10,
            Yeast::WLP830 => 0.05..0.10,
            Yeast::WLP833 => 0.05..0.10,
            Yeast::WLP835 => 0.08..0.12,
            Yeast::WLP838 => 0.05..0.10,
        }
    }

    /// Alcohol tolerance, expected
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn alcohol_tolerance(&self) -> f32 {
        let range = self.alcohol_tolerance_range();
        f32::midpoint(range.start, range.end)
    }

    /// Flocculation
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn flocculation(&self) -> Flocculation {
        match *self {
            Yeast::KveikVoss => Flocculation::VeryHigh,
            Yeast::LutraKveik => Flocculation::MediumHigh,
            Yeast::LallemandMunichClassic => Flocculation::Low,
            Yeast::SafaleT58 => Flocculation::Medium,
            Yeast::SafaleW68 => Flocculation::Medium,
            Yeast::SafaleWB06 => Flocculation::Low,
            Yeast::SaflagerW3470 => Flocculation::High,
            Yeast::WLP300 => Flocculation::Low,
            Yeast::WLP351 => Flocculation::Low,
            Yeast::WLP380 => Flocculation::Low,
            Yeast::WLP820 => Flocculation::Medium,
            Yeast::WLP830 => Flocculation::Medium,
            Yeast::WLP833 => Flocculation::Medium,
            Yeast::WLP835 => Flocculation::Medium,
            Yeast::WLP838 => Flocculation::MediumHigh,
        }
    }

    /// Is the yeast dry?
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn is_dry(&self) -> bool {
        match *self {
            Yeast::KveikVoss => true,
            Yeast::LutraKveik => false,
            Yeast::LallemandMunichClassic => true,
            Yeast::SafaleT58 => true,
            Yeast::SafaleW68 => true,
            Yeast::SafaleWB06 => true,
            Yeast::SaflagerW3470 => true,
            Yeast::WLP300 => false,
            Yeast::WLP351 => false,
            Yeast::WLP380 => false,
            Yeast::WLP820 => false,
            Yeast::WLP830 => false,
            Yeast::WLP833 => false,
            Yeast::WLP835 => false,
            Yeast::WLP838 => false,
        }
    }

    /// Pitching rate, if known
    #[must_use]
    pub fn pitching_rate(&self) -> Option<(Grams, Liters)> {
        match *self {
            // 50-100g/hL
            Yeast::LallemandMunichClassic => Some((Grams(75.0), Liters(100.0))),
            _ => None,
        }
    }
}

impl fmt::Display for Yeast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Yeast::LutraKveik => write!(f, "[Lutra Kveik OYL-071]"),
            Yeast::KveikVoss => write!(f, "[Kveik Voss]"),
            Yeast::LallemandMunichClassic => {
                write!(f, "[Lallemand Munich Classic German Wheat-Style Ale Yeast]")
            }
            Yeast::SafaleT58 => write!(f, "[Safale T58]"),
            Yeast::SafaleW68 => write!(f, "[Safale W68)]"),
            Yeast::SafaleWB06 => write!(f, "[Safale WB-06]"),
            Yeast::SaflagerW3470 => write!(f, "[SafLager W-34/70]"),
            Yeast::WLP300 => write!(f, "[White Labs Hefeweizen Ale Yeast WLP300]"),
            Yeast::WLP351 => write!(f, "[White Labs Bavarian Weizen Ale Yeast WLP351]"),
            Yeast::WLP380 => write!(f, "[White Labs Hefeweizen IV Ale Yeast WLP380]"),
            Yeast::WLP820 => write!(f, "[White Labs Oktoberfest/Märzen WLP820]"),
            Yeast::WLP830 => write!(f, "[White Labs German Lager Yeast WLP830]"),
            Yeast::WLP833 => write!(f, "[White Labs German Bock Lager Yeast WLP833]"),
            Yeast::WLP835 => write!(f, "[White Labs German X Lager Yeast WLP835]"),
            Yeast::WLP838 => write!(f, "[White Labs Southern German Lager Yeast WLP838]"),
        }
    }
}
