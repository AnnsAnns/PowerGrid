use consumer::{Consumer, ConsumerType};
use log::{debug, info, trace, warn, };
use powercable::*;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration, env};
use tokio::{sync::Mutex, task};
use topic_handler::{accept_offer_handler, tick_handler, scale_handler};
use serde_json::json;

mod consumer;
mod topic_handler;

type SharedConsumer = Arc<Mutex<ConsumerHandler>>;

struct ConsumerHandler {
    pub name: String,
    pub consumer: Consumer,
    pub client: AsyncClient,
    pub offer_handler: OfferHandler,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
    info!("Starting consumer simulation...");

    let consumer_name = env::var("CONSUMER_NAME").unwrap_or(generate_unique_name());
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let consumer_type_str= env::var("CONSUMER_TYPE").unwrap_or(ConsumerType::G0.to_string());
    let consumer_type = ConsumerType::from_str(&consumer_type_str);
    let consumer =
        Consumer::new(latitude, longitude, consumer_name.clone(), consumer_type);
    debug!("Created {} of type {}", consumer_name, consumer_type_str);

    let mut mqttoptions = MqttOptions::new(
        consumer_name.clone(),
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::ACCEPT_BUY_OFFER_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::POWER_CONSUMER_SCALE, QoS::AtMostOnce)
        .await
        .unwrap();
    debug!("Connected to MQTT broker as {}", consumer_name);

    
    let shared_consumer = Arc::new(Mutex::new(ConsumerHandler {
        name: consumer_name.clone(),
        consumer,
        client: client.clone(),
        offer_handler: OfferHandler::new(),
    }));

    // while loop over notifications
    while let Ok(notification) = eventloop.poll().await {
        trace!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                TICK_TOPIC => {
                    let _ = task::spawn(tick_handler(shared_consumer.clone(), p.payload));
                }
                ACCEPT_BUY_OFFER_TOPIC => {
                    let _ = task::spawn(accept_offer_handler(shared_consumer.clone(), p.payload));
                }
                POWER_CONSUMER_SCALE => {
                    let _ = task::spawn( scale_handler(shared_consumer.clone(), p.payload));
                }
                _ => {
                    let _ = task::spawn(async move {
                        warn!("Unknown topic: {}", p.topic);
                    });
                }
            }
        }
    }
    info!("Exiting consumer simulation...");
}
