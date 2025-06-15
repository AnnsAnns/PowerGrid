use crate::vehicle::VehicleStatus;
use crate::SharedVehicle;
use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{
    charger::{ChargeAccept, ChargeOffer, ChargeRequest, Get}, generate_rnd_pos, CHARGER_ACCEPT, CHARGER_CHARGING_GET, CHARGER_CHARGING_RELEASE, CHARGER_REQUEST
};
use rumqttc::QoS;

/**
 * # Description
 * Sends a charge request to all chargers.<br>
 * This function creates a ChargeRequest message containing the vehicle's name, the amount of charge needed,
 * and the vehicle's current position.<br>
 * Important is that the amount doesn´t contains the amount of energy needed to drive to the charger
 *
 * # Parameters
 * - `handler`: The shared vehicle handler containing the vehicle and its state.
 */
pub async fn create_charger_request(handler: SharedVehicle) {
    let handler = handler.lock().await;

    let request = ChargeRequest {
        vehicle_name: handler.vehicle.get_name().clone(),
        charge_amount: handler.vehicle.battery_non_mut().get_free_capacity() as usize,// TODO
        vehicle_position: handler.vehicle.get_location(),
        vehicle_consumption: handler.vehicle.get_consumption(),
    };

    // publish charging request to all chargers
    info!("Sending charge request {:?}", request);
    handler
        .client
        .publish(CHARGER_REQUEST, QoS::ExactlyOnce, false, request.to_bytes())
        .await
        .unwrap()
}

/**
 * # Description
 * Receives a charge offer from a charger and stores it in the vehicle's state.
 *
 * # Parameters
 * - `handler`: The shared vehicle handler containing the vehicle and its state.
 * - `payload`: The payload of the message received on the CHARGER_OFFER topic, containing the charge offer information.
 */
pub async fn receive_offer(handler: SharedVehicle, payload: Bytes) {
    let mut handler = handler.lock().await;

    // Deserialize the ChargeOffer message
    let charge_offer = ChargeOffer::from_bytes(payload).unwrap();
    

    // Check if the offer is for the current vehicle
    if charge_offer.vehicle_name.eq(&handler.vehicle.get_name()) {
        debug!("Received charge offer: {:?}", charge_offer);
        handler.charge_offers.push(charge_offer.clone());
    }
}

