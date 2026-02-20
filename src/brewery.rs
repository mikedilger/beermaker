use crate::Packaging;
use crate::ingredients::{Salt, WaterProfile};
use crate::prelude::*;
use crate::units::prelude::*;
use serde::{Deserialize, Serialize};

/// Set of equipment and supplies that are used to make beer.
/// Independent of any recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brewery {
    /// Water profile
    pub water_profile: WaterProfile,

    /// Water salts available
    pub salts_available: Vec<Salt>,

    /// Water acids/bases available
    pub acids_available: Vec<Acid>,

    /// Max usable volume of your mash tun (or kettle if BIAB)
    pub mash_tun_volume: Liters,

    /// Mash tun losses
    pub mash_tun_losses: Liters,

    /// Max usable volume of your boil kettle
    pub max_kettle_volume: Liters,

    /// How much is left behind in the kettle after the boil
    /// This may depend a lot on how you empty the kettle (pumping,
    /// racking, or pouring), and if your hops are kept isolated.
    pub kettle_losses: Liters,

    /// Volume lost per hour during the boil.
    ///
    /// For a 5 gallon homebrew, this is typically 4 - 5 liters.
    /// You can do better by using the radius of your pot: Area(in square cm) * 0.00428
    /// = pi * (radius cm)^2 * 0.00428
    /// So 35 cm diameter loses 4.1 liters per hour
    /// So 26 cm diameter loses 2.27238 liters per hour
    ///
    /// You can do even better by measuring them under your conditions.
    /// Because the actual losses depend on heat input (the strength of
    /// your boil, which can vary quite a lot), ambient humidity, ambient
    /// temperature, altitude, and air movement (how powerful your
    /// ventilation is).
    pub boil_evaporation_per_hour: Liters,

    /// How much water 1kg of grains soaks up.
    /// Estimate:  1.0 L/kg
    pub grain_absorption_per_kg: Liters,

    /// How much absorption happens from your hops (5 L/kg) is normal.
    /// Much less however if you squeeze hop bags afterwards.
    pub hops_absorption_per_kg: Liters,

    /// Your mash efficiency. Generally 0.6 - 0.9. Lower for bigger
    /// beers. Lower for traditional lautering. Higher for BIAB or
    /// straining through sieves. If you don't know, you can start
    /// with an estimate:
    ///    Traditional lautering: 0.68
    ///    BIAB: 0.84
    ///    Pour/strain: 0.87
    /// Best to measure this and average it over time.
    pub mash_efficiency: f32,

    /// The temperature of water added during mash infusions.
    /// Often people use boiling water, but it might be somewhat
    /// off boiling. You be the judge.
    pub infusion_temperature: Celsius,

    /// Room temperature, used as the initial temperature of the grains
    /// for strike and mash infusions.
    pub room_temperature: Celsius,

    /// If chilling is done with an ice bath
    pub ice_bath: bool,

    /// What size containers do you have for fermentation purposes?
    pub fermenters: Vec<Liters>,

    /// What size containers do you have for lagering?
    pub lagerers: Vec<Liters>,

    /// Packaging
    pub packaging: Packaging,
}

impl Brewery {
    /// Ice bath ice weight
    #[must_use]
    pub fn ice_weight(&self) -> Kilograms {
        Kilograms(self.max_kettle_volume.0 / 2.0)
    }

    /// Chilled water for ice bath
    #[must_use]
    pub fn chilled_water_volume(&self) -> Liters {
        self.max_kettle_volume
    }
}
