use bytes::Bytes;
use log::{debug, info, trace, warn};
use powercable::{charger::{Get, ChargeAccept, ChargeOffer, ChargeRequest}, CHARGER_OFFER, CHARGER_CHARGING_ACK};
use rumqttc::QoS;

use crate::{offer_handling::ReservedOffer, SharedCharger};

pub async fn receive_request(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    let charge_request= ChargeRequest::from_bytes(payload).unwrap();
    info!("Charge request: {:?}", charge_request);

    if handler.charger.get_free_ports() == 0 {
        info!("Charger {} has no free ports, rejecting request from {}", handler.charger.get_name(), charge_request.vehicle_name);
        return;
    }

    let energy_for_way = (charge_request.vehicle_position.distance_to(*handler.charger.get_position()) * charge_request.vehicle_consumption) as usize;
    info!("Vehicle {} needs {} kWh for the way to the charger", charge_request.vehicle_name, energy_for_way);
    let charge_amount = charge_request.charge_amount + energy_for_way;
    let reserable_charge = handler.charger.get_available_charge();
    info!("Charger {} has {} kWh available for reservation", handler.charger.get_name(), reserable_charge);

    let reserved_charge = if charge_amount > reserable_charge {
        reserable_charge
    } else {
        charge_amount
    };


    let price = handler.charger.get_current_price();

    let offer = ReservedOffer::new(
        charge_request.vehicle_name.clone(),
     reserved_charge, price);

    handler.reserve_offer(offer);

    let offer = ChargeOffer::new(
        handler.charger.get_name().clone(),
        charge_request.vehicle_name,
        price,
        reserved_charge as f64,
        handler.charger.get_position().clone(),
    );

    info!("Sending charge offer to {}: {} kWh at {}€", offer.vehicle_name, offer.charge_amount, offer.charge_price);
    handler.client.publish(
        CHARGER_OFFER,
        QoS::ExactlyOnce,
        false,
        offer.to_bytes(),
    ).await.unwrap();
}


/**
 * Checks if the charger was accepted by a vehicle.
 * Ignores accepts that are not for this charger.
 */
pub async fn accept_handler(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    // Deserialize the payload into a ChargeAccept object
    debug!("Received charge accept payload: {:?}", payload);
    let acceptance: ChargeAccept = ChargeAccept::from_bytes(payload).unwrap();

    let target_id = acceptance.vehicle_name.clone();

    // This is not something we care about
    if handler.get_reserved_offer(acceptance.vehicle_name.clone()).is_none() {
        debug!("Received offer for {} but we are not interested in it", target_id);
        return;
    } else if &acceptance.charger_name != handler.charger.get_name() {
        warn!("We were not accepted by {}, removing from reserved list", acceptance.vehicle_name);
        handler.release_offer(acceptance.vehicle_name.clone());
    } else {
        info!("We were accepted by {}", acceptance.vehicle_name);
        handler.accept_reserve(acceptance.vehicle_name.clone());
    }
}

/**
 * # Description
 * Handles a GET request from a vehicle.
 * 
 * # Parameters
 * - `charger`: The shared charger handler containing the charger and its state.
 * - `payload`: The payload containing the GET request data.
 */
pub async fn answer_get(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    // Deserialize the payload into a Get object
    trace!("Received get payload: {:?}", payload);
    let mut get: Get = Get::from_bytes(payload).unwrap();
    
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
    }
}