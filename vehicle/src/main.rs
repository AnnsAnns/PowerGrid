use battery::Battery;
use log::{debug, info};
use powercable::{charger::ChargeOffer, CHARGER_OFFER, TICK_TOPIC, WORLDMAP_EVENT_TOPIC};
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use topic_handler::{tick_handler, worldmap_event_handler};
use vehicle::Vehicle;

mod battery;
mod charger_handling;
mod database;
mod topic_handler;
mod vehicle;

type SharedVehicle = Arc<Mutex<VehicleHandler>>;

struct VehicleHandler {
    pub name: String,
    pub vehicle: Vehicle,
    pub charge_offers: Vec<ChargeOffer>,
    pub target_charger: Option<ChargeOffer>,
    pub client: AsyncClient,
}

#[tokio::main]
async fn main() {
    // init vehicle
    let vehicle_name: String = powercable::generate_unique_name();
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let vehicle = Vehicle::new(vehicle_name.clone(), latitude, longitude);

    let log_path = format!(
        "logs/vehicle_{}.log",
        vehicle_name.clone().replace(" ", "_")
    );
    let _log2 = log2::open(log_path.as_str()).level("info").start();

    info!("{:#?}", vehicle);

    let mut mqttoptions = MqttOptions::new(
        vehicle_name.clone(),
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
        .subscribe(powercable::WORLDMAP_EVENT_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client.subscribe(CHARGER_OFFER, QoS::AtMostOnce).await.unwrap();
    info!("Connected to MQTT broker");

    let shared_vehicle = Arc::new(Mutex::new(VehicleHandler {
        name: vehicle_name.clone(),
        vehicle,
        target_charger: None,
        charge_offers: Vec::new(),
        client: client.clone(),
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
                    let payload = match serde_json::from_slice::<ChargeOffer>(&p.payload) {
                        Ok(offer) => offer,
                        Err(e) => {
                            debug!("Failed to deserialize ChargeOffer: {}", e);
                            continue;
                        }
                    };
                    let _ = task::spawn(charger_handling::receive_offer(shared_vehicle.clone(), payload));
                }
                _ => {
                    let _ = task::spawn(async move {
                        debug!("Unknown topic: {}", p.topic);
                    });
                }
            }
        }
    }
    info!("Exiting electric vehicle simulation...");
}
