use bytes::Bytes;
use log::{debug, info};
use powercable::{
    generate_rnd_pos,
    tickgen::{Phase, TickPayload},
    POWER_LOCATION_TOPIC, VEHICLE_TOPIC,
};
use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;
use crate::{charger_handling::{accept_offer, create_charger_request}, vehicle::VehicleStatus, SharedVehicle};


#[derive(Serialize, Deserialize, Debug)]
/**
 * LocationPayload represents the payload for the worldmap event.
 */
pub struct LocationPayload {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub icon: String,
    pub label: String,
    pub action: String,
}

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

    // do these actions on all ticks (every 5 minutes)
    handler.lock().await.vehicle.drive();
    publish_vehicle(handler.clone()).await;
    publish_location(handler.clone()).await;
}

pub async fn worldmap_event_handler(handler: SharedVehicle, payload: Bytes) {
    let payload: LocationPayload = serde_json::from_slice(&payload).unwrap();

    if payload.icon == ":car:" {
        publish_vehicle(handler.clone()).await;
    }
}

pub async fn process_tick(handler: SharedVehicle) {
    let mut locked_handler = handler.lock().await;
  
    if locked_handler.target_charger.is_none() {
        if locked_handler.vehicle.battery().get_soc() <= 0.5 {
            info!(
                "{} has no charge left, searching for charging station",
                locked_handler.vehicle.get_name()
            );
            locked_handler.vehicle.set_status(VehicleStatus::SearchingForCharger);
            task::spawn(create_charger_request(handler.clone()));
        }

        if locked_handler.vehicle.get_location() == locked_handler.vehicle.get_destination() {
            locked_handler.vehicle.set_destination(generate_rnd_pos());
        }
    }

    if locked_handler.target_charger.is_some()
        && locked_handler.vehicle.get_location() == locked_handler.vehicle.get_destination()
        && locked_handler.vehicle.get_status().eq(&VehicleStatus::SearchingForCharger)
    {
        debug!("{} has arrived at the destination, requesting charge", locked_handler.vehicle.get_name());
        locked_handler.vehicle.set_status(VehicleStatus::Charging);
        // Create an arrival message to send to the charger
        task::spawn(create_arrival(handler.clone()));
    } else if locked_handler.vehicle.get_status().eq(&VehicleStatus::RANDOM) || locked_handler.vehicle.get_status().eq(&VehicleStatus::SearchingForCharger) {
        // locked_handler.vehicle.drive(50.0);
    }
}

pub async fn commerce_tick(handler: SharedVehicle) {
    {
        let l_handler = handler.lock().await;
        if l_handler.target_charger.is_some() || l_handler.charge_offers.is_empty() {
            return;
        }
    }
    debug!("{} has received charge offers, accepting the best one", handler.lock().await.vehicle.get_name());
    accept_offer(handler.clone()).await;
}

pub async fn publish_vehicle(handler: SharedVehicle) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.vehicle.get_name().clone();
    let mut vehicle_payload = json!(handler.vehicle);
    vehicle_payload["speed_kph"] = json!(handler.vehicle.get_speed_kph());
    vehicle_payload["soc"] = json!((handler.vehicle.battery().get_soc() * 100.0) as u32);

    let client = &mut handler.client;
    client
        .publish(
            format!("{}/{}", VEHICLE_TOPIC, name),
            QoS::ExactlyOnce,
            true,
            serde_json::to_string(&vehicle_payload).unwrap(),
        )
        .await
        .unwrap();
}

pub async fn publish_location(handler: SharedVehicle) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.vehicle.get_name().clone();
    let location = handler.vehicle.get_location();
    let destination = handler.vehicle.get_destination();
    let speed = handler.vehicle.get_speed_kph();
    let percentage = handler.vehicle.battery().get_soc() * 100.0;
    let client = &mut handler.client;
    let location_payload = json!({
        "name" : name,
        "lat": location.latitude,
        "lon": location.longitude,
        "line": [[location.latitude, location.longitude], [destination.latitude, destination.longitude]],
        "color": "grey",
        "icon": ":car:",
        "label": format!("{:.1}%", percentage),
    })
    .to_string();

    client
        .publish(
            POWER_LOCATION_TOPIC,
            QoS::ExactlyOnce,
            true,
            location_payload,
        )
        .await
        .unwrap();
}