/// # Description
/// Accepts the best charge offer available.<br>
/// This function calculates the best offer based on the distance to the charger and the charge price,<br>
/// then drives the vehicle to the charger and publishes an acceptance message.
/// 
/// # Parameters
/// - `handler`: The shared vehicle handler containing the vehicle and its state.
pub async fn accept_offer(handler: SharedVehicle) {
    let mut handler = handler.lock().await;

    if handler.charge_offers.is_empty() {
        info!("No charge offers available to accept.");
        return;
    }
    
    let mut best_satisfiable_offer = (None, f64::MAX);// If satisfied, the lower the cost, the better
    let mut best_unsatisfiable_offer = (None, usize::MIN);// If not satisfied, the higher the amount, the better
    info!("Evaluating charge offers...");
    for offer in handler.charge_offers.iter() {
        if handler.vehicle.distance_to(offer.charger_position) > handler.vehicle.get_range() {
            debug!("Offer from {} is too far away: {} km, vehicle range: {} km",
                offer.charger_name, handler.vehicle.distance_to(offer.charger_position), handler.vehicle.get_range());
            continue;
        }

        let energy_for_way = (handler.vehicle.distance_to(offer.charger_position) * (handler.vehicle.get_consumption()/ 100.0)) as usize;// km * kWh/km = kWh
        debug!("energy needed for way to {}: {} kWh", offer.charger_name, energy_for_way);
        let needed_amount = handler.vehicle.battery_non_mut().get_free_capacity() as usize + energy_for_way;// including the energy for the way
        debug!("needed amount for charging: {} kWh", needed_amount);
        if offer.charge_amount < needed_amount {// Offer is not enough to fully charge
            debug!("Offer from {} is not enough to fully charge", offer.charger_name);
            if best_unsatisfiable_offer.0.is_none() || offer.charge_amount > best_unsatisfiable_offer.1 {// If not satisfied, the higher the amount, the better
                best_unsatisfiable_offer = (Some(offer.clone()), offer.charge_amount);
                debug!("Amount is {}", offer.charge_amount);
            }
        } else {// Offer is enough to fully charge
            debug!("Offer from {} is enough to fully charge", offer.charger_name);
            let costs = offer.charge_amount as f64 * offer.charge_price;
            debug!("Cost is {}", costs);
            if best_satisfiable_offer.0.is_none() || costs < best_satisfiable_offer.1 {
                best_satisfiable_offer = (Some(offer.clone()), costs);
            }
        }
    }

    let best_offer = if best_satisfiable_offer.0.is_some() {
        debug!("Found satisfiable offer");
        best_satisfiable_offer.0.unwrap()
    } else {
        if best_unsatisfiable_offer.0.is_some() {
            debug!("Found unsatisfiable offer");
            best_unsatisfiable_offer.0.unwrap()
        } else {
            warn!("No suitable charge offers found, take first offer.");
            handler.charge_offers[0].clone()// Fallback to the first offer if no suitable offer was found
        }
    };

    // drive to the charger
    handler
        .vehicle
        .set_status(VehicleStatus::SearchingForCharger);
    handler.vehicle.set_destination(best_offer.charger_position.clone());

    info!(
        "Accepting best offer from {}: {} kWh at {}€",
        best_offer.charger_name, best_offer.charge_amount, best_offer.charge_price
    );

    handler.charge_offers.clear();
    handler.target_charger = Some(best_offer.clone());

    let acceptance = ChargeAccept {
        charger_name: best_offer.charger_name.clone(),
        vehicle_name: handler.vehicle.get_name().clone(),
    };

    handler
        .client
        .publish(
            CHARGER_ACCEPT,
            QoS::ExactlyOnce,
            false,
            acceptance.to_bytes(),
        )
        .await
        .unwrap()
}

/// # Description
/// Creates a Get message to request charging from the target charger.
/// 
/// # Parameters
/// - `handler`: The shared vehicle handler containing the vehicle and its state.
pub async fn create_get(handler: SharedVehicle) {
    let handler = handler.lock().await;

    let get = Get {
        charger_name: handler
            .target_charger
            .as_ref()
            .unwrap()
            .charger_name
            .clone(),
        vehicle_name: handler.vehicle.get_name().clone(),
        amount: handler.vehicle.battery_non_mut().max_addable_charge(None),
    };

    info!("Sending get: {:?}", get);
    handler
        .client
        .publish(
            CHARGER_CHARGING_GET,
            QoS::ExactlyOnce,
            false,
            get.to_bytes(),
        )
        .await
        .unwrap();
}

pub async fn get_ack_handling(handler: SharedVehicle, payload: Bytes) {
    let mut handler = handler.lock().await;

    // Deserialize the Ack message
    let get = Get::from_bytes(payload).unwrap();

    // Check if the ack is for the current vehicle
    if get.vehicle_name.eq(&handler.vehicle.get_name()) {
        let amount_charged = handler.vehicle.battery().add_charge(get.amount);
        info!(
            "Charged {} kWh of {} kWh requested",
            amount_charged, get.amount
        );

        info!("Received charge offer acknowledgement: {:?}", get);
        info!(
            "Battery charge after ack: {} kWh, soc {}",
            handler.vehicle.battery().get_level(),
            handler.vehicle.battery().get_soc()
        );

        // At 85% state of charge, we consider the vehicle fully charged
        if handler.vehicle.battery().get_soc() >= 0.95 {
            info!("{} has been fully charged.", handler.vehicle.get_name());

            handler
                .client
                .publish(
                    CHARGER_CHARGING_RELEASE,
                    QoS::ExactlyOnce,
                    false,
                    get.to_bytes(),
                )
                .await
                .unwrap();

            handler.vehicle.set_status(VehicleStatus::RANDOM);
            handler.target_charger = None;
            handler.vehicle.set_destination(generate_rnd_pos());
        }
    }
}
