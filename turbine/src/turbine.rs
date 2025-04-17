struct Turbine {
    id: usize,
    wind_power: f64,
    rotor_dimension: f64,
    air_density: f64,
    wind_speed: f64,
    power_coefficient: f64,
}

impl Default for Turbine {
    fn default() -> Self {
        Turbine {
            id: 0,
            wind_power: 0.0,
            rotor_dimension: 0.0,
            air_density: 0.0,
            wind_speed: 0.0,
            power_coefficient: 0.0,
        }
    }
}