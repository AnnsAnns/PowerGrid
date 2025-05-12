use consumer::{Consumer, ConsumerType};
use log::{debug, info};
use powercable::{generate_unique_name, OfferHandler, ACCEPT_BUY_OFFER_TOPIC, TICK_TOPIC};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration, env};
use tokio::{sync::Mutex, task};
use topic_handler::{accept_offer_handler, tick_handler};

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
        .filter(None, log::LevelFilter::Warn)
        .init();
    info!("Starting consumer simulation...");

    let consumer_name: String = generate_unique_name();
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let consumer_type_str= env::var("CONSUMER_TYPE").expect("CONSUMER_TYPE not set");
    let consumer_type = ConsumerType::from_str(&consumer_type_str);
    let consumer =
        Consumer::new(latitude, longitude, consumer_name.clone(), consumer_type);

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
    info!("Connected to MQTT broker as {}", consumer_name);

    let shared_consumer = Arc::new(Mutex::new(ConsumerHandler {
        name: consumer_name.clone(),
        consumer,
        client: client.clone(),
        offer_handler: OfferHandler::new(),
    }));

    while let Ok(notification) = eventloop.poll().await {
        debug!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                TICK_TOPIC => {
                    let _ = task::spawn(tick_handler(shared_consumer.clone(), p.payload));
                }
                ACCEPT_BUY_OFFER_TOPIC => {
                    let _ = task::spawn(accept_offer_handler(shared_consumer.clone(), p.payload));
                }
                _ => {
                    let _ = task::spawn(async move {
                        debug!("Unknown topic: {}", p.topic);
                    });
                }
            }
        }
    }
    info!("Exiting consumer simulation...");
}
