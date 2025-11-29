use derive_more::{Add, Div, Mul, Sub, Sum};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Parts per million
/// Same as mg/kg which in water is also mg/L
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub, Mul, Div,
)]
pub struct Ppm(pub f32);

impl fmt::Display for Ppm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} ppm", self.0)
    }
}

/// Specific Gravity
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub)]
pub struct SpecificGravity(pub f32);

impl fmt::Display for SpecificGravity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} s.g.", self.0)
    }
}

/// Plato (approx same as Brix).  Percentage of dissolved sugar.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sum, Sub)]
pub struct Plato(pub f32);

impl fmt::Display for Plato {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}% plato", self.0)
    }
}

impl From<SpecificGravity> for Plato {
    fn from(sg: SpecificGravity) -> Plato {
        let sg = sg.0;
        Plato(-616.868 + 1111.14 * sg - 630.272 * sg.powi(2) + 135.997 * sg.powi(3))
    }
}

impl From<Plato> for SpecificGravity {
    fn from(plato: Plato) -> SpecificGravity {
        let plato = plato.0;
        SpecificGravity(1.000 + (plato / (258.6 - ((plato / 258.2) * 227.1))))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_concentration_conversions() {
        let a = SpecificGravity(1.031);
        let b = Into::<SpecificGravity>::into(Into::<Plato>::into(a));
        println!("  a={a} b={b}");
        assert!(approx_eq!(f32, a.0, b.0, epsilon = 0.0005));

        let a = Plato(7.1);
        let b = Into::<Plato>::into(Into::<SpecificGravity>::into(a));
        println!("  a={a} b={b}");
        assert!(approx_eq!(f32, a.0, b.0, epsilon = 0.005));
    }
}
