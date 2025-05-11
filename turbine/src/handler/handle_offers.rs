use std::sync::Arc;

use bytes::Bytes;
use log::{info, warn};
use powercable::{
    tickgen::{Phase, TickPayload},
    Offer, POWER_NETWORK_TOPIC,
};
use rumqttc::QoS;
use tokio::sync::Mutex;

use crate::{SharedTurbine, TurbineHandler};

pub async fn handle_buy_offer(handler: SharedTurbine, payload: Bytes) {
    let offer: Offer = serde_json::from_slice(&payload).unwrap();
    info!("Received buy offer: {:?}", offer);
    {
        let mut handler = handler.lock().await;
        handler.offer_handler.add_offer(offer.clone());
    }
}

pub async fn ack_buy_offer(handler: SharedTurbine, payload: Bytes) {
    let offer: Offer = serde_json::from_slice(&payload).unwrap();

    if offer.get_ack_for().is_none() {
        warn!("Received ACK for offer {} without ack_for field", offer.get_id());
        return;
    }
    
    if handler.lock().await.offer_handler.has_sent_offer(&offer.get_id()) {
        if offer.get_ack_for().unwrap() != handler.lock().await.name.as_str() {
            info!("Received ACK for offer {} from {} - We didn't get it, freeing reserved energy again 😔", offer.get_id(), offer.get_ack_for().unwrap());
            handler.lock().await.remaining_power += offer.get_amount();
        } else {
            info!("Received ACK for own offer {} from {} - We did it 😄", offer.get_id(), offer.get_ack_for().unwrap());
        }
    }
}
