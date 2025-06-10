use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{charger::{Arrival, ChargeAccept, ChargeOffer, ChargeRequest, Port, PRICE_DISTANCE_FACTOR}, Position, CHARGER_ACCEPT, CHARGER_ARRIVAL, CHARGER_REQUEST};
use rumqttc::QoS;
use crate::SharedVehicle;
use crate::vehicle::VehicleStatus;

/**
 * Sends a charge request to all chargers.
 * This function creates a ChargeRequest message containing the vehicle's name, the amount of charge needed,
 * and the vehicle's current position. Important is that the amount doesn´t contains the amount of energy needed to drive to the charger
 * 
 * @param handler: The shared vehicle handler containing the vehicle and its state.
 */
pub async fn create_charger_request(handler: SharedVehicle) {
    let handler = handler.lock().await;

    let request = ChargeRequest {
        vehicle_name: handler.vehicle.get_name().clone(),
        charge_amount: handler.vehicle.battery_non_mut().get_free_capacity(),
        vehicle_position: Position {
            latitude: handler.vehicle.get_latitude(),
            longitude: handler.vehicle.get_longitude(),
        },
    };
    info!(
        "Sending charge request for {} kWh from vehicle {} at position ({}, {})",
        request.charge_amount, request.vehicle_name, request.vehicle_position.latitude, request.vehicle_position.longitude
    );

    // publish charging request to all chargers
    handler
        .client
        .publish(CHARGER_REQUEST, QoS::ExactlyOnce, false, request.to_bytes())
        .await
        .unwrap()
}

/**
 * Handles incoming charge offers from chargers.
 * 
 * @param handler: The shared vehicle handler containing the vehicle and its state.
 * @param payload: The payload of the message received on the CHARGER_OFFER topic, containing the charge offer.
 */
pub async fn receive_offer(handler: SharedVehicle, payload: Bytes) {
    let mut handler = handler.lock().await;

    // Deserialize the ChargeOffer message
    let charge_offer = ChargeOffer::from_bytes(payload).unwrap();

    if charge_offer.vehicle_name.eq(handler.vehicle.get_name()) {
        info!(
            "Received charge offer from {}: {} kWh at {}€",
            charge_offer.charger_name, charge_offer.charge_amount, charge_offer.charge_price
        );
        handler.charge_offers.push(charge_offer.clone());
    }
}

/**
 * Accepts the best charge offer based on the distance to the charger and the charge price.
 * 
 * @param handler: The shared vehicle handler containing the vehicle and its state.
 */
pub async fn accept_offer(handler: SharedVehicle) {
    let mut handler = handler.lock().await;

    if handler.charge_offers.is_empty() {
        info!("No charge offers available to accept.");
        return;
    }
    // We calculate the best offer based on the distance to the charger * PRICE_DISTANCE_FACTOR and the charge price * (1.0-PRICE_DISTANCE_FACTOR)
    let mut best_offer = (None, 1.0);
    for offer in handler.charge_offers.iter() {
        let distance = handler.vehicle.distance_to(offer.charger_position.latitude, offer.charger_position.longitude);
        let price =
            offer.charge_price * (1.0 - PRICE_DISTANCE_FACTOR) + distance * PRICE_DISTANCE_FACTOR;

        debug!(
            "Offer from {}: {} kWh at {}€ (distance: {} km, calculated price (with distance): {:.2}€)",
            offer.charger_name,
            offer.charge_amount,
            offer.charge_price,
            distance,
            price
        );

        if best_offer.0.is_none() || price < best_offer.1 {
            best_offer = (Some(offer.clone()), price);
        }
    }

    // Has to be something by now
    let offer = best_offer.0.unwrap();
    
    // drive to the charger
    handler.vehicle.set_status(VehicleStatus::SearchingForCharger);
    handler.vehicle.set_destination(Position {
        latitude: offer.charger_position.latitude,
        longitude: offer.charger_position.longitude,
    });
    
    info!(
        "Accepting best offer from {}: {} kWh at {}€",
        offer.charger_name,
        offer.charge_amount,
        offer.charge_price
    );

    handler.charge_offers.clear();
    handler.target_charger = Some(offer.clone());

    // Create a ChargeAccept message to send to the charger
    debug!("{} has to drive {}km to the charger(lib.rs)",
        handler.vehicle.get_name(),
        handler.vehicle.get_location().distance_to(Position {
            latitude: offer.charger_position.latitude,
            longitude: offer.charger_position.longitude,
        })
    );
    let energy_for_way = handler.vehicle.distance_to(
        offer.charger_position.latitude,
        offer.charger_position.longitude,
    ) * handler.vehicle.get_consumption();

    let acceptance = ChargeAccept {
        charger_name: offer.charger_name.clone(),
        vehicle_name: handler.vehicle.get_name().clone(),
        real_amount: offer.charge_amount as usize + energy_for_way as usize,// real amount is the charge amount + energy for the way to the charger
    };
    info!(
        "Vehicle {} accepts charge offer from {}: {} kWh (including {} kWh for the way to the charger)",
        handler.vehicle.get_name(),
        acceptance.charger_name,
        acceptance.real_amount,
        energy_for_way
    );

    handler.client.publish(
        CHARGER_ACCEPT,
        QoS::ExactlyOnce,
        false,
        acceptance.to_bytes(),
    ).await.unwrap()
}

/**
 * Sends an arrival message to the charger when the vehicle arrives at the charger.
 * The arrival message contains the charger's name, the vehicle's name, and the amount of energy needed to charge.
 * 
 * @param handler: The shared vehicle handler containing the vehicle and its state.
 */
pub async fn create_arrival(handler: SharedVehicle) {
    let handler = handler.lock().await;
    let arrival: Arrival = Arrival {
        charger_name: handler.target_charger.as_ref().unwrap().charger_name.clone(),
        vehicle_name: handler.vehicle.get_name().clone(),
        needed_amount: handler.vehicle.battery_non_mut().get_free_capacity(),
    };
    info!("Send arrival: {:?} to charger: {}", arrival, arrival.charger_name);
    handler.client.publish(
        CHARGER_ARRIVAL,
        QoS::ExactlyOnce,
        false,
        arrival.to_bytes(),
    ).await.unwrap()
}

/**
 * If handler receives a message on the CHARGER_PORT topic, it means that a port is available for the vehicle to charge.
 * This function checks if the port is for this vehicle and sets the port number in the vehicle.
 * 
 * @param handler: The shared vehicle handler containing the vehicle and its state.
 * @param payload: The payload of the message received on the CHARGER_PORT topic, containing the port information.
 */
pub async fn receive_port(handler: SharedVehicle, payload: Bytes) {
    let mut handler = handler.lock().await;

    // Deserialize the Port message
    let port: Port = Port::from_bytes(payload).unwrap();

    // Check if the port is for this vehicle
    if port.vehicle_name.eq(handler.vehicle.get_name()) {
        info!(
            "Received port information from charger {}: Port {} is now available for vehicle {}",
            port.charger_name, port.port, port.vehicle_name
        );
        handler.vehicle.set_port(Some(port.port));
    }
}

