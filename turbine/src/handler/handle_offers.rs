use std::sync::Arc;

use bytes::Bytes;
use log::info;
use powercable::{
    tickgen::{Phase, TickPayload},
    Offer, POWER_NETWORK_TOPIC,
};
use rumqttc::QoS;
use tokio::sync::Mutex;

use crate::TurbineHandler;

pub async fn handle_buy_offer(handler: Arc<Mutex<TurbineHandler>>, payload: Bytes) {
    let offer: Offer = serde_json::from_slice(&payload).unwrap();
    info!("Received buy offer: {:?}", offer);
    {
        let mut handler = handler.lock().await;
        handler.offer_handler.add_offer(offer.clone());
    }
}

pub async fn accept_buy_offer(handler: Arc<Mutex<TurbineHandler>>, topic: String) {
    let id: String = powercable::get_id_from_topic(&topic);
    let handler = handler.lock().await;
    let offer = handler.offer_handler.get_offer(id.as_str());
    if offer.is_some() {
        let offer = offer.unwrap();
        info!("Accepted buy offer: {:?}", offer);
        handler
            .client
            .publish(
                powercable::ACK_ACCEPT_BUY_OFFER_TOPIC,
                QoS::ExactlyOnce,
                false,
                id,
            )
            .await
            .unwrap();
    } else {
        info!("No offer found for ID: {}", id);
    }
}

pub async fn ack_buy_offer(handler: Arc<Mutex<TurbineHandler>>, topic: String) {
    let offer_id = powercable::get_id_from_topic(&topic);
    let mut handler = handler.lock().await;
    info!("Received ACK for offer: {}", offer_id);
    if let Some(offer) = handler.offer_handler.get_offer(offer_id.as_str()) {
        info!("Offer accepted: {:?}", offer);
        handler.offer_handler.remove_offer(offer_id.as_str());
    }
}
