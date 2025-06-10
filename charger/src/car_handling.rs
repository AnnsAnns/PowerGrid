use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{charger::{Arrival, ChargeAccept, ChargeOffer, ChargeRequest, Port}, offer, CHARGER_OFFER, CHARGER_PORT};
use rumqttc::QoS;

use crate::{offer_handling::ReservedOffer, SharedCharger};

pub async fn receive_request(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    let request: ChargeRequest = ChargeRequest::from_bytes(payload).unwrap();
    info!("Received charge request from {}: {} kWh at ({}, {})", 
           request.vehicle_name, request.charge_amount, request.vehicle_position.latitude, request.vehicle_position.longitude);
    
    if handler.charger.get_free_ports() == 0 {
        info!("Charger {} has no free ports, rejecting request from {}", handler.charger.get_name(), request.vehicle_name);
        return;
    }

    let charge_amount = request.charge_amount;
    let reserable_charge = handler.charger.get_available_charge();
    info!("Charger {} has {} kWh available for reservation", handler.charger.get_name(), reserable_charge);
    
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
        handler.charger.get_name().clone(),
        request.vehicle_name,
        price,
        reserved_charge as f64,
        handler.charger.get_position().clone(),
    );

    info!("Sending charge offer to {}: {} kWh at {}€", offer.vehicle_name, offer.charge_amount, offer.charge_price);
    handler.client.publish(
        CHARGER_OFFER,
        rumqttc::QoS::ExactlyOnce,
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
        debug!("We were not accepted by {}, removing from reserved list", acceptance.vehicle_name);
        handler.release_offer(acceptance.vehicle_name.clone());
    } else {
        info!("We were accepted by {}", acceptance.vehicle_name);
        handler.accept_reserve(acceptance.vehicle_name.clone());
    }
}

pub async fn answer_arrival_with_port(charger: SharedCharger, payload: Bytes) {
    let mut handler = charger.lock().await;

    // Deserialize the payload into an Arrival object
    let arrival = Arrival::from_bytes(payload).unwrap();

    if &arrival.vehicle_name != handler.charger.get_name() {
        debug!("Arrival from vehicle {} is not for this charger {}", arrival.vehicle_name, handler.charger.get_name());
        return;
    }

    // Check if the arrival is for this charger
    if &arrival.charger_name != handler.charger.get_name() {
        debug!("Arrival from vehicle {} is not for this charger {}", arrival.vehicle_name, handler.charger.get_name());
        return;
    }
    info!("Received arrival from {}", arrival.vehicle_name);

    // get free port
    let port: Port = Port::new(handler.charger.get_name().clone(), arrival.vehicle_name, handler.charger.use_port().unwrap());

    handler.client.publish(
        CHARGER_PORT,
        QoS::ExactlyOnce,
        false,
        port.to_bytes(),
    ).await.unwrap()
}