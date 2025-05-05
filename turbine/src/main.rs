use rumqttc::{MqttOptions, AsyncClient, QoS};
use serde::de;
use tokio::{task, time};
use std::{result, time::Duration};
use log::{info, debug};

mod turbine;
mod meta_data;
mod parsing;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting turbine simulation...");
    
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();

    let mut turbine = turbine::Turbine::new(
        turbine::random_rotor_dimension(),
        latitude,
        longitude,
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature).await.unwrap(),
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind).await.unwrap(),
    );

    let mut mqttoptions = MqttOptions::new("turbine", "mosquitto_broker", 1884);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce).await.unwrap();
    info!("Connected to MQTT broker");

    info!("Turbine simulation started. Waiting for messages...");
    while let Ok(notification) = eventloop.poll().await {
        info!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            if p.topic == powercable::TICK_TOPIC {
                turbine.tick();
                turbine.approximate_wind_data().await;
                turbine.approximate_temperature_data().await;
                let current_power = turbine.get_power_output();
                info!("Current power output: {} Watt", current_power);
                let result = client.publish("turbine/power", QoS::ExactlyOnce, false, current_power.to_string()).await;
                debug!("Result of publish: {:?}", result);
            }
        }
    }
}