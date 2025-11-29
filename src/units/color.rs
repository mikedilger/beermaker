use derive_more::{Add, Div};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Color in Lovabond (°L)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Div)]
pub struct Lovabond(pub f32);

impl fmt::Display for Lovabond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °L", self.0)
    }
}

/// Color in EBC
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Div)]
pub struct Ebc(pub f32);

impl fmt::Display for Ebc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} EBC", self.0)
    }
}

/// Color in SRM
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Add, Div)]
pub struct Srm(pub f32);

impl fmt::Display for Srm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} SRM", self.0)
    }
}

impl From<Srm> for Lovabond {
    fn from(v: Srm) -> Lovabond {
        Lovabond((v.0 + 0.76) / 1.3546)
    }
}

impl From<Lovabond> for Srm {
    fn from(v: Lovabond) -> Srm {
        Srm(v.0 * 1.3546 - 0.76)
    }
}

impl From<Srm> for Ebc {
    fn from(v: Srm) -> Ebc {
        Ebc(v.0 * 1.97)
    }
}

impl From<Ebc> for Srm {
    fn from(v: Ebc) -> Srm {
        Srm(v.0 / 1.97)
    }
}

impl From<Ebc> for Lovabond {
    fn from(v: Ebc) -> Lovabond {
        Into::<Srm>::into(v).into()
    }
}

impl From<Lovabond> for Ebc {
    fn from(v: Lovabond) -> Ebc {
        Into::<Srm>::into(v).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_color_conversions() {
        let a = Lovabond(14.56);
        let b = Into::<Lovabond>::into(Into::<Srm>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Lovabond>::into(Into::<Ebc>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Ebc(14.56);
        let b = Into::<Ebc>::into(Into::<Srm>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Ebc>::into(Into::<Lovabond>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));

        let a = Srm(14.56);
        let b = Into::<Srm>::into(Into::<Ebc>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
        let b = Into::<Srm>::into(Into::<Lovabond>::into(a));
        assert!(approx_eq!(f32, a.0, b.0, ulps = 10));
    }
}
