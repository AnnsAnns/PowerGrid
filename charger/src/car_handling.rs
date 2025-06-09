use bytes::Bytes;
use log::{debug, info};
use powercable::{charger::{ChargeOffer, ChargeRequest}, CHARGER_OFFER};

use crate::{offer_handling::ReservedOffer, SharedCharger};

pub async fn receive_request(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    let request: ChargeRequest = ChargeRequest::from_bytes(payload).unwrap();
    info!("Received charge request from {}: {} kWh at ({}, {})", 
           request.vehicle_name, request.charge_amount, request.position.latitude, request.position.longitude);
    
    if handler.charger.get_free_ports() == 0 {
        info!("Charger {} has no free ports, rejecting request from {}", handler.name, request.vehicle_name);
        return;
    }

    let charge_amount = request.charge_amount as usize;
    let reserable_charge = handler.charger.get_available_charge();

    let reserved_charge = if charge_amount > reserable_charge {
        reserable_charge
    } else {
        charge_amount
    };


    let price = handler.charger.get_current_price();

    let offer = ReservedOffer::new(
        request.vehicle_name.clone(),
     reserved_charge, price);

    handler.reserve_offer(offer);

    let offer = ChargeOffer::new(
        handler.name.clone(),
        request.vehicle_name,
        price,
        reserved_charge as f64,
        handler.charger.get_position().clone(),
    );

    info!("Sending charge offer to {}: {} kWh at {}€", offer.vehicle_name, offer.charge_amount, offer.charge_price);
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
    let target_id = payload.vehicle_name.clone();

    // This is not something we care about
    if handler.get_reserved_offer(target_id.clone()).is_none() {
        debug!("Received offer for {} but we are not interested in it", target_id);
        return;
    }

    // Somebody else was accepted
    if &payload.charger_name != &handler.name {
        debug!("We were not accepted by {}, removing from reserved list", payload.vehicle_name);
        handler.release_offer(target_id.clone());
    } else {
        debug!("We were accepted by {}", payload.vehicle_name);
        handler.accept_reserve(target_id.clone());
    }
}