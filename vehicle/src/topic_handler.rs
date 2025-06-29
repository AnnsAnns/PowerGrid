use bytes::Bytes;
use rand::Rng;
use tracing::{debug, info, trace, warn};
use powercable::{generate_rnd_pos, tickgen::{Phase, TickPayload}, POWER_LOCATION_TOPIC, VEHICLE_TOPIC};
use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;
use crate::{charger_handling::{accept_offer, create_charger_request, create_get}, vehicle::{VehicleAlgorithm, VehicleStatus}, SharedVehicle};

const FIND_CHARGER_AT_LEAST: f64 = 0.3; // 30% charge left

/// # Description
/// The `LocationPayload` struct represents the payload for location updates in the world map.<br>
/// 
/// # Fields
/// - `name`: The name of the vehicle.
/// - `lat`: The latitude of the vehicle's current location.
/// - `lon`: The longitude of the vehicle's current location.
/// - `icon`: The icon representing the vehicle on the world map.
/// - `label`: The label to be displayed on the world map, typically showing the vehicle's state of charge (SoC).
/// - `action`: The action to be performed when the vehicle is clicked on the world map.
#[derive(Serialize, Deserialize, Debug)]
pub struct LocationPayload {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub icon: String,
    pub label: String,
    pub action: String,
}

/// # Description
/// Called when a tick is received on the `TICK_TOPIC` topic.<br>
/// It processes the tick based on the current phase and delegates the handling to the appropriate function.<br>
/// For the `Process` phase, it calls `process_tick`, and for the `Commerce` phase, it calls `commerce_tick`.<br>
/// For the `PowerImport` phase, no action is needed.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance and the MQTT client.
/// - `payload`: The incoming payload containing the tick information in JSON format.
pub async fn tick_handler(handler: SharedVehicle, payload: Bytes) {
    let payload: TickPayload = serde_json::from_slice(&payload).unwrap();
    match payload.phase {
        Phase::Process => {
            process_tick(handler.clone()).await;
        }
        Phase::Commerce => {
            commerce_tick(handler.clone()).await;
        }
        Phase::PowerImport => {
            // No action needed
        }
    }

    // drive on all ticks (every 5 minutes)
    handler.lock().await.vehicle.drive();
    publish_vehicle(handler.clone()).await;
    publish_location(handler.clone()).await;
}

/// # Description
/// The `process_tick` function is called during the process phase of the tick.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance and the MQTT client.
pub async fn process_tick(handler: SharedVehicle) { // TODO: rework this function cause its chaos
    let mut locked_handler = handler.lock().await;
  
    if locked_handler.target_charger.is_none() {
        if locked_handler.vehicle.battery().get_soc() <= FIND_CHARGER_AT_LEAST { // If the vehicle low on battery, search for a charger
            info!("{} has no charge left, searching for charging station", locked_handler.vehicle.get_name());
            locked_handler.vehicle.set_status(VehicleStatus::SearchingForCharger);
            task::spawn(create_charger_request(handler.clone()));
        }

        if locked_handler.vehicle.get_location() == locked_handler.vehicle.get_destination() {
            let mut rng = rand::rng();
            match rng.random_range(0..11) { // Average parking time is 3 hours (made up)
                0 => {
                    locked_handler.vehicle.set_status(VehicleStatus::Random); // Generate new destination
                    locked_handler.vehicle.set_destination(generate_rnd_pos());
                },
                _ => locked_handler.vehicle.set_status(VehicleStatus::Parked), // Usually the car is parked after reaching its destination
            }
        }
    } else { // Driving to a charger
        if locked_handler.vehicle.get_location() == locked_handler.vehicle.get_next_stop() {
            if locked_handler.vehicle.get_status().eq(&VehicleStatus::SearchingForCharger) {
                info!("{} has arrived at the destination, requesting charge", locked_handler.vehicle.get_name());
                locked_handler.vehicle.set_status(VehicleStatus::Charging);
            }
            if locked_handler.vehicle.get_status().eq(&VehicleStatus::Charging) {
                task::spawn(create_get(handler.clone()));
            }
        }
    }
}

/// # Description
/// The `commerce_tick` function is called during the commerce phase of the tick.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance and the MQTT client.
pub async fn commerce_tick(handler: SharedVehicle) {
    {
        let l_handler = handler.lock().await;
        if l_handler.target_charger.is_some() || l_handler.charge_offers.is_empty() {
            return;
        }
    }
    trace!("{} has received charge offers, accepting the best one", handler.lock().await.vehicle.get_name());
    accept_offer(handler.clone()).await;
}

/// # Description
/// Publishes on the `VEHICLE_TOPIC/vehicle_name` the current `vehicle_speed` and battery `soc` as a percentage to the MQTT broker.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance and the MQTT client.
pub async fn publish_vehicle(handler: SharedVehicle) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.vehicle.get_name().clone();
    let mut vehicle_payload = json!(handler.vehicle);
    vehicle_payload["speed_kph"] = json!(handler.vehicle.get_speed());
    vehicle_payload["soc"] = json!((handler.vehicle.battery().get_soc_percentage()) as u32);

    let client = &mut handler.client;
    client.publish(
        format!("{}/{}", VEHICLE_TOPIC, name),
        QoS::ExactlyOnce,
        true,
        serde_json::to_string(&vehicle_payload).unwrap(),
    ).await.unwrap();
}

/// # Description
/// Publishes on the `POWER_LOCATION_TOPIC` the current location and state of the vehicle to the MQTT broker.<br>
/// Its used to display the vehicle on the world map in the frontend.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance and the MQTT client.
pub async fn publish_location(handler: SharedVehicle) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.vehicle.get_name().clone();
    let location = handler.vehicle.get_location();
    let next_stop = handler.vehicle.get_next_stop();
    let destination = handler.vehicle.get_destination();
    let percentage = handler.vehicle.battery().get_soc_percentage();// TODO: why still warning about speed?
    let client = &mut handler.client;
    let destination_payload = json!({
        "name" : format!("{}-destination", name),
        "lat": destination.latitude,
        "lon": destination.longitude,
        "line": [[location.latitude, location.longitude], [destination.latitude, destination.longitude]],
        "color": "grey",
        "dashArray": "8,8",
        "icon": ":triangular_flag_on_post:",
    })
    .to_string();
    let location_payload = json!({
        "name" : name,
        "lat": location.latitude,
        "lon": location.longitude,
        "line": [[location.latitude, location.longitude], [next_stop.latitude, next_stop.longitude]],
        "color": "#B07070",
        "icon": ":car:",
        "label": format!("{:.1}%", percentage),
    }).to_string();

    client.publish(
        POWER_LOCATION_TOPIC,
        QoS::ExactlyOnce,
        true,
        destination_payload,
    ).await.unwrap();
    client.publish(
        POWER_LOCATION_TOPIC,
        QoS::ExactlyOnce,
        true,
        location_payload,
    ).await.unwrap();
}

/// # Description
/// If the vehicle is clicked on the world map, this function is called.<br>
/// It publishes the vehicle's information to the MQTT broker to update the world map.<br>
/// It is called when a message is received on the `WORLDMAP_EVENT_TOPIC` topic.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance and the MQTT client.
/// - `payload`: The incoming payload containing the event information in JSON format.
pub async fn worldmap_event_handler(handler: SharedVehicle, payload: Bytes) {
    let payload: LocationPayload = serde_json::from_slice(&payload).unwrap();

    if payload.icon == ":car:" {
        publish_vehicle(handler.clone()).await;
    }
}

/// # Description
/// The `scale_handler` function processes incoming scale configuration messages for the vehicle.<br>
/// It updates the vehicle's consumption scale based on the received payload.<br>
/// It is called when a message is received on the `CONFIG_VEHICLE_SCALE` topic.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance.
/// - `payload`: The incoming payload containing the scale configuration in JSON format.
pub async fn scale_handler(handler: SharedVehicle, payload: Bytes) {
    let mut handler = handler.lock().await;
    trace!("Received scale: {:?}", payload);
    let scale = serde_json::from_slice(&payload).unwrap();
    handler.vehicle.set_scale(scale);
    debug!("Consumption Scale set to: {}", scale);
}

/// # Description
/// The `algorithm_handler` function processes incoming algorithm configuration messages for the vehicle.<br>
/// It updates the vehicle's algorithm based on the received payload.<br>
/// It is called when a message is received on the `CONFIG_VEHICLE_ALGORITHM` topic.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the vehicle handler, which contains the vehicle instance.
/// - `payload`: The incoming payload containing the algorithm configuration in JSON format.
pub async fn algorithm_handler(handler: SharedVehicle, payload: Bytes) {
    let mut handler = handler.lock().await;
    trace!("Received algorithm: {:?}", payload);
    let algo_num = serde_json::from_slice(&payload).unwrap();
    let algorithm = match algo_num {// TODO: isn´t enum equal to int --> optimize
        0 => VehicleAlgorithm::Best,
        1 => VehicleAlgorithm::Random,
        2 => VehicleAlgorithm::Closest,
        3 => VehicleAlgorithm::Cheapest,
        _ => {
            warn!("Unknown algorithm number: {}, defaulting to Best", algo_num);
            VehicleAlgorithm::Best
        }
    };
    handler.vehicle.set_algorithm(algorithm);
    debug!("Algorithm set to: {:?}", algorithm);
}