use bytes::Bytes;
use tracing::{debug, info, trace};
use powercable::{charger::{Get, ChargeAccept, ChargeOffer, ChargeRequest}, CHARGER_OFFER, CHARGER_CHARGING_ACK};
use rumqttc::QoS;

use crate::{offer_handling::ReservedOffer, SharedCharger};

/// # Description
/// Every charge request is answered with a charge offer, if the charger has at least one free port.<br>
/// The charger reserves the requested charge amount and sends a `ChargeOffer` to the vehicle.
/// 
/// # Arguments
/// - `charger`: The shared charger handler containing the charger and its state.
/// - `payload`: The payload containing the charge request data.
pub async fn receive_request(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    let charge_request= ChargeRequest::from_bytes(payload).unwrap();
    debug!("Charge request: {:?}", charge_request);

    if handler.charger.get_free_ports() == 0 {
        info!("Charger {} has no free ports, rejecting request from {}", handler.charger.get_name(), charge_request.vehicle_name);
        return;
    }

    let energy_for_way = (charge_request.vehicle_position.distance_to(handler.charger.get_position()) * (charge_request.vehicle_consumption/ 100.0)) as usize;// km * kWh/km = kWh
    let charge_amount = charge_request.charge_amount + energy_for_way;// including the energy for the way
    debug!("Vehicle {} wants {} kWh, but needs {} kWh for the way, so we need to reserve {} kWh", 
        charge_request.vehicle_name, charge_request.charge_amount, energy_for_way, charge_amount);
    let reservable_charge = charge_amount.min(handler.charger.get_available_charge());// cant reserve more than the charger has
    debug!("Vehicle {} requests {} kWh and we can reserve {} kWh", charge_request.vehicle_name, charge_amount, reservable_charge);

    
    // ReserveOffer for own system
    let offer = ReservedOffer::new(
        charge_request.vehicle_name.clone(),
        reservable_charge, handler.charger.get_current_price());
    debug!("Creating reserved offer: {:?}", offer);
    handler.reserve_offer(offer);

    // ChargeOffer for the vehicle
    let offer = ChargeOffer::new(
        handler.charger.get_name().clone(),
        charge_request.vehicle_name.clone(),// TODO: why no gray name, like other fields have?
        handler.charger.get_current_price(),
        reservable_charge,
        handler.charger.get_position(),
    );
    debug!("Creating charge offer: {:?}", offer);

    // Publish the offer to the vehicle
    handler.client.publish(
        CHARGER_OFFER,
        QoS::ExactlyOnce,
        false,
        offer.to_bytes(),
    ).await.unwrap();
    trace!("Published on topic {}: {:?}", CHARGER_OFFER, offer);
}


/// # Description
/// Handles a charge accept message from a vehicle.<br>
/// This function first checks if the vehicle is in the reserved offers list.
/// If it is, it checks if the charger name matches the one in the accept message.
/// If it does, it accepts the reservation; otherwise, it releases the reservation offer.
/// 
/// # Arguments
/// - `charger`: The shared charger handler containing the charger and its state.
/// - `payload`: The payload containing the charge accept data.
pub async fn accept_handler(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    // Deserialize the payload into a ChargeAccept object
    let charge_accept: ChargeAccept = ChargeAccept::from_bytes(payload).unwrap();
    debug!("Charge accept: {:?}", charge_accept);

    // This is not something we care about
    if handler.get_reserved_offer(charge_accept.vehicle_name.clone()).is_none() {
        debug!("Received accept from {} but we didn`t reserve an offer", charge_accept.vehicle_name);
    } else if &charge_accept.charger_name != handler.charger.get_name() {
        info!("We were not accepted by {}, removing from reserved list", charge_accept.vehicle_name);
        handler.release_offer(charge_accept.vehicle_name.clone(), true);
    } else {
        info!("We were accepted by {}", charge_accept.vehicle_name);
        handler.accept_reserve(charge_accept.vehicle_name.clone());
    }
}

/// # Description
/// Handles a get request from a vehicle.<br>
/// 
pub async fn answer_get(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    // Deserialize the payload into a Get object
    let mut get = Get::from_bytes(payload).unwrap();
    debug!("Get request: {:?}", get);
    
    if get.charger_name.eq(handler.charger.get_name()) {
        info!("Received get request from {}", get.vehicle_name);

        let amount_we_can_give = handler.charger.take_reserved_charge(get.amount);

        get.amount = amount_we_can_give;

        info!("Sending ack: {:?}", get);
        handler.client.publish(
            CHARGER_CHARGING_ACK,
            QoS::ExactlyOnce,
            false,
            get.to_bytes(),
        ).await.unwrap();
        trace!("Published on topic {}: {:?}", CHARGER_CHARGING_ACK, get);
    }
}

pub async fn release_car(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    // Deserialize the payload into a Get object
    trace!("Received release payload: {:?}", payload);
    let get: Get = Get::from_bytes(payload).unwrap();

    if get.charger_name.eq(handler.charger.get_name()) {
        info!("Received release request from {}", get.vehicle_name);
        handler.release_offer(get.vehicle_name, false);
    }
}