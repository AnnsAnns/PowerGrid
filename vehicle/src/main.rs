use log::{debug, info, warn};
use powercable::{charger::ChargeOffer, CHARGER_OFFER, TICK_TOPIC, WORLDMAP_EVENT_TOPIC};
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
    pub vehicle: Vehicle,
    pub charge_offers: Vec<ChargeOffer>,
    pub target_charger: Option<ChargeOffer>,
    pub client: AsyncClient,
}

#[tokio::main]
async fn main() {
    // init vehicle
    let vehicle_name: String = powercable::generate_unique_name();
    let vehicle = Vehicle::new(vehicle_name.clone(), powercable::generate_rnd_pos());

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
        .subscribe(powercable::TICK_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::WORLDMAP_EVENT_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client.subscribe(CHARGER_OFFER, QoS::ExactlyOnce).await.unwrap();
    info!("Connected to MQTT broker");

    let shared_vehicle = Arc::new(Mutex::new(VehicleHandler {
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
                    let payload = match ChargeOffer::from_bytes(p.payload) {
                        Ok(offer) => offer,
                        Err(e) => {
                            warn!("Failed to deserialize ChargeOffer: {}", e);
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
