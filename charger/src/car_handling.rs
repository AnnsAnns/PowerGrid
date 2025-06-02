use log::{debug, info};
use powercable::{charger::{ChargeOffer, ChargeRequest}, CHARGER_OFFER};

use crate::{offer_handling::ReservedOffer, SharedCharger};

async fn receive_request(charger: SharedCharger, payload: ChargeRequest) {
    let mut handler = charger.lock().await;
    
    if handler.charger.get_free_ports() == 0 {
        info!("Charger {} has no free ports, rejecting request from {}", handler.name, payload.own_id);
        return;
    }

    let charge_amount = payload.charge_amount as usize;
    let reserable_charge = handler.charger.get_available_charge();

    let reserved_charge = if charge_amount > reserable_charge {
        reserable_charge
    } else {
        charge_amount
    };


    let price = handler.charger.get_current_price();

    let offer = ReservedOffer::new(
        payload.own_id.clone(),
     reserved_charge, price);

    handler.reserve_offer(offer);

    let offer = ChargeOffer::new(
        handler.name.clone(),
        payload.own_id,
        price,
        reserved_charge as f64,
        handler.charger.get_latitude(),
        handler.charger.get_longitude(),
    );

    handler.client.publish(
        CHARGER_OFFER,
        rumqttc::QoS::AtMostOnce,
        false,
        offer.to_bytes(),
    ).await.unwrap();
}

/// This function handles incoming charge requests from cars
/// 
/// # Arguments
/// * `charger` - A shared reference to the charger handler
/// * `payload` - The charge request payload containing the car's ID and requested charge amount
pub async fn check_accepted_offers(charger: SharedCharger, payload: ChargeOffer) {
    let mut handler = charger.lock().await;
    let target_id = payload.charge_target.clone();

    // This is not something we care about
    if handler.get_reserved_offer(target_id.clone()).is_none() {
        debug!("Received offer for {} but we are not interested in it", target_id);
        return;
    }

    // Somebody else was accepted
    if &payload.own_id != &handler.name {
        debug!("We were not accepted by {}, removing from reserved list", payload.charge_target);
        handler.release_offer(target_id.clone());
    } else {
        debug!("We were accepted by {}", payload.charge_target);
        handler.accept_reserve(target_id.clone());
    }
}