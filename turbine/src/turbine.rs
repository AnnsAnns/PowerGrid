const GAS_CONSTANT: f64 = 287.1; // J/(kg·K)

struct Turbine {
    id: usize,
    rotor_dimension: f64,
    wind_speed: f64,
    power_coefficient: f64,
    temperature: f64,
    air_pressure: f64,
}

impl Default for Turbine {
    fn default() -> Self {
        Turbine {
            id: 0,
            rotor_dimension: 0.0,
            wind_speed: 0.0,
            power_coefficient: 0.0,
            temperature: 0.0,
            air_pressure: 0.0,
        }
    }
}

impl Turbine {
    pub fn new(
        id: usize,
        rotor_dimension: f64,
        wind_speed: f64,
        power_coefficient: f64,
        temperature: f64,
        air_pressure: f64,
    ) -> Self {
        Turbine {
            id,
            rotor_dimension,
            wind_speed,
            power_coefficient,
            temperature,
            air_pressure,
        }
    }

    /// Calculate the current power output (in Watt) of the turbine
    /// based on the wind speed and rotor dimension.
    pub fn calculate_power(&self) -> f64 {
        // Power = 0.5 * air_density * rotor_area * wind_speed^3
        let rotor_area = std::f64::consts::PI * (self.rotor_dimension / 2.0).powi(2);
        0.5 * self.calculate_air_density() * rotor_area * self.wind_speed.powi(3)
    }

    pub fn calculate_air_density(&self) -> f64 {
        // Air density = pressure / (gas_constant * temperature)
        self.air_pressure / (GAS_CONSTANT * self.temperature)
    }
}