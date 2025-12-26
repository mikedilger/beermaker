use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Volume in Milliliters (mL, metric)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Add, Sum, Sub, Mul, Div)]
pub struct Milliliters(pub f32);

impl fmt::Display for Milliliters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} mL", self.0)
    }
}

impl Eq for Milliliters {}

impl Ord for Milliliters {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for Milliliters {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Volume in Liters (L, metric)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Add, Sum, Sub, Mul, Div)]
pub struct Liters(pub f32);

impl fmt::Display for Liters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} L", self.0)
    }
}

impl Eq for Liters {}

impl Ord for Liters {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for Liters {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Volume in Gallons (gal, imperial)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Add, Sum, Sub, Mul, Div)]
pub struct Gallons(pub f32);

impl fmt::Display for Gallons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} gal", self.0)
    }
}

impl Eq for Gallons {}

impl Ord for Gallons {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for Gallons {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Volume in U.S liquid quarts (qt, US imperial)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Add, Sum, Sub, Mul, Div)]
pub struct Quarts(pub f32);

impl fmt::Display for Quarts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} qts", self.0)
    }
}

impl Eq for Quarts {}

impl Ord for Quarts {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for Quarts {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Volume in U.S fluid oz
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Add, Sum, Sub, Mul, Div)]
pub struct FluidOunces(pub f32);

impl fmt::Display for FluidOunces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} fl oz", self.0)
    }
}

impl Eq for FluidOunces {}

impl Ord for FluidOunces {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for FluidOunces {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Cup = 8 fl oz
// Pint = 16 fl oz

const MILLILITERS_PER_LITER: f32 = 1000.0;
const LITERS_PER_GALLON: f32 = 3.7854;
const QUARTS_PER_GALLON: f32 = 4.0;
const FLUID_OUNCES_PER_GALLON: f32 = 128.0;
const MILLILITERS_PER_FLUID_OUNCE: f32 = 29.5734375;

impl From<Liters> for Milliliters {
    fn from(v: Liters) -> Self {
        Milliliters(v.0 * MILLILITERS_PER_LITER)
    }
}

impl From<Gallons> for Milliliters {
    fn from(v: Gallons) -> Self {
        Milliliters(v.0 * LITERS_PER_GALLON * MILLILITERS_PER_LITER)
    }
}

impl From<Quarts> for Milliliters {
    fn from(v: Quarts) -> Self {
        Milliliters(v.0 / QUARTS_PER_GALLON * LITERS_PER_GALLON * MILLILITERS_PER_LITER)
    }
}

impl From<FluidOunces> for Milliliters {
    fn from(v: FluidOunces) -> Self {
        Milliliters(v.0 * MILLILITERS_PER_FLUID_OUNCE)
    }
}

impl From<Milliliters> for Liters {
    fn from(v: Milliliters) -> Self {
        Liters(v.0 / MILLILITERS_PER_LITER)
    }
}

impl From<Gallons> for Liters {
    fn from(v: Gallons) -> Self {
        Liters(v.0 * LITERS_PER_GALLON)
    }
}

impl From<Quarts> for Liters {
    fn from(v: Quarts) -> Self {
        Liters(v.0 / QUARTS_PER_GALLON * LITERS_PER_GALLON)
    }
}

impl From<FluidOunces> for Liters {
    fn from(v: FluidOunces) -> Self {
        Liters(v.0 * MILLILITERS_PER_FLUID_OUNCE / MILLILITERS_PER_LITER)
    }
}

impl From<Milliliters> for Gallons {
    fn from(v: Milliliters) -> Self {
        Gallons(v.0 / MILLILITERS_PER_LITER / LITERS_PER_GALLON)
    }
}

impl From<Liters> for Gallons {
    fn from(v: Liters) -> Self {
        Gallons(v.0 / LITERS_PER_GALLON)
    }
}

impl From<Quarts> for Gallons {
    fn from(v: Quarts) -> Self {
        Gallons(v.0 / QUARTS_PER_GALLON)
    }
}

impl From<FluidOunces> for Gallons {
    fn from(v: FluidOunces) -> Self {
        Gallons(v.0 / FLUID_OUNCES_PER_GALLON)
    }
}

impl From<Milliliters> for Quarts {
    fn from(v: Milliliters) -> Self {
        Quarts(v.0 / MILLILITERS_PER_LITER / LITERS_PER_GALLON * QUARTS_PER_GALLON)
    }
}

impl From<Liters> for Quarts {
    fn from(v: Liters) -> Self {
        Quarts(v.0 / LITERS_PER_GALLON * QUARTS_PER_GALLON)
    }
}

impl From<Gallons> for Quarts {
    fn from(v: Gallons) -> Self {
        Quarts(v.0 * QUARTS_PER_GALLON)
    }
}

impl From<FluidOunces> for Quarts {
    fn from(v: FluidOunces) -> Self {
        Quarts(v.0 / FLUID_OUNCES_PER_GALLON * QUARTS_PER_GALLON)
    }
}

impl From<Milliliters> for FluidOunces {
    fn from(v: Milliliters) -> Self {
        FluidOunces(v.0 / MILLILITERS_PER_FLUID_OUNCE)
    }
}

impl From<Liters> for FluidOunces {
    fn from(v: Liters) -> Self {
        FluidOunces(v.0 / MILLILITERS_PER_FLUID_OUNCE * MILLILITERS_PER_LITER)
    }
}

impl From<Quarts> for FluidOunces {
    fn from(v: Quarts) -> Self {
        FluidOunces(v.0 / QUARTS_PER_GALLON * FLUID_OUNCES_PER_GALLON)
    }
}

impl From<Gallons> for FluidOunces {
    fn from(v: Gallons) -> Self {
        FluidOunces(v.0 * FLUID_OUNCES_PER_GALLON)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_volume_conversions() {
        let a = Milliliters(18.211);
        let b = Into::<Milliliters>::into(Into::<Liters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Milliliters>::into(Into::<Gallons>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Milliliters>::into(Into::<Quarts>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Milliliters>::into(Into::<FluidOunces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Liters(18.211);
        let b = Into::<Liters>::into(Into::<Milliliters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Liters>::into(Into::<Gallons>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Liters>::into(Into::<Quarts>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Liters>::into(Into::<FluidOunces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Gallons(18.211);
        let b = Into::<Gallons>::into(Into::<Milliliters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Gallons>::into(Into::<Liters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Gallons>::into(Into::<Quarts>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Gallons>::into(Into::<FluidOunces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Quarts(18.211);
        let b = Into::<Quarts>::into(Into::<Milliliters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Quarts>::into(Into::<Liters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Quarts>::into(Into::<Gallons>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Quarts>::into(Into::<FluidOunces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = FluidOunces(18.211);
        let b = Into::<FluidOunces>::into(Into::<Milliliters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<FluidOunces>::into(Into::<Liters>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<FluidOunces>::into(Into::<Gallons>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<FluidOunces>::into(Into::<Quarts>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
}
}
