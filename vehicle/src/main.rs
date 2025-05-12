use battery::Battery;
use log::info;
use rand::Rng;
use vehicle::Vehicle;

mod vehicle;
mod battery;

fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
    info!("Starting electric vehicle simulation...");

    // init battery
    let mut rng = rand::rng();
    let battery = Battery::new(
        rng.random_range(21.3..118.0),
        25.0,
        rng.random_range(0.02..0.12),
        rng.random_range(7.0..350.0),
        rng.random_range(30.0..600.0),
        rng.random_range(0.90..0.98),
        rng.random_range(0.85..0.95),
    );

    // init vehicle
    let vehicle_name: String = powercable::generate_unique_name();
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let mut vehicle = Vehicle::new(
        vehicle_name,
        latitude,
        longitude,
        battery,
    );

    println!("{:#?}", vehicle);
    info!("Exiting electric vehicle simulation...");
}
