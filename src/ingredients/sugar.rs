use crate::units::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A type of sugar
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Sugar {
    /// Sucrose, also known as Cane Sugar or Table Sugar
    Sucrose,

    /// Fructose
    Fructose,

    /// Turbinado
    Turbinado,

    /// Dextose, also known as Dextrose or Corn Sugar, a monohydrate of Glucose
    Dextrose,

    /// Maltodextrin
    Maltodextrin,

    /// Dry Malt Extract
    DME,

    /// Brown Sugar
    BrownSugar,

    /// Corn Syrup
    CornSyrup,

    /// Honey
    Honey,

    /// Invert Sugar
    InvertSugar,

    /// Maple Syrup
    MapleSyrup,

    /// Light Liquid Malt Extract
    LightLME,
}

impl Sugar {
    /// Points per pound per gallon
    #[must_use]
    pub fn ppg(&self) -> f32 {
        self.fermentability() * 46.0
    }

    /// Color, EBC
    #[must_use]
    pub fn ebc(&self) -> Ebc {
        match *self {
            Sugar::DME => Ebc(4.0),         // guess
            Sugar::BrownSugar => Ebc(15.0), // guess
            Sugar::CornSyrup => Ebc(2.0),   // guess
            Sugar::Honey => Ebc(5.0),       // guess
            Sugar::MapleSyrup => Ebc(10.0), // guess
            Sugar::LightLME => Ebc(8.0),    // Estimate
            _ => Ebc(0.0),
        }
    }

    /// Fermentability
    ///
    /// This is what fraction will ferment, via yeast, like pure sugar.
    /// The part that doesn't ferment may be water, unfermentable sugars,
    /// or other things.
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn fermentability(&self) -> f32 {
        match *self {
            Sugar::Sucrose => 1.00, // brewersfriend and omnicalculator agree 1.00
            Sugar::Fructose => 1.00,
            Sugar::Turbinado => 1.00,
            Sugar::Dextrose => 0.91, // brewersfriend and omnicalculator agree 0.91
            Sugar::InvertSugar => 0.91, // as a hydrate
            Sugar::BrownSugar => 0.89,
            Sugar::Maltodextrin => 0.03, // 0.86 from somewhere
            Sugar::MapleSyrup => 0.77,
            Sugar::Honey => 0.74, // omnicalculator says 0.78
            Sugar::CornSyrup => 0.69,
            Sugar::DME => 0.68, // brewersfriend and omnicalculator agree 0.68
            Sugar::LightLME => 0.68, // presume it is like DME
        }
    }

    /// Unfermentability, percentage that remains as final gravity
    ///
    /// Note that this is not just `1.0 - fermentability` because
    /// some parts are not even sugar (e.g. water).
    #[must_use]
    #[allow(clippy::match_same_arms)]
    pub fn unfermentability(&self) -> f32 {
        match *self {
            Sugar::Sucrose => 0.0,
            Sugar::Fructose => 0.0,
            Sugar::Turbinado => 0.0,
            Sugar::Dextrose => 0.0,
            Sugar::InvertSugar => 0.0,
            Sugar::BrownSugar => 0.0,
            Sugar::Maltodextrin => 0.97,
            Sugar::MapleSyrup => 0.02, // wild guess, sugars mostly ferm.
            Sugar::Honey => 0.075,     // honey is 17-20% water, 5-10% unferm
            Sugar::CornSyrup => 0.05,  // wild guess
            Sugar::DME => 0.12,        // 20% water
            Sugar::LightLME => 0.12,   // presume it is like DME
        }
    }

    /// Amount for priming
    // See also "Brew By the Numbers, Zymurgy, Summer 1995
    #[must_use]
    pub fn priming_amount(
        &self,
        co2_volume: f32,
        beer_volume: Liters,
        beer_temp: Celsius,
    ) -> Grams {
        let beer_volume: Gallons = beer_volume.into();
        let beer_temp: Fahrenheit = beer_temp.into();

        // initial carbonation
        let residual_co2_volume =
            3.0378 - 0.050_062 * beer_temp.0 + 0.000_265_55 * beer_temp.0.powi(2);

        let factor = self.fermentability();

        Grams((15.195 * beer_volume.0 * (co2_volume - residual_co2_volume)) / factor)
    }
}

impl fmt::Display for Sugar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Sugar::Sucrose => write!(f, "[Sucrose]"),
            Sugar::Fructose => write!(f, "[Fructose]"),
            Sugar::Turbinado => write!(f, "[Turbinado]"),
            Sugar::Dextrose => write!(f, "[Dextrose]"),
            Sugar::InvertSugar => write!(f, "[Invert Sugar]"),
            Sugar::BrownSugar => write!(f, "[Brown Sugar]"),
            Sugar::Maltodextrin => write!(f, "[Maltodextrin]"),
            Sugar::MapleSyrup => write!(f, "[Maple Syrup]"),
            Sugar::Honey => write!(f, "[Honey]"),
            Sugar::CornSyrup => write!(f, "[Corn Syrup]"),
            Sugar::DME => write!(f, "[DME]"),
            Sugar::LightLME => write!(f, "[Light LME]"),
        }
    }
}
