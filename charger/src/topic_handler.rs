use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{
    offer::structure::OFFER_PACKAGE_SIZE,
    tickgen::{Phase, TickPayload, INTERVAL_15_MINS},
    ChartEntry, Offer, ACK_ACCEPT_BUY_OFFER_TOPIC, BUY_OFFER_TOPIC, POWER_CHARGER_TOPIC,
    POWER_LOCATION_TOPIC, POWER_TRANSFORMER_CONSUMPTION_TOPIC,
};
use rumqttc::QoS;
use serde_json::json;

use crate::SharedCharger;

pub async fn tick_handler(handler: SharedCharger, payload: Bytes) {
    let payload: TickPayload = serde_json::from_slice(&payload).unwrap();
    match payload.phase {
        Phase::Process => {
            process_tick(handler, payload).await;
        }
        Phase::Commerce => {
            commerce_tick(handler).await;
        }
        Phase::PowerImport => {
            // No action needed
        }
    }
}

pub async fn process_tick(handler: SharedCharger, payload: TickPayload) {
    // Publish the amount of power we consumed in the last tick
    {
        let mut handler = handler.lock().await;
        let last_timestamp = payload.timestamp - INTERVAL_15_MINS;

        handler
            .client
            .publish(
                POWER_TRANSFORMER_CONSUMPTION_TOPIC,
                QoS::ExactlyOnce,
                false,
                ChartEntry::new(
                    handler.name.clone(),
                    handler.consumed_last_tick as isize,
                    last_timestamp,
                )
                .to_string(),
            )
            .await
            .unwrap();

        handler.consumed_last_tick = 0.0;
    }

    handler.lock().await.offer_handler.remove_all_offers();
    let mut packages_askable = handler.lock().await.charger.amount_of_needed_packages();
    if packages_askable == 0 {
        info!("No packages available for sale");
        return;
    }
    if packages_askable > 100 {
        debug!("Too many packages available for sale: {}", packages_askable);
        packages_askable = 100;
    }

    for i in 0..packages_askable {
        let mut handler = handler.lock().await;
        let offer_id = format!("{}-{}", handler.name, i);
        let offer = Offer::new(
            offer_id,
            handler
                .charger
                .get_price_if_had_charge(i * OFFER_PACKAGE_SIZE as usize),
            OFFER_PACKAGE_SIZE,
            handler.charger.get_latitude(),
            handler.charger.get_longitude(),
        );

        handler.offer_handler.add_offer(offer.clone());

        handler
            .client
            .publish(
                BUY_OFFER_TOPIC,
                rumqttc::QoS::ExactlyOnce,
                false,
                offer.to_bytes(),
            )
            .await
            .unwrap()
    }

    publish_location(handler.clone()).await;
}

pub async fn commerce_tick(handler: SharedCharger) {
    let handler = handler.lock().await;
    let current_power = handler.charger.get_current_charge();

    handler
        .client
        .publish(
            POWER_CHARGER_TOPIC,
            rumqttc::QoS::ExactlyOnce,
            false,
            current_power.to_string(),
        )
        .await
        .unwrap();
}

pub async fn accept_offer_handler(handler: SharedCharger, payload: Bytes) {
    let mut offer: Offer = Offer::from_bytes(payload).unwrap();
    if offer.get_accepted_by().is_none() {
        warn!(
            "Received ACK for offer {} without accepted_by field",
            offer.get_id()
        );
        return;
    }

    let mut handler = handler.lock().await;

    if !handler.offer_handler.has_sent_offer(&offer.get_id()) {
        offer.set_ack_for(offer.get_accepted_by().unwrap().clone());

        handler.offer_handler.add_sent_offer(offer.clone());
        handler
            .client
            .publish(
                ACK_ACCEPT_BUY_OFFER_TOPIC,
                rumqttc::QoS::ExactlyOnce,
                false,
                offer.to_bytes(),
            )
            .await
            .unwrap();
        debug!("ACK for offer {} sent", offer.get_id());

        handler.charger.add_charge(offer.get_amount() as usize);
        handler.consumed_last_tick += offer.get_amount();
    }
}

pub async fn publish_location(handler: SharedCharger) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.name.clone();
    let latitude = handler.charger.get_latitude();
    let longitude = handler.charger.get_longitude();
    let percentage = handler.charger.get_charge_percentage() * 100.0;
    let client = &mut handler.client;
    let location_payload = json!({
        "name" : name,
        "lat": latitude,
        "lon": longitude,
        "icon": ":battery:",
        "label": format!("{:.1}%", percentage),
    })
    .to_string();

    client
        .publish(
            POWER_LOCATION_TOPIC,
            QoS::ExactlyOnce,
            true,
            location_payload.clone(),
        )
        .await
        .unwrap();
    debug!("Location published: {}", location_payload);
}
