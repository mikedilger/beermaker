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
