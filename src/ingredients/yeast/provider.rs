use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum YeastProvider {
    /// Bootleg Biology
    BootlegBiology,

    /// Cellar Science
    CellarScience,

    /// East Coast Yeast
    EastCoastYeast,

    /// Escarpment Labs
    EscarpmentLabs,

    /// Fermentis
    Fermentis,

    /// GigaYeast
    GigaYeast,

    /// Imperial Yeast
    ImperialYeast,

    /// Lallemand
    Lallemand,

    /// Mangrove Jack's
    MangroveJacks,

    /// Munton's
    Muntons,

    /// Omega Yeast Labs
    OmegaYeastLabs,

    /// RVA Yeast Labs
    RVAYeastLabs,

    /// The Yeast Bay
    TheYeastBay,

    /// White Labs
    WhiteLabs,

    /// Wyeast
    Wyeast
}

impl fmt::Display for YeastProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            YeastProvider::BootlegBiology => write!(f, "Bootleg Biology"),
            YeastProvider::CellarScience => write!(f, "Cellar Science"),
            YeastProvider::EastCoastYeast => write!(f, "East Coast Yeast"),
            YeastProvider::EscarpmentLabs => write!(f, "Escarpment Labs"),
            YeastProvider::Fermentis => write!(f, "Fermentis"),
            YeastProvider::GigaYeast => write!(f, "Giga Yeast"),
            YeastProvider::ImperialYeast => write!(f, "Imperial Yeast"),
            YeastProvider::Lallemand => write!(f, "Lallemand"),
            YeastProvider::MangroveJacks => write!(f, "Mangrove Jack's"),
            YeastProvider::Muntons => write!(f, "Munton's"),
            YeastProvider::OmegaYeastLabs => write!(f, "Omega Yeast Labs"),
            YeastProvider::RVAYeastLabs => write!(f, "RVA Yeast Labs"),
            YeastProvider::TheYeastBay => write!(f, "The Yeast Bay"),
            YeastProvider::WhiteLabs => write!(f, "White Labs"),
            YeastProvider::Wyeast => write!(f, "Wyeast"),
        }
    }
}
