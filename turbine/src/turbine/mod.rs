pub mod calculations;
pub mod base;
pub mod aggregator;
pub mod power_coefficient;

pub use base::Turbine;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// Generates a random rotor dimension between 50 and 150 meters.
pub fn random_rotor_dimension(seed: u64) -> f64 {
    let mut rng = StdRng::seed_from_u64(seed);
    
    rng.random_range(50.0..150.0)
}