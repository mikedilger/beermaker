use derive_more::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Temperature in Celsius (metric)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sub, Mul, Div)]
pub struct Celsius(pub f32);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °C", self.0)
    }
}

/// Temperature in Fahrenheit (imperial)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Sub, Mul, Div)]
pub struct Fahrenheit(pub f32);

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °F", self.0)
    }
}

const TEMP_CONVERT: f32 = 9.0 / 5.0;

impl From<Celsius> for Fahrenheit {
    fn from(v: Celsius) -> Fahrenheit {
        Fahrenheit((v.0 * TEMP_CONVERT) + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(v: Fahrenheit) -> Celsius {
        Celsius((v.0 - 32.0) / TEMP_CONVERT)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_temperature_conversions() {
        let a = Celsius(63.54);
        let b = Into::<Celsius>::into(Into::<Fahrenheit>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Fahrenheit(200.00);
        let b = Into::<Fahrenheit>::into(Into::<Celsius>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
    }
}
