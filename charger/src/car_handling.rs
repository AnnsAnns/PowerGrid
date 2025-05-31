use log::{debug, info};
use powercable::{charger::{ChargeOffer, ChargeRequest}, CHARGER_OFFER};

use crate::SharedCharger;

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

    handler.charger.reserve_charge(reserved_charge);
    handler.charger.reserve_port();
    let price = handler.charger.get_current_price();
    handler.currently_reserved_for.push(payload.own_id.clone());

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

pub async fn check_accepted_offers(charger: SharedCharger, payload: ChargeOffer) {
    let mut handler = charger.lock().await;
    let target_id = payload.charge_target.clone();

    // This is not something we care about
    if !handler.currently_reserved_for.contains(&target_id) {
        return;
    }

    // Somebody else was accepted
    if &payload.own_id != &handler.name {
        debug!("We were not accepted by {}, removing from reserved list", payload.own_id);
        handler.currently_reserved_for.retain(|id| id != &target_id);
        handler.charger.release_reserved_charge(payload.charge_amount as usize);

        // @TODO: Continue here and fix the currently reserved list because it needs charge_amount etc in the list
    }
}