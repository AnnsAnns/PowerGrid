use tracing::{debug, info, warn};
use powercable::{charger::ChargeOffer, CHARGER_CHARGING_ACK, CHARGER_OFFER, CONFIG_VEHICLE_SCALE, CONFIG_VEHICLE, MQTT_BROKER, MQTT_BROKER_PORT, TICK_TOPIC, CONFIG_VEHICLE_ALGORITHM, WORLDMAP_EVENT_TOPIC};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use topic_handler::{tick_handler, worldmap_event_handler};
use charger_handling::{receive_offer};
use vehicle::Vehicle;

use crate::{charger_handling::get_ack_handling, topic_handler::{algorithm_handler, scale_handler, show_handler}};

mod battery;
mod charger_handling;
mod database;
mod topic_handler;
mod vehicle;

type SharedVehicle = Arc<Mutex<VehicleHandler>>;

struct VehicleHandler {
    pub vehicle: Vehicle,
    pub charge_offers: Vec<ChargeOffer>,
    pub target_charger: Option<ChargeOffer>,
    pub client: AsyncClient,
    pub seed: u64,
}

pub async fn start_vehicle(i: u64) {
    // init vehicle
    let seed = powercable::generate_seed(i, powercable::OwnType::Vehicle);
    let vehicle_name: String = powercable::generate_unique_name(seed);
    let vehicle = Vehicle::new(vehicle_name.clone(), powercable::generate_rnd_pos(seed), seed);
    info!("{:#?}", vehicle);

    let mut mqttoptions = MqttOptions::new(
        vehicle_name.clone(),
        MQTT_BROKER,
        MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(TICK_TOPIC, QoS::ExactlyOnce)
        .await.unwrap();
    client
        .subscribe(WORLDMAP_EVENT_TOPIC, QoS::ExactlyOnce)
        .await.unwrap();
    client.subscribe(CHARGER_OFFER, QoS::ExactlyOnce)
        .await.unwrap();
    client
        .subscribe(CHARGER_CHARGING_ACK, QoS::ExactlyOnce)
        .await.unwrap();
    client
        .subscribe(CONFIG_VEHICLE_SCALE, QoS::ExactlyOnce)
        .await.unwrap();
    client
        .subscribe(CONFIG_VEHICLE_ALGORITHM, QoS::ExactlyOnce)
        .await.unwrap();
    client
        .subscribe(CONFIG_VEHICLE, QoS::ExactlyOnce)
        .await.unwrap();
    debug!("Connected to MQTT broker");

    let shared_vehicle = Arc::new(Mutex::new(VehicleHandler {
        vehicle,
        target_charger: None,
        charge_offers: Vec::new(),
        client: client.clone(),
        seed,
    }));

    while let Ok(notification) = eventloop.poll().await {
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                TICK_TOPIC => {
                    let _ = task::spawn(tick_handler(shared_vehicle.clone(), p.payload));
                }
                WORLDMAP_EVENT_TOPIC => {
                    let _ = task::spawn(worldmap_event_handler(shared_vehicle.clone(), p.payload));
                }
                CHARGER_OFFER => {
                    let _ = task::spawn(receive_offer(shared_vehicle.clone(), p.payload));
                }
                CHARGER_CHARGING_ACK => {
                    let _ = task::spawn(get_ack_handling(shared_vehicle.clone(), p.payload));
                }
                CONFIG_VEHICLE_SCALE => {
                    let _ = task::spawn(scale_handler(shared_vehicle.clone(), p.payload));
                }
                CONFIG_VEHICLE_ALGORITHM => {
                    let _ = task::spawn(algorithm_handler(shared_vehicle.clone(), p.payload));
                }
                CONFIG_VEHICLE => {
                    let _ = task::spawn(show_handler(shared_vehicle.clone(), p.payload));
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
    info!("Exiting electric vehicle simulation...");
}