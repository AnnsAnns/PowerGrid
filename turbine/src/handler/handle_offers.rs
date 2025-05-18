use bytes::Bytes;
use log::{debug, warn};
use powercable::Offer;

use crate::SharedTurbine;

pub async fn handle_buy_offer(handler: SharedTurbine, payload: Bytes) {
    let offer: Offer = Offer::from_bytes(payload).unwrap();
    debug!("Received buy offer: {:?}", offer);
    {
        let mut handler = handler.lock().await;
        handler.offer_handler.add_offer(offer.clone());
    }
}

pub async fn ack_buy_offer(handler: SharedTurbine, payload: Bytes) {
    let offer: Offer = Offer::from_bytes(payload).unwrap();

    if offer.get_ack_for().is_none() {
        warn!("Received ACK for offer {} without ack_for field", offer.get_id());
        return;
    }
    
    if handler.lock().await.offer_handler.has_sent_offer(&offer.get_id()) {
        if offer.get_ack_for().unwrap() != handler.lock().await.name.as_str() {
            debug!("Received ACK for offer {} from {} - We didn't get it, freeing reserved energy again 😔", offer.get_id(), offer.get_ack_for().unwrap());
            handler.lock().await.remaining_power += offer.get_amount();
        } else {
            debug!("Received ACK for own offer {} from {} - We did it 😄", offer.get_id(), offer.get_ack_for().unwrap());
        }
    }
}
