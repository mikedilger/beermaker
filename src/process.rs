use crate::Packaging;
use crate::ingredients::{AcidConcentration, SaltConcentration, WaterProfile};
use crate::units::prelude::*;
use serde::{Deserialize, Serialize};

/// Process by which the beer is made, independent of recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    /// Water profile
    pub water_profile: WaterProfile,

    /// Water salts
    pub water_salts: Vec<SaltConcentration>,

    /// Water acids/bases
    pub water_acids: Vec<AcidConcentration>,

    /// The effective volume of your boil kettle... the max level you can boil at.
    /// Make sure to leave some space for rolling boils and foam.
    pub kettle_volume: Liters,

    /// How much is left behind in the kettle after the boil
    pub kettle_losses: Liters,

    /// Volume lost per hour during the boil.
    ///
    /// For a 5 gallon homebrew, this is typically 4 - 5 liters.
    /// You can do better by using the radius of your pot: Area(in square cm) * 0.00428
    /// = pi * (radius cm)^2 * 0.00428
    /// So 35 cm loses 4.1 liters per hour
    /// So 26 cm loses 2.27238 liters per hour
    ///
    /// You can do even better by measuring them under your conditions.
    /// Because the actual losses depend on heat input (the strength of
    /// your boil, which can vary quite a lot), ambient humidity, ambient
    /// temperature, altitude, and air movement (how powerful your
    /// ventilation is).
    pub boil_evaporation_per_hour: Liters,

    /// How much water 1kg of grains soaks up.
    ///
    /// BIAB without squeezing: 0.375 L/kg (or 0.667 L/kg)
    /// BIAB with squeezing: 0.28 - 0.51 L/kg
    /// Traditional mash tun: 0.8 - 1.2 L/kg (0.8 - 0.96)
    pub grain_absorption_per_kg: Liters,

    /// How much absorption happens from your hops (5 L/kg) is normal.
    /// Much less however if you squeeze hop bags afterwards.
    pub hops_absorption_per_kg: Liters,

    /// Your mash efficiency. Generally 0.6 - 0.9. Lower for bigger
    /// beers. Higher for BIAB or straining through sieves.
    /// Best to measure this and average it over time.
    pub mash_efficiency: f32,

    /// If chilling is done with an ice bath
    pub ice_bath: bool,

    /// The volume that the fermenter can handle, leaving room for
    /// head/krausen. This is also known as batch size.
    pub ferment_volume: Liters,

    /// Fermenter loss percent. This is the trub at the bottom that
    /// you cannot rack or drink.
    // TODO this may depend on recipe
    pub ferment_loss_percent: f32,

    /// Room temperature, used as the initial temperature of the grains
    /// for strike and mash infusions.
    pub room_temperature: Celsius,

    /// The temperature of water added during mash infusions.
    /// Often people use boiling water, but it might be somewhat
    /// off boiling. You be the judge.
    pub infusion_temperature: Celsius,

    /// Partial boil dilution. Usually 0, but if you dilute your
    /// post-boiled pre-fermented wort (e.g. you do a partial boil)
    /// then indicate what volume that dilution would be.
    /// This technique allows you to use a smaller kettle, and also
    /// has affects on the outcome.
    pub partial_boil_dilution: Liters,

    /// Post-ferment dilution, allows fermenting at higher gravity,
    /// while providing a lower ABV product.
    pub post_ferment_dilution: Liters,

    /// Packaging
    pub packaging: Packaging,
}

impl Process {
    /// The water profile (after salts and acids)
    #[must_use]
    pub fn adjusted_water_profile(&self) -> WaterProfile {
        let mut profile = self.water_profile;

        for salt_conc in &self.water_salts {
            profile.add_salt(*salt_conc);
        }

        for acid_conc in &self.water_acids {
            profile.add_acid(*acid_conc);
        }

        profile
    }

    /// The product volume at the end
    #[must_use]
    pub fn product_volume(&self) -> Liters {
        self.post_ferment_volume() + self.post_ferment_dilution
    }

    /// The post ferment, pre-diluted product
    #[must_use]
    pub fn post_ferment_volume(&self) -> Liters {
        self.ferment_volume - self.ferment_losses()
    }

    /// Ferment losses
    #[must_use]
    pub fn ferment_losses(&self) -> Liters {
        self.ferment_volume * self.ferment_loss_percent
    }

    /// The post-boil volume, pre kettle losses
    #[must_use]
    pub fn post_boil_pre_loss_volume(&self) -> Liters {
        self.post_boil_volume() + self.kettle_losses
    }

    /// The post-boil volume, after kettle losses
    #[must_use]
    pub fn post_boil_volume(&self) -> Liters {
        self.ferment_volume - self.partial_boil_dilution
    }
}
