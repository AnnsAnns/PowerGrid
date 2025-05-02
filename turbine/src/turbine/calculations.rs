use log::info;

use crate::meta_data::MetaDataWrapper;

use super::{power_coefficient::{find_closest_coefficient_for_wind, get_wind_power_coefficients_e101}, Turbine};

const GAS_CONSTANT: f64 = 287.1; // J/(kg·K)
pub const AIR_PRESSURE: f64 = 101.325; // Pa

impl Turbine {
    pub fn get_power_output(&self) -> f64 {
        // Assuming a standard air density of 1.225 kg/m^3 at sea level
        let air_density = self.calculate_air_density(AIR_PRESSURE, self.approximate_temperature.as_ref().unwrap().air_temperature); // Standard temperature in Kelvin
        info!("✈️ Air density: {} kg/m³", air_density);
        self.calculate_power(air_density, self.approximate_wind.as_ref().unwrap().wind_strength)
    }

    /// Calculate the current power output (in Watt) of the turbine
    /// based on the wind speed and rotor dimension.
    pub fn calculate_power(&self, air_density: f64, wind_speed: f64) -> f64 {
        // Power = 0.5 * air_density * rotor_area * wind_speed^3 * Cp
        0.5 * self.get_rotor_area() * (air_density * wind_speed.powi(3)) * find_closest_coefficient_for_wind(wind_speed)
    }

    pub fn calculate_air_density(&self, air_pressure: f64, temperature: f64) -> f64 {
        // Air density = pressure / (gas_constant * temperature)
        let temperature_in_kelvin = temperature + 273.15; // Convert Celsius to Kelvin
        air_pressure / (GAS_CONSTANT * temperature_in_kelvin)
    }
}