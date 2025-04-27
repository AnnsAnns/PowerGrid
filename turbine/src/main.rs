use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::{result, time::Duration};

mod turbine;
mod meta_data;
mod parsing;

const LATITUDE: f64 = 51.80449506075378;
const LONGITUDE: f64 = 6.247927193955036;
const ROTOR_DIMENSION: f64 = 101.0; // in meters

#[tokio::main]
async fn main() {
    println!("Starting turbine simulation...");
    let mut mqttoptions = MqttOptions::new("turbine", "mosquitto_broker", 1884);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();
    println!("Connected to MQTT broker");
    
    let mut turbine = turbine::Turbine::new(
        ROTOR_DIMENSION,
        LATITUDE,
        LONGITUDE,
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature).await.unwrap(),
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind).await.unwrap(),
    );
    turbine.get_closest_wind_stations().await;
    turbine.get_closest_temperature_stations().await;
    turbine.approximate_wind_data().await;
    turbine.approximate_temperature_data().await;
    
    let current_power = turbine.get_power_output();
    println!("Current power output: {} Watt", current_power);
    let result = client.publish("turbine/power", QoS::ExactlyOnce, false, current_power.to_string()).await;
    print!("Result of publish: {:?}", result);
    
    task::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            println!("Received = {:?}", notification);
        }
    });    
}