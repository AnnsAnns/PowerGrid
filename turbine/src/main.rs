use log::{debug, info};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::de;
use std::{result, time::Duration};
use tokio::{task, time};

mod meta_data;
mod parsing;
mod turbine;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
    info!("Starting turbine simulation...");

    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();

    let mut turbine = turbine::Turbine::new(
        turbine::random_rotor_dimension(),
        latitude,
        longitude,
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature)
            .await
            .unwrap(),
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind)
            .await
            .unwrap(),
    );

    let mut mqttoptions = MqttOptions::new(
        "turbine",
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(20));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    info!("Turbine simulation started. Waiting for messages...");
    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                debug!("Event = {v:?}");
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = v {
                    if p.topic == powercable::TICK_TOPIC {
                        turbine.tick();
                        turbine.approximate_wind_data().await;
                        turbine.approximate_temperature_data().await;
                        let current_power = turbine.get_power_output();
                        info!("Current power output: {} Watt", current_power);
                        let result = client
                            .publish(
                                powercable::POWER_NETWORK_TOPIC,
                                QoS::ExactlyOnce,
                                false,
                                current_power.to_string(),
                            )
                            .await;
                        debug!("Result of publish: {:?}", result);
                    }
                }
            }
            Err(e) => {
                debug!("Error = {e:?}");
                break;
            }
        }
    }
}
