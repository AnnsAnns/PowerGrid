use turbine::power_coefficient::find_closest_coefficient_for_wind;

mod turbine;
mod meta_data;
mod parsing;

const LATITUDE: f64 = 51.80449506075378;
const LONGITUDE: f64 = 6.247927193955036;
const ROTOR_DIMENSION: f64 = 101.0; // in meters

#[tokio::main]
async fn main() {
    println!("Creating turbine with a rotor dimension of {} meters at latitude {} and longitude {}", ROTOR_DIMENSION, LATITUDE, LONGITUDE);

    let mut turbine = turbine::Turbine::new(
        ROTOR_DIMENSION,
        LATITUDE,
        LONGITUDE,
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature).await.unwrap(),
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind).await.unwrap(),
    );

    println!("Fetching metadata for the turbine...");
    turbine.get_closest_wind_stations().await;
    turbine.get_closest_temperature_stations().await;
    turbine.approximate_wind_data().await;
    turbine.approximate_temperature_data().await;
    println!("🧑‍🔬 Power Coefficient: {} Cp", find_closest_coefficient_for_wind(turbine.approximate_wind.as_ref().unwrap().wind_strength));
    println!("⛅ Current temperature {} °C", turbine.approximate_temperature.as_ref().unwrap().air_temperature);
    println!("🍃 Current wind strength {} m/s", turbine.approximate_wind.as_ref().unwrap().wind_strength);
    println!("🌬️ The Turbines rotor area is: {} m²", turbine.get_rotor_area());
    println!("🌍 The Turbines location is: {}°N, {}°E", turbine.get_latitude(), turbine.get_longitude());
    //println!("📊 Estimation based on E-101 is roughly {} Watt", estimate_power_at_wind(turbine.approximate_wind.as_ref().unwrap().wind_strength));
    println!("⚡ The Turbines current power output is: {} Kilowatt", turbine.get_power_output());
}
