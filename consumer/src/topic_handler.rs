use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{offer::structure::OFFER_PACKAGE_SIZE, tickgen::{Phase, TickPayload}, Offer, ACK_ACCEPT_BUY_OFFER_TOPIC, BUY_OFFER_TOPIC, POWER_CHARGER_TOPIC, POWER_NETWORK_TOPIC, POWER_LOCATION_TOPIC};
use rumqttc::QoS;
use serde_json::json;

use crate::{SharedConsumer};

pub async fn tick_handler(
    handler: SharedConsumer,
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
    }
}

pub async fn process_tick(
    handler: SharedConsumer,
) {
    handler.lock().await.offer_handler.remove_all_offers();
    let mut packages_askable = handler.lock().await.consumer.amount_of_needed_packages();
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
        let offer = Offer::new(offer_id, handler.consumer.get_price_if_had_charge(i * OFFER_PACKAGE_SIZE as usize), OFFER_PACKAGE_SIZE);

        handler.offer_handler.add_offer(offer.clone());

        handler.client.publish(
            BUY_OFFER_TOPIC,
            rumqttc::QoS::ExactlyOnce,
            false,
            offer.to_bytes().unwrap(),
        ).await.unwrap()
    }
    
    publish_location(handler.clone()).await;
}

pub async fn commerce_tick(
    handler: SharedConsumer,
) {
    // do nothing
}

pub async fn accept_offer_handler(
    handler: SharedConsumer,
    payload: Bytes
) {

}

pub async fn publish_location(
    handler: SharedConsumer,
) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.name.clone();
    let latitude = handler.consumer.get_latitude();
    let longitude = handler.consumer.get_longitude();
    let consumer_type = handler.consumer.get_consumer_type();
    let client = &mut handler.client;
    let location_payload = json!({
        "name" : name,
        "lat": latitude,
        "lon": longitude,
        "icon": format!("{:?}", consumer_type),
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
    debug!("Published location: {:?}", location_payload);
}