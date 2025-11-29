use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Weight in Grams (g, metric)
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Grams(pub f32);

impl fmt::Display for Grams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} g", self.0)
    }
}

/// Weight in Kilograms (kg, metric)
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Kilograms(pub f32);

impl fmt::Display for Kilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} kg", self.0)
    }
}

/// Weight in Milligrams (mg, metric)
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Milligrams(pub f32);

impl fmt::Display for Milligrams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} mg", self.0)
    }
}

/// Weight in Ounces (oz, imperial)
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Ounces(pub f32);

impl fmt::Display for Ounces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} oz", self.0)
    }
}

/// Weight in Pounds (lbs, imperial)
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Pounds(pub f32);

impl fmt::Display for Pounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} lbs", self.0)
    }
}

const MILLIGRAMS_PER_GRAM: f32 = 1000.0;
const GRAMS_PER_OUNCE: f32 = 28.34952;
const OUNCES_PER_POUND: f32 = 16.0;
const GRAMS_PER_KILOGRAM: f32 = 1000.0;
const POUNDS_PER_KILOGRAM: f32 = 2.204_623;

impl From<Kilograms> for Grams {
    fn from(v: Kilograms) -> Self {
        Grams(v.0 * GRAMS_PER_KILOGRAM)
    }
}

impl From<Milligrams> for Grams {
    fn from(v: Milligrams) -> Self {
        Grams(v.0 / MILLIGRAMS_PER_GRAM)
    }
}

impl From<Ounces> for Grams {
    fn from(v: Ounces) -> Self {
        Grams(v.0 * GRAMS_PER_OUNCE)
    }
}

impl From<Pounds> for Grams {
    fn from(v: Pounds) -> Self {
        Grams(v.0 * GRAMS_PER_OUNCE * OUNCES_PER_POUND)
    }
}

impl From<Grams> for Ounces {
    fn from(v: Grams) -> Self {
        Ounces(v.0 / GRAMS_PER_OUNCE)
    }
}

impl From<Kilograms> for Ounces {
    fn from(v: Kilograms) -> Self {
        Ounces(v.0 * POUNDS_PER_KILOGRAM * OUNCES_PER_POUND)
    }
}

impl From<Milligrams> for Ounces {
    fn from(v: Milligrams) -> Self {
        Ounces(v.0 / MILLIGRAMS_PER_GRAM / GRAMS_PER_OUNCE)
    }
}

impl From<Pounds> for Ounces {
    fn from(v: Pounds) -> Self {
        Ounces(v.0 * OUNCES_PER_POUND)
    }
}

impl From<Grams> for Pounds {
    fn from(v: Grams) -> Self {
        Pounds(v.0 / GRAMS_PER_OUNCE / OUNCES_PER_POUND)
    }
}

impl From<Kilograms> for Pounds {
    fn from(v: Kilograms) -> Self {
        Pounds(v.0 * POUNDS_PER_KILOGRAM)
    }
}

impl From<Milligrams> for Pounds {
    fn from(v: Milligrams) -> Self {
        Pounds(v.0 / MILLIGRAMS_PER_GRAM / GRAMS_PER_OUNCE / OUNCES_PER_POUND)
    }
}

impl From<Ounces> for Pounds {
    fn from(v: Ounces) -> Self {
        Pounds(v.0 / OUNCES_PER_POUND)
    }
}

impl From<Grams> for Kilograms {
    fn from(v: Grams) -> Self {
        Kilograms(v.0 / GRAMS_PER_KILOGRAM)
    }
}

impl From<Milligrams> for Kilograms {
    fn from(v: Milligrams) -> Self {
        Kilograms(v.0 / MILLIGRAMS_PER_GRAM / GRAMS_PER_KILOGRAM)
    }
}

impl From<Ounces> for Kilograms {
    fn from(v: Ounces) -> Self {
        Kilograms(v.0 / OUNCES_PER_POUND / POUNDS_PER_KILOGRAM)
    }
}

impl From<Pounds> for Kilograms {
    fn from(v: Pounds) -> Self {
        Kilograms(v.0 / POUNDS_PER_KILOGRAM)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_weight_conversions() {
        let a = Ounces(16.5);
        let b = Into::<Ounces>::into(Into::<Kilograms>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Ounces>::into(Into::<Pounds>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Ounces>::into(Into::<Grams>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Pounds(16.5);
        let b = Into::<Pounds>::into(Into::<Kilograms>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Pounds>::into(Into::<Ounces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Pounds>::into(Into::<Grams>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Grams(16.5);
        let b = Into::<Grams>::into(Into::<Kilograms>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Grams>::into(Into::<Ounces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Grams>::into(Into::<Pounds>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Kilograms(16.5);
        let b = Into::<Kilograms>::into(Into::<Grams>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Kilograms>::into(Into::<Ounces>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Kilograms>::into(Into::<Pounds>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
    }
}
