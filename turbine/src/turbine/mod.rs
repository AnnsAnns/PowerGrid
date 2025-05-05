pub mod calculations;
pub mod base;
pub mod aggregator;
pub mod power_coefficient;

pub use base::Turbine;
use rand::Rng;

/// Generates a random rotor dimension between 50 and 150 meters.
pub fn random_rotor_dimension() -> f64 {
    let mut rng = rand::rng();
    let rotor_dimension = rng.random_range(50.0..150.0);
    rotor_dimension
}