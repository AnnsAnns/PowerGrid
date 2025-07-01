use tracing::{debug, info, trace, warn};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use powercable::{generate_seed, OfferHandler, ACCEPT_BUY_OFFER_TOPIC, CONFIG_CONSUMER, CONFIG_CONSUMER_SCALE, TICK_TOPIC};
use consumer::{Consumer, ConsumerType};
use topic_handler::{accept_offer_handler, tick_handler, scale_handler};

pub mod consumer;
mod topic_handler;
mod map_handler;

type SharedConsumer = Arc<Mutex<ConsumerHandler>>;

struct ConsumerHandler {
    pub consumer: Consumer,
    pub client: AsyncClient,
    pub offer_handler: OfferHandler,
}

pub async fn start_consumer(consumer_type: ConsumerType, i: u64) {
    let consumer_type_str = consumer_type.to_string();
    let seed = generate_seed(i, powercable::OwnType::Consumer);
    let mut consumer =
        Consumer::new(powercable::generate_rnd_pos(seed), consumer_type);

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
        .subscribe(TICK_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", TICK_TOPIC);
    client
        .subscribe(ACCEPT_BUY_OFFER_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", ACCEPT_BUY_OFFER_TOPIC);
    client
        .subscribe(CONFIG_CONSUMER_SCALE, QoS::ExactlyOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", CONFIG_CONSUMER_SCALE);
    client
        .subscribe(CONFIG_CONSUMER, QoS::ExactlyOnce)
        .await
        .unwrap();
    trace!("Subscribed to {} topic", CONFIG_CONSUMER);

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
                    task::spawn(tick_handler(shared_consumer.clone(), p.payload));
                }
                ACCEPT_BUY_OFFER_TOPIC => {
                    task::spawn(accept_offer_handler(shared_consumer.clone(), p.payload));
                }
                CONFIG_CONSUMER_SCALE => {
                    task::spawn(scale_handler(shared_consumer.clone(), p.payload));
                }
                CONFIG_CONSUMER => {
                    task::spawn(topic_handler::show_handler(shared_consumer.clone(), p.payload));
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
    info!("Exiting consumer simulation...");
}
