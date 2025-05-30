use log::{debug, info, trace, warn};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration, env};
use tokio::{sync::Mutex, task};

use powercable::*;
use consumer::{Consumer, ConsumerType};
use topic_handler::{accept_offer_handler, tick_handler, scale_handler};

mod consumer;
mod topic_handler;
mod map_handler;

type SharedConsumer = Arc<Mutex<ConsumerHandler>>;

struct ConsumerHandler {
    pub consumer: Consumer,
    pub client: AsyncClient,
    pub offer_handler: OfferHandler,
}

#[tokio::main]
async fn main() {
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let consumer_type_str= env::var("CONSUMER_TYPE").unwrap_or(ConsumerType::H.to_string()); // TODO: simplify
    let consumer_type = ConsumerType::from_str(&consumer_type_str); // TODO: simplify
    let mut consumer =
        Consumer::new(latitude, longitude, consumer_type);

    let log_path = format!("logs/consumer_{}.log", consumer_type_str.replace(" ", "_"));
    let _log2 = log2::open(log_path.as_str()).level("info").start();
    debug!("Created {}", consumer_type_str);

    let mut mqttoptions = MqttOptions::new(
        consumer_type.to_string(),
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    debug!("Connected to MQTT broker as {}", consumer_type.to_string());

    client
        .subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", TICK_TOPIC);
    client
        .subscribe(ACCEPT_BUY_OFFER_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", ACCEPT_BUY_OFFER_TOPIC);
    client
        .subscribe(powercable::POWER_CONSUMER_SCALE, QoS::AtMostOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", POWER_CONSUMER_SCALE);

    consumer.parse_csv().await.unwrap();
    
    let shared_consumer = Arc::new(Mutex::new(ConsumerHandler {
        consumer,
        client: client.clone(),
        offer_handler: OfferHandler::new(),
    }));

    task::spawn(map_handler::map_update_task(shared_consumer.clone()));

    // while loop over notifications
    while let Ok(notification) = eventloop.poll().await {
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
