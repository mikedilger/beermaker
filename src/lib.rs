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
pub use printer::print_recipe;

use std::ops::Range;

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
