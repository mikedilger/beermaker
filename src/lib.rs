//! Beermaker is a library for helping you make beer.

#![warn(clippy::pedantic)]
#![deny(
    missing_debug_implementations,
    trivial_numeric_casts,
    clippy::string_slice,
    unused_import_braces,
    unused_results,
    unused_lifetimes,
    unused_labels,
    unused_extern_crates,
    non_ascii_idents,
    keyword_idents,
    deprecated_in_future,
    unstable_features,
    single_use_lifetimes,
    unreachable_pub,
    missing_copy_implementations,
    missing_docs
)]
#![allow(clippy::match_same_arms, clippy::doc_markdown)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

/// Units
pub mod units;

/// Ingredients
pub mod ingredients;

/// Prelude, for importing all of the units and ingredients
///
/// `use beermaker::prelude::*`
pub mod prelude {
    pub use crate::ingredients::*;
    pub use crate::units::alkalinity::*;
    pub use crate::units::color::*;
    pub use crate::units::concentration::*;
    pub use crate::units::hardness::*;
    pub use crate::units::temperature::*;
    pub use crate::units::time::*;
    pub use crate::units::volume::*;
    pub use crate::units::weight::*;
    pub use crate::units::{Ibu, Ph};
}

mod style;
pub use style::Style;

mod process;
pub use process::{Packaging, Process};

mod mash;
pub use mash::MashRest;

mod recipe;
pub use recipe::Recipe;

mod printer;
pub use printer::{Steps, print_recipe};

use std::ops::Range;
use units::concentration::{Brix, SpecificGravity};
use units::temperature::{Celsius, Fahrenheit};

/// Correct a specific gravity reading from a hydrometer when the
/// sample is not at the calibration temperature of the hydrometer.
// should be close to the table:
//  10C    -0.002
//  21.1C  +0.001
//  26.7C  +0.003
//  32.2C  +0.005
//  37.8C  +0.008
//  48.9C  +0.013
#[must_use]
#[rustfmt::skip]
pub fn hydrometer_temp_correction(
    reading_gravity: f32,
    reading_temp: Celsius,
    calibration_temp: Celsius,
) -> f32 {
    let f: Fahrenheit = reading_temp.into();
    let t = f.0;
    let f: Fahrenheit = calibration_temp.into();
    let cal = f.0;

    let num = 1.001_303_4
        - 0.000_134_722_13 * t
        + 0.000_002_040_526 * t.powi(2)
        - 0.000_000_002_328_209_4 * t.powi(3);

    let denom = 1.001_303_4
        - 0.000_134_722_13 * cal
        + 0.000_002_040_526 * cal.powi(2)
        - 0.000_000_002_328_209_4 * cal.powi(3);

    reading_gravity * (num / denom)
}

/// Refractometer correction for Wort.
///
/// Your refractometer will have 3 scales:
///   SGsugar - presuming just water and sugar
///   Brix - literal measurement
///   SGwort - presuming it is wort like beer, with a correction
///
/// Use the pre-fermentation reading as `original_brix`
/// Use the current reading as `current_brix`
///
/// This function will return the current specific gravity
/// and the current % ABV.
///
/// Both sugar and alcohol will refract light. To figure out which
/// is which, the `original_sg_wort` reading is required.
#[must_use]
pub fn refractometer_correction(
    original_sg_wort: SpecificGravity,
    current_sg_wort: SpecificGravity,
) -> (SpecificGravity, f32) {

    let original_brix: Brix = original_sg_wort.into();
    let current_brix: Brix = current_sg_wort.into();

    let gravity = SpecificGravity(
        1.0
            + 0.006276 * current_brix.0
            - 0.002349 * original_brix.0
    );

    let abw = 0.67062 * original_brix.0
        - 0.66091 * current_brix.0;

    let abv = (gravity.0 * abw)/0.791;

    (gravity, abv)

    // I'm not using this one since I'll fall in line with the Zymurgy
    // article. But I don't want to forget it.
    //
    // Sugar refraction formula (amount beyond water's refractive index)
    //
    // sugar_refraction = 0.001510963 * plato - 0.0001460907

    // I'm not using this one since I'll fall in line with the Zymurgy
    // article. But I don't want to forget it.
    //
    // Alcohol refraction formula (within rage up to 20% alcohol)
    //
    // 4PL Curve fit to
    // The Refractive Indices of Ethyl Alcohol and Water Mixtures,
    //   1939, Janina Nowakowska
    // x = percent alcohol by weight
    // y = refractive index
    // y = 1.401565 + (1.333143 - 1.401565)/(1 + (x/60.46651)^1.286271)

    // ABV = (76.08 * (og - fg) / (1.775 - og)) * (fg / 0.794)

    // ABV to ABW:
    // ABW = ABV * 0.8
    // ABV = ABW * 1.25

    // We don't bother with wort correction factor, we require the operator
    // to read the SGwort scale instead.
    // But
    // Actual Brix = Measured Brix/WCF
    // WCF = Wort Correction Factor (usually around 1.04, calculate for \
    //       individual refractometers)
}

fn union_ranges<T: PartialOrd + Copy>(ranges: &[Range<T>]) -> Range<T> {
    let mut start = ranges[0].start;
    let mut end = ranges[0].end;
    for range in ranges.iter().skip(1) {
        if range.start < start {
            start = range.start;
        }
        if range.end > end {
            end = range.end;
        }
    }
    start..end
}
