use fake::{faker::lorem::de_de::Word, Fake};
use rumqttc::{MqttOptions, AsyncClient, QoS};
use serde::de;
use tokio::{task, time};
use std::{result, time::Duration};
use log::{info, debug};

mod charger;

#[tokio::main]
async fn main() {
    env_logger::builder()
    .filter(None, log::LevelFilter::Debug)
    .init();
    info!("Starting turbine simulation...");
    let charger_name: String = Word().fake();
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let mut charger = charger::Charger::new(latitude, longitude, 10000, 500, 5, charger_name);

    let mut mqttoptions = MqttOptions::new("charger", powercable::MQTT_BROKER, powercable::MQTT_BROKER_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce).await.unwrap();
    client.subscribe(powercable::POWER_TRANSFORMER_DIFF_TOPIC, QoS::AtMostOnce).await.unwrap();
    info!("Connected to MQTT broker");

    while let Ok(notification) = eventloop.poll().await {
        debug!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                powercable::TICK_TOPIC => {
                    
                },
                _ => {
                    info!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
}