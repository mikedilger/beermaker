use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Time in minutes
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Add,
    Sum,
    Sub,
    Mul,
    Div,
)]
pub struct Minutes(pub usize);

impl fmt::Display for Minutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} min", self.0)
    }
}

/// Time in hours
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Add,
    Sum,
    Sub,
    Mul,
    Div,
)]
pub struct Hours(pub usize);

impl fmt::Display for Hours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} hours", self.0)
    }
}

/// Time in days
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Add,
    Sum,
    Sub,
    Mul,
    Div,
)]
pub struct Days(pub usize);

impl fmt::Display for Days {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} days", self.0)
    }
}

impl From<Minutes> for Hours {
    fn from(m: Minutes) -> Self {
        Hours(m.0 / 60)
    }
}

impl From<Hours> for Days {
    fn from(h: Hours) -> Self {
        Days(h.0 / 24)
    }
}

impl From<Days> for Hours {
    fn from(d: Days) -> Self {
        Hours(d.0 * 24)
    }
}

impl From<Hours> for Minutes {
    fn from(h: Hours) -> Self {
        Minutes(h.0 * 60)
    }
}
