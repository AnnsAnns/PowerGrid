use crate::meta_data::MetaDataWrapper;

use super::Turbine;

const GAS_CONSTANT: f64 = 287.1; // J/(kg·K)

impl Turbine {
    /// Calculate the current power output (in Watt) of the turbine
    /// based on the wind speed and rotor dimension.
    pub fn calculate_power(&self, air_density: f64, wind_speed: f64) -> f64 {
        // Power = 0.5 * air_density * rotor_area * wind_speed^3
        let rotor_area = std::f64::consts::PI * (self.get_rotor_dimension() / 2.0).powi(2);
        0.5 * air_density * rotor_area * wind_speed.powi(3)
    }

    pub fn calculate_air_density(&self, air_pressure: f64, temperature: f64) -> f64 {
        // Air density = pressure / (gas_constant * temperature)
        air_pressure / (GAS_CONSTANT * temperature)
    }
}