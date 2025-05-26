use std::sync::Arc;
use bytes::Bytes;
use log::{debug, warn};
use powercable::{offer::structure::OFFER_PACKAGE_SIZE, tickgen::{Phase, TickPayload}, ChartEntry, ACCEPT_BUY_OFFER_TOPIC, POWER_NETWORK_TOPIC, POWER_TRANSFORMER_GENERATION_TOPIC};
use rumqttc::QoS;
use tokio::sync::Mutex;

use crate::{init, TurbineHandler};

pub async fn process_tick(
    handler: Arc<Mutex<TurbineHandler>>,
    payload: TickPayload,
) {

    let (client, power, name) = {
        let mut handler = handler.lock().await;
        handler.turbine.tick();
        handler.offer_handler.remove_all_offers();
        handler.remaining_power = handler.turbine.get_power_output();
        debug!("Current power output: {} Watt", handler.remaining_power);

        (handler.client.clone(), handler.remaining_power, handler.name.clone())
    };

    let _ = client
    .publish(
        POWER_TRANSFORMER_GENERATION_TOPIC,
        QoS::ExactlyOnce,
        false,
        ChartEntry::new(
            name.clone(),
            power as isize,
            payload.timestamp,
        ).to_string()
    )
    .await;

    init::publish_location(handler.clone()).await;
}

pub async fn commerce_tick(
    handler: Arc<Mutex<TurbineHandler>>,
) {
    while handler.lock().await.remaining_power > OFFER_PACKAGE_SIZE && handler.lock().await.offer_handler.has_offers() {
        let mut handler = handler.lock().await;
        let mut offer = match handler.offer_handler.get_best_non_sent_offer() {
            Some(offer) => offer.clone(),
            None => {
                debug!("No offers available, remaining power: {}", handler.remaining_power);
                break;
            }
        };

        if offer.get_amount() != OFFER_PACKAGE_SIZE {
            warn!("Offer power size is not equal to OFFER_PACKAGE_SIZE: {:?}", offer);
        }

        handler.remaining_power -= offer.get_amount();
        offer.set_accepted_by(handler.name.clone());
        handler.offer_handler.add_sent_offer(offer.clone());

        handler.client.publish(
            ACCEPT_BUY_OFFER_TOPIC,
            QoS::ExactlyOnce,
            false,
            offer.to_bytes(),
        ).await.unwrap();
    }
}

pub async fn handle_tick(
    handler: Arc<Mutex<TurbineHandler>>,
    payload: Bytes,
) {
    let payload =
    serde_json::from_slice::<powercable::tickgen::TickPayload>(
        &payload,
    )
    .unwrap();

    match payload.phase {
        Phase::Process => process_tick(handler.clone(), payload).await,
        Phase::Commerce => commerce_tick(handler.clone()).await,
        Phase::PowerImport => {
            // No action needed
        }
    }
}