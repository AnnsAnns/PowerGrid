use log::{debug, info};
use powercable::{charger::ChargeOffer, CHARGER_CHARGING_ACK, CHARGER_OFFER, CONFIG_SCALE_VEHICLE, MQTT_BROKER, MQTT_BROKER_PORT, TICK_TOPIC, CONFIG_VEHICLE_STUPID, WORLDMAP_EVENT_TOPIC};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use topic_handler::{tick_handler, worldmap_event_handler};
use charger_handling::{receive_offer};
use vehicle::Vehicle;

use crate::{charger_handling::get_ack_handling, topic_handler::{scale_handler, stupid_handler}};

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
    let _log2 = log2::open(log_path.as_str())
        .level("debug")
        .module_filter(|module| !module.starts_with("rumqttc"))
        .start();
    
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
        .subscribe(CONFIG_SCALE_VEHICLE, QoS::ExactlyOnce)
        .await.unwrap();
    client
        .subscribe(CONFIG_VEHICLE_STUPID, QoS::ExactlyOnce)
        .await.unwrap();
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
                    let _ = task::spawn(receive_offer(shared_vehicle.clone(), p.payload));
                }
                CHARGER_CHARGING_ACK => {
                    let _ = task::spawn(get_ack_handling(shared_vehicle.clone(), p.payload));
                }
                CONFIG_SCALE_VEHICLE => {
                    let _ = task::spawn(scale_handler(shared_vehicle.clone(), p.payload));
                }
                CONFIG_VEHICLE_STUPID => {
                    let _ = task::spawn(stupid_handler(shared_vehicle.clone(), p.payload));
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
