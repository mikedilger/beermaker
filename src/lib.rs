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
