use charger::Charger;
use log::{debug, info};
use offer_handling::ReservedOffer;
use powercable::{generate_unique_name, OfferHandler, ACCEPT_BUY_OFFER_TOPIC, TICK_TOPIC, CHARGER_REQUEST, generate_rnd_pos};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use topic_handler::{accept_offer_handler, tick_handler};
use car_handling::receive_request;

mod charger;
mod topic_handler;
mod car_handling;
mod offer_handling;

type SharedCharger = Arc<Mutex<ChargerHandler>>;

struct ChargerHandler {
    pub name: String,
    pub charger: Charger,
    pub client: AsyncClient,
    pub currently_reserved_for: Vec<ReservedOffer>,
    pub offer_handler: OfferHandler,
    pub consumed_last_tick: f64,
}

#[tokio::main]
async fn main() {
    let charger_name: String = format!("Charger {}", generate_unique_name());
    let log_path = format!("logs/charger_{}.log", charger_name.clone().replace(" ", "_"));
    let _log2 = log2::open(log_path.as_str()).level("info").start();
    info!("Starting charger simulation...");

    let charger =
        charger::Charger::new(generate_rnd_pos(), 5000, 100, 5, charger_name.clone());

    let mut mqttoptions = MqttOptions::new(
        charger_name.clone(),
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(powercable::TICK_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::ACCEPT_BUY_OFFER_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::CHARGER_REQUEST, QoS::ExactlyOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    let shared_charger = Arc::new(Mutex::new(ChargerHandler {
        name: charger_name.clone(),
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
                    let _ = task::spawn(tick_handler(shared_charger.clone(), p.payload));
                }
                ACCEPT_BUY_OFFER_TOPIC => {
                    let _ = task::spawn(accept_offer_handler(shared_charger.clone(), p.payload));
                }
                CHARGER_REQUEST => {
                    let _ = task::spawn(receive_request(shared_charger.clone(), p.payload));
                }
                // wenn seine offer nicht accepted wurde muss er die offer releasen check_accepted_offers
                _ => {
                    let _ = task::spawn(async move {
                        debug!("Unknown topic: {}", p.topic);
                    });
                }
            }
        }
    }
    info!("Exiting charger simulation...");
}
