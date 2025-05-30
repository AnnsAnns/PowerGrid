use bytes::Bytes;
use chrono::{NaiveDateTime, NaiveTime, Timelike};
use log::{debug, info, trace, warn};
use powercable::{
    offer::structure::OFFER_PACKAGE_SIZE,
    tickgen::{Phase, TickPayload},
    ChartEntry, Offer, ACK_ACCEPT_BUY_OFFER_TOPIC, BUY_OFFER_TOPIC, POWER_CONSUMER_TOPIC,
    POWER_LOCATION_TOPIC, POWER_TRANSFORMER_CONSUMPTION_TOPIC,
};
use rumqttc::QoS::*;
use serde_json::json;

use crate::SharedConsumer;

pub async fn tick_handler(handler: SharedConsumer, payload: Bytes) {
    let tick_payload: TickPayload = serde_json::from_slice(&payload).unwrap();
    match tick_payload.phase {
        Phase::Process => {
            process_tick(handler, payload).await;
        }
        Phase::Commerce => {
            commerce_tick(handler, payload).await;
        }
        Phase::PowerImport => {
            // No action needed
        }
    }
}

pub async fn process_tick(handler: SharedConsumer, payload: Bytes) {
    let packages_askable = {
        let mut handler = handler.lock().await;
        handler.consumer.tick();
        handler.offer_handler.remove_all_offers();
        handler.consumer.get_demand()
    };

    trace!("Demand (without scale): {:?}", packages_askable);

    //aus dem charger kopiert
    handler
        .lock()
        .await
        .consumer
        .set_current_consumption(packages_askable);
    debug!("Packages askable: {:?}", packages_askable);
    if packages_askable == 0 {
        info!("No packages available for sale");
        return;
    }

    // for every energy package, create an offer
    for i in 0..packages_askable {
        let mut handler = handler.lock().await;
        let offer_id = format!("{}-{}", handler.consumer.get_consumer_type().to_string(), i);
        // offer with max price
        let offer = Offer::new(
            offer_id,
            1.0,
            OFFER_PACKAGE_SIZE,
            handler.consumer.get_latitude(),
            handler.consumer.get_longitude(),
        );

        handler.offer_handler.add_offer(offer.clone()); //why?

        // publish offer
        handler
            .client
            .publish(BUY_OFFER_TOPIC, ExactlyOnce, false, offer.to_bytes())
            .await
            .unwrap()
    }
}

pub async fn commerce_tick(handler: SharedConsumer, payload: Bytes) {
    let handler = handler.lock().await;
    let tick_payload = TickPayload::from_bytes(payload).unwrap();
    // publish consumption to consumer for only consumer data
    handler
        .client
        .publish(
            POWER_TRANSFORMER_CONSUMPTION_TOPIC,
            ExactlyOnce,
            false,
            ChartEntry::new(
                handler.consumer.get_consumer_type().to_detailed_string(),
                handler.consumer.get_current_consumption() as isize,
                tick_payload.timestamp,
            )
            .to_string(),
        )
        .await
        .unwrap();
}

pub async fn accept_offer_handler(handler: SharedConsumer, payload: Bytes) {
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
        // send ACK to network
        handler
            .client
            .publish(
                ACK_ACCEPT_BUY_OFFER_TOPIC,
                ExactlyOnce,
                false,
                offer.to_bytes(),
            )
            .await
            .unwrap();
        trace!("ACK for offer {} sent", offer.get_id());
    }
}

pub async fn scale_handler(handler: SharedConsumer, payload: Bytes) {
    debug!("Received scale: {:?}", payload);
    let scale = serde_json::from_slice(&payload).unwrap();
    let mut handler = handler.lock().await;
    handler.consumer.set_current_scale(scale);
    trace!(
        "Consumer {} scale set to {}",
        handler.consumer.get_consumer_type().to_string(),
        scale
    );
}
