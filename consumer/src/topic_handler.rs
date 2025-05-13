use bytes::Bytes;
use chrono::{NaiveDateTime, NaiveTime, Timelike};
use log::{debug, info, warn};
use powercable::{offer::structure::OFFER_PACKAGE_SIZE, tickgen::{Phase, TickPayload}, Offer, ACK_ACCEPT_BUY_OFFER_TOPIC, BUY_OFFER_TOPIC, POWER_CONSUMER_TOPIC, POWER_LOCATION_TOPIC, POWER_NETWORK_TOPIC};
use rumqttc::QoS::*;
use serde::de;
use serde_json::json;

use crate::{SharedConsumer};

pub async fn tick_handler(
    handler: SharedConsumer,
    payload: Bytes
) {
    let tick_payload: TickPayload = serde_json::from_slice(&payload).unwrap();
    match tick_payload.phase {
        Phase::Process => {
            process_tick(handler, payload).await;
        }
        Phase::Commerce => {
            commerce_tick(handler).await;
        }
    }
}

pub async fn process_tick(
    handler: SharedConsumer,
    payload: Bytes
) {
    handler.lock().await.offer_handler.remove_all_offers();

    //Generate demand
    let tick_payload: TickPayload = serde_json::from_slice(&payload).unwrap();
    let timestamp = tick_payload.timestamp;
    debug!("Extracted timestamp: {}", timestamp);
    let trimmed = timestamp.strip_suffix(" UTC").unwrap();
    let dt = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S%.f").unwrap();
    let rounded_time = round_to_15min(dt);
    let demand = handler.lock().await.consumer.get_demand(rounded_time).await.unwrap_or(0.0);
    debug!("Demand (without scale): {:?}", demand);


    //aus dem charger kopiert
    let packages_askable = demand as u32 * handler.lock().await.consumer.get_current_scale() as u32;
    handler.lock().await.consumer.set_current_consumption(packages_askable as f32);
    debug!("Packages askable: {:?}", packages_askable);
    if packages_askable == 0 {
        info!("No packages available for sale");
        return;
    }

    // for every energy package, create an offer
    for i in 0..packages_askable {
        let mut handler = handler.lock().await;
        let offer_id = format!("{}-{}", handler.name, i);
        // offer with max price
        let offer = Offer::new(offer_id, f64::MAX, OFFER_PACKAGE_SIZE);

        handler.offer_handler.add_offer(offer.clone()); //why?

        // publish offer
        handler.client.publish(
            BUY_OFFER_TOPIC,
            ExactlyOnce,
            false,
            offer.to_bytes().unwrap(),
        ).await.unwrap()
    }
}



pub async fn commerce_tick(
    handler: SharedConsumer,
) {
    let handler = handler.lock().await;
    // Publish location
    let location_payload = json!({
        "name" : handler.consumer.get_name(),
        "lat": handler.consumer.get_latitude(),
        "lon": handler.consumer.get_longitude(),
        "icon": handler.consumer.get_consumer_type().to_icon(),
        "label": format!("{:.1}kW", handler.consumer.get_current_consumption()),
    })
    .to_string();
    handler.client
        .publish(
            POWER_LOCATION_TOPIC,
            ExactlyOnce,
            true,
            location_payload.clone(),
        )
        .await
        .unwrap();
    debug!("Published location: {:?}", location_payload);
}

pub async fn accept_offer_handler(
    handler: SharedConsumer,
    payload: Bytes
) {
    let mut offer: Offer = serde_json::from_slice(&payload).unwrap();
    if offer.get_accepted_by().is_none() {
        warn!("Received ACK for offer {} without accepted_by field", offer.get_id());
        return;
    }

    let mut handler = handler.lock().await;
    
    if !handler.offer_handler.has_sent_offer(&offer.get_id()) {
        offer.set_ack_for(offer.get_accepted_by().unwrap().clone());

        handler.offer_handler.add_sent_offer(offer.clone());
        // send ACK to network
        handler.client.publish(
            ACK_ACCEPT_BUY_OFFER_TOPIC,
            ExactlyOnce,
            false,
            offer.to_bytes().unwrap(),
        ).await.unwrap();
        debug!("ACK for offer {} sent", offer.get_id());

        // publish consumption to network
        handler.client.publish(
            POWER_NETWORK_TOPIC,
            ExactlyOnce,
            false,
            (-1 * offer.get_amount() as i32).to_string()
        ).await.unwrap();
    }
}

pub async fn scale_handler(
    handler: SharedConsumer,
    payload: Bytes
) {
    debug!("Received scale: {:?}", payload);
    let scale: u8 = serde_json::from_slice(&payload).unwrap();
    let mut handler = handler.lock().await;
    handler.consumer.set_current_scale(scale);
    debug!("Consumer {} scale set to {}", handler.name, scale);
}

fn round_to_15min(dt: NaiveDateTime) -> NaiveTime {
    let minutes = dt.minute();
    let rounded = minutes - (minutes % 15);
    return NaiveTime::from_hms_opt(dt.hour(), rounded, 0).unwrap();
}
