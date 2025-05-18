use bytes::Bytes;
use powercable::{tickgen::{Phase, TickPayload}, POWER_LOCATION_TOPIC};
use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::SharedVehicle;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationPayload {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub icon: String,
    pub label: String,
    pub action: String,
}

pub async fn tick_handler(
    handler: SharedVehicle,
    payload: Bytes
) {
    let payload: TickPayload = serde_json::from_slice(&payload).unwrap();
    match payload.phase {
        Phase::Process => {
            process_tick(handler).await;
        }
        Phase::Commerce => {
            commerce_tick(handler).await;
        }
        Phase::PowerImport => {
            // No action needed
        }
    }
}

pub async fn worldmap_event_handler(
    handler: SharedVehicle,
    payload: Bytes
) {
    let payload: LocationPayload = serde_json::from_slice(&payload).unwrap();

    if payload.icon == ":car:" {
        publish_soc(handler.clone()).await;
    }
}

pub async fn process_tick(
    handler: SharedVehicle,
) {
    {
        let vehicle = &mut handler.lock().await.vehicle;

        if vehicle.get_location() == vehicle.get_destination() {
            let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();
            vehicle.set_destination(latitude, longitude);
        }
        vehicle.drive();
    }
    publish_location(handler.clone()).await;
    publish_soc(handler.clone()).await;
}

pub async fn commerce_tick(
    handler: SharedVehicle,
) {
    
}

pub async fn publish_location(
    handler: SharedVehicle,
) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.name.clone();
    let (latitude, longitude) = handler.vehicle.get_location();
    let percentage = handler.vehicle.battery().state_of_charge() * 100.0;
    let client = &mut handler.client;
    let location_payload = json!({
        "name" : name,
        "lat": latitude,
        "lon": longitude,
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

pub async fn publish_soc(
    handler: SharedVehicle,
) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.name.clone();
    let soc = handler.vehicle.battery().state_of_charge() * 100.0;
    let client = &mut handler.client;

    client
        .publish(
            format!("vehicle/{}/battery/soc", name),
            QoS::ExactlyOnce,
            true,
            format!("{}", soc),
        )
        .await
        .unwrap();
}