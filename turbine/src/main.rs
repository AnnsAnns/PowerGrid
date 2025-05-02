use rumqttc::{MqttOptions, AsyncClient, QoS};
use serde::de;
use tokio::{task, time};
use std::{result, time::Duration};
use log::{info, debug};

mod turbine;
mod meta_data;
mod parsing;

const ROTOR_DIMENSION: f64 = 101.0; // in meters

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting turbine simulation...");
    let mut mqttoptions = MqttOptions::new("turbine", "mosquitto_broker", 1884);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();
    info!("Connected to MQTT broker");
    
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();

    let mut turbine = turbine::Turbine::new(
        ROTOR_DIMENSION,
        latitude,
        longitude,
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature).await.unwrap(),
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind).await.unwrap(),
    );
    
    task::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            info!("Received = {:?}", notification);
        }
    });

    info!("Turbine simulation started. Waiting for messages...");
    // Keep the main thread alive to receive messages
    loop {
        turbine.approximate_wind_data().await;
        turbine.approximate_temperature_data().await;
        let current_power = turbine.get_power_output();
        info!("Current power output: {} Watt", current_power);
        let result = client.publish("turbine/power", QoS::ExactlyOnce, false, current_power.to_string()).await;
        debug!("Result of publish: {:?}", result);

        time::sleep(Duration::from_secs(1)).await;

        turbine.tick();
        info!("Tick: {}", turbine.get_tick());
    }
}