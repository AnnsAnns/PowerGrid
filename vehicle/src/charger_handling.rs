use crate::vehicle::{Vehicle, VehicleAlgorithm, VehicleStatus};
use crate::SharedVehicle;
use bytes::Bytes;
use tracing::{debug, info};
use powercable::{
    charger::*, generate_rnd_pos, CHARGER_ACCEPT, CHARGER_CHARGING_GET, CHARGER_CHARGING_RELEASE, CHARGER_REQUEST
};
use rand::Rng;
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
 * Receives a charge offer from a charger and stores it.
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

    // Determine the best offer based on the vehicle's algorithm
    let best_offer = match handler.vehicle.get_algorithm() {
        VehicleAlgorithm::Best => get_best_offer(&handler.charge_offers, handler.vehicle.clone()),
        VehicleAlgorithm::Random => get_random_offer(&handler.charge_offers),
        VehicleAlgorithm::Cheapest => get_cheapest_offer(&handler.charge_offers),
        VehicleAlgorithm::Closest => get_closest_offer(&handler.charge_offers, handler.vehicle.clone()),
    }.unwrap();

    // drive to the charger
    handler.vehicle.set_status(VehicleStatus::SearchingForCharger);
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
        ).await.unwrap();
}

/// # Description
/// Selects the best charge offer based the cost.
/// It is calculated by multiplying the charge price with the charge amount.<br>
/// Bcause every charge offer has a different distance to the vehicle, the energy needed to drive to the charger is also considered.<br>
/// If no offer is found that satisfies the vehicle's needs, the best available offer is returned.
/// 
/// # Parameters
/// - `offers`: A slice of `ChargeOffer` instances representing the available charge offers.
/// - `vehicle`: The vehicle for which the best charge offer is being selected.
/// 
/// # Returns
/// An `Option<ChargeOffer>` containing the best charge offer, or `None` if no offers are available.
fn get_best_offer(offers: &[ChargeOffer], vehicle: Vehicle) -> Option<ChargeOffer> {
    debug!("Selecting the best charge offer.");
    if offers.is_empty() {
        return None;
    }
    
    // Create a sorted vector by cost (charge_price * charge_amount)
    let mut sorted_offers: Vec<ChargeOffer> = offers.to_vec();
    sorted_offers.sort_by(|a, b| {
        let costs = a.charge_price * a.charge_amount as f64;
        let costs_b = b.charge_price * b.charge_amount as f64;
        costs.partial_cmp(&costs_b).unwrap()
    });
    debug!("Sorted offers by cost: {:?}", sorted_offers);

    // Return the first offer that is enough to fully charge the vehicle
    for offer in &sorted_offers {
        let energy_for_way = (vehicle.distance_to(offer.charger_position) * (vehicle.get_consumption()/ 100.0)) as usize;// km * kWh/km = kWh
        let needed_amount = vehicle.battery_non_mut().get_free_capacity() as usize + energy_for_way;// including the energy for the way
        debug!("needed amount: {} kWh, offer from {}: {} kWh, costs: {}", needed_amount, offer.charger_name, offer.charge_amount, offer.charge_price * offer.charge_amount as f64);
        if offer.charge_amount >= needed_amount {
            return Some(offer.clone());
        }
    };
    debug!("No satisfying offer found, returning best unsatisfying offer: {:?}", sorted_offers.first());
    return sorted_offers.first().cloned();// If no satifiable offer was found, return the best offer that doesn't satisfy
}

/// # Description
/// Selects a random charge offer from the list of available offers.
/// 
/// # Parameters
/// - `offers`: A slice of `ChargeOffer` instances representing the available charge offers.
/// 
/// # Returns
/// An `Option<ChargeOffer>` containing a randomly selected charge offer, or `None` if no offers are available.
fn get_random_offer(offers: &[ChargeOffer]) -> Option<ChargeOffer> {
    debug!("Selecting a random charge offer.");
    if offers.is_empty() {
        return None;
    }
    
    let mut rng = rand::rng();// TODO: use seed for reproducibility
    Some(offers[rng.random_range(0..offers.len())].clone())
}

/// # Description
/// Selects the cheapest charge offer from the list of available offers.
/// 
/// # Parameters
/// - `offers`: A slice of `ChargeOffer` instances representing the available charge offers.
/// 
/// # Returns
/// An `Option<ChargeOffer>` containing the cheapest charge offer, or `None` if no offers are available.
fn get_cheapest_offer(offers: &[ChargeOffer]) -> Option<ChargeOffer> {
    debug!("Selecting the cheapest charge offer.");
    if offers.is_empty() {
        return None;
    }
    debug!("Offers: {:?}", offers);
    let res = offers.iter().min_by(|a, b| a.charge_price.partial_cmp(&b.charge_price).unwrap()).cloned();
    debug!("Cheapest offer: {:?}", res);
    res
}

/// # Description
/// Selects the closest charge offer based on the vehicle's current position.
/// 
/// # Parameters
/// - `offers`: A slice of `ChargeOffer` instances representing the available charge offers.
/// - `vehicle`: The vehicle for which the closest charge offer is being selected.
/// 
/// # Returns
/// An `Option<ChargeOffer>` containing the closest charge offer, or `None` if no offers are available.
fn get_closest_offer(offers: &[ChargeOffer], vehicle: Vehicle) -> Option<ChargeOffer> {
    debug!("Selecting the closest charge offer.");
    if offers.is_empty() {
        return None;
    }
    debug!("Offers: {:?}", offers);
    let res = offers.iter().min_by(|a, b| vehicle.distance_to(a.charger_position).partial_cmp(&vehicle.distance_to(b.charger_position)).unwrap()).cloned();
    debug!("Closest offer: {:?}", res);
    res
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
        ).await.unwrap();
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
                ).await.unwrap();

            handler.vehicle.set_status(VehicleStatus::Random);
            handler.target_charger = None;
            handler.vehicle.set_destination(generate_rnd_pos());
        }
    }
}