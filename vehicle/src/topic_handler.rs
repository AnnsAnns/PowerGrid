use bytes::Bytes;
use log::info;
use powercable::{
    tickgen::{Phase, TickPayload},
    POWER_LOCATION_TOPIC,
};
use rumqttc::QoS;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;

use crate::{charger_handling::{accept_offer, create_charger_request}, SharedVehicle};

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn worldmap_event_handler(handler: SharedVehicle, payload: Bytes) {
    let payload: LocationPayload = serde_json::from_slice(&payload).unwrap();

    if payload.icon == ":car:" {
        publish_soc(handler.clone()).await;
    }
}

pub async fn process_tick(handler: SharedVehicle) {
    {
        let mut locked_handler = handler.lock().await;

        if locked_handler.target_charger.is_none() {
            if locked_handler.vehicle.battery().state_of_charge() <= 0.3 {
                info!(
                    "{} has no charge left, searching for charging station",
                    locked_handler.vehicle.get_name()
                );
                task::spawn(create_charger_request(handler.clone()));
            }

            if locked_handler.vehicle.get_location() == locked_handler.vehicle.get_destination() {
                let (latitude, longitude) =
                    powercable::generate_latitude_longitude_within_germany();
                locked_handler.vehicle.set_destination(latitude, longitude);
            }
        }

        if locked_handler.target_charger.is_some()
            && locked_handler.vehicle.get_location() == locked_handler.vehicle.get_destination()
        {
            info!(
                "{} has arrived at the destination, requesting charge",
                locked_handler.vehicle.get_name()
            );

            //@todo: charge
        } else {
            locked_handler.vehicle.drive(50.0);
        }
    }
    publish_vehicle(handler.clone()).await;
    publish_soc(handler.clone()).await;// TODO: whyyy?
}

pub async fn commerce_tick(handler: SharedVehicle) {
    let l_handler = handler.lock().await;
    if l_handler.target_charger.is_none() && !l_handler.charge_offers.is_empty() {
        info!(
            "{} has received charge offers, accepting the best one",
            l_handler.vehicle.get_name()
        );
        accept_offer(handler.clone()).await;
    }
}

pub async fn publish_vehicle(handler: SharedVehicle) {
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

pub async fn publish_soc(handler: SharedVehicle) {
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