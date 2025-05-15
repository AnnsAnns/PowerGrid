use std::sync::Arc;

use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{offer::structure::OFFER_PACKAGE_SIZE, tickgen::{Phase, TickPayload}, ACCEPT_BUY_OFFER_TOPIC, POWER_NETWORK_TOPIC};
use rumqttc::QoS;
use tokio::sync::Mutex;

use crate::{init, TurbineHandler};

pub async fn process_tick(
    handler: Arc<Mutex<TurbineHandler>>,
) {
    let (client, power) = {
        let mut handler = handler.lock().await;
        handler.offer_handler.remove_all_offers();
        handler.turbine.tick();
        handler.turbine.approximate_wind_data().await;
        handler.turbine.approximate_temperature_data().await;
        handler.remaining_power = handler.turbine.get_power_output();
        debug!("Current power output: {} Watt", handler.remaining_power);

        (handler.client.clone(), handler.remaining_power)
    };

    let _ = client
    .publish(
        POWER_NETWORK_TOPIC,
        QoS::ExactlyOnce,
        false,
        power.to_string(),
    )
    .await;

    init::publish_location(handler.clone()).await;
}

pub async fn commerce_tick(
    handler: Arc<Mutex<TurbineHandler>>,
) {
    while (handler.lock().await.remaining_power > OFFER_PACKAGE_SIZE && handler.lock().await.offer_handler.has_offers()) {
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
            offer.to_bytes().unwrap(),
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
        Phase::Process => process_tick(handler.clone()).await,
        Phase::Commerce => commerce_tick(handler.clone()).await,
        Phase::PowerImport => {
            // No action needed
        }
    }
}