mod turbine;
mod meta_data;
mod parsing;

const LATITUDE: f64 = 51.80449506075378;
const LONGITUDE: f64 = 6.247927193955036;
const ROTOR_DIMENSION: f64 = 120.0; // in meters

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
}
