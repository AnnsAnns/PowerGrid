use std::{sync::Arc, time::Duration};
use battery::Battery;
use log::{debug, info};
use powercable::{TICK_TOPIC, WORLDMAP_EVENT_TOPIC};
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::{sync::Mutex, task};
use topic_handler::{tick_handler, worldmap_event_handler};
use vehicle::Vehicle;

mod vehicle;
mod battery;
mod topic_handler;

type SharedVehicle = Arc<Mutex<VehicleHandler>>;

struct VehicleHandler {
    pub name: String,
    pub vehicle: Vehicle,
    pub client: AsyncClient,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
    info!("Starting electric vehicle simulation...");

    // init battery
    let mut rng = rand::rng();
    let battery = Battery::new(
        rng.random_range(21.3..118.0),
        rng.random_range(0.5..1.0),
        25.0,
        rng.random_range(0.02..0.12),
        rng.random_range(7.0..350.0),
        rng.random_range(30.0..600.0),
        rng.random_range(0.90..0.98),
        rng.random_range(0.85..0.95),
    );

    // init vehicle
    let vehicle_name: String = powercable::generate_unique_name();
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
    let vehicle = Vehicle::new(
        vehicle_name.clone(),
        latitude,
        longitude,
        battery,
    );
    println!("{:#?}", vehicle);

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
    info!("Connected to MQTT broker");

    let shared_vehicle = Arc::new(Mutex::new(VehicleHandler {
        name: vehicle_name.clone(),
        vehicle,
        client: client.clone(),
    }));

    while let Ok(notification) = eventloop.poll().await {
        debug!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                TICK_TOPIC => {
                    let _ = task::spawn(tick_handler(shared_vehicle.clone(), p.payload));
                }
                WORLDMAP_EVENT_TOPIC => {
                    let _ = task::spawn(worldmap_event_handler(shared_vehicle.clone(), p.payload));
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
