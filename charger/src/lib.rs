use car_handling::{accept_handler, answer_get, receive_request};
use charger::Charger;
use tracing::{info, warn};
use offer_handling::ReservedOffer;
use powercable::{
    generate_rnd_pos, generate_seed, generate_unique_name, OfferHandler, OwnType, ACCEPT_BUY_OFFER_TOPIC, CHARGER_ACCEPT, CHARGER_CHARGING_GET, CHARGER_CHARGING_RELEASE, CHARGER_REQUEST, TICK_TOPIC
};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use topic_handler::{accept_offer_handler, tick_handler};

use crate::car_handling::release_car;

mod car_handling;
mod charger;
mod offer_handling;
mod topic_handler;

type SharedCharger = Arc<Mutex<ChargerHandler>>;

struct ChargerHandler {
    pub charger: Charger,
    pub client: AsyncClient,
    pub currently_reserved_for: Vec<ReservedOffer>,
    pub offer_handler: OfferHandler,
    pub consumed_last_tick: f64,
}

pub async fn start_charger(i: u64) {
    let seed = generate_seed(i, OwnType::Charger);

    let charger_name: String = format!("Charger {}", generate_unique_name(seed));
    info!("Starting charger simulation...");

    let charger = Charger::new(charger_name.clone(), generate_rnd_pos(seed), 100, 300, 5);
    info!("{:#?}", charger);

    let mut mqttoptions = MqttOptions::new(
        charger_name.clone(),
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(TICK_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(ACCEPT_BUY_OFFER_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(CHARGER_REQUEST, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(CHARGER_ACCEPT, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(CHARGER_CHARGING_GET, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(CHARGER_CHARGING_RELEASE, QoS::ExactlyOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    let shared_charger = Arc::new(Mutex::new(ChargerHandler {
        charger,
        client: client.clone(),
        offer_handler: OfferHandler::new(),
        currently_reserved_for: Vec::new(),
        consumed_last_tick: 0.0,
    }));

    while let Ok(notification) = eventloop.poll().await {
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                TICK_TOPIC => {
                    task::spawn(tick_handler(shared_charger.clone(), p.payload));
                }
                ACCEPT_BUY_OFFER_TOPIC => {
                    task::spawn(accept_offer_handler(shared_charger.clone(), p.payload));
                }
                CHARGER_REQUEST => {
                    task::spawn(receive_request(shared_charger.clone(), p.payload));
                }
                CHARGER_ACCEPT => {
                    task::spawn(accept_handler(shared_charger.clone(), p.payload));
                }
                CHARGER_CHARGING_GET => {
                    task::spawn(answer_get(shared_charger.clone(), p.payload));
                }
                CHARGER_CHARGING_RELEASE => {
                    task::spawn(release_car(shared_charger.clone(), p.payload));
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
    println!("Exiting charger simulation...");
}