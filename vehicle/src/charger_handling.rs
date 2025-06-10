use bytes::Bytes;
use log::{debug, info, warn};
use powercable::{charger::{ChargeAccept, ChargeOffer, ChargeRequest, PRICE_DISTANCE_FACTOR}, Position, CHARGER_ACCEPT, CHARGER_REQUEST};
use rumqttc::QoS;
use crate::SharedVehicle;
use crate::vehicle::VehicleStatus;

/**
 * Sends a request to all chargers in the network to find a suitable charger for the vehicle.
 * Sends request on the CHARGER_REQUEST topic with the vehicle's current state of charge, latitude, and longitude.
 * // TODO: Keep that below in mind:
 * !! Every charger has to calculate the SoC based on vehicles SoC and distance to the charger !!
 * !! If charger is too far away, it should not respond to the request !!
 */
pub async fn create_charger_request(vehicle: SharedVehicle) {
    let handler = vehicle.lock().await;
    info!("Creating charger request for vehicle: {}", handler.vehicle.get_name());

    let request = ChargeRequest {
        vehicle_name: handler.vehicle.get_name().clone(),
        charge_amount: handler.vehicle.battery_non_mut().get_free_capacity(),
        vehicle_position: Position {
            latitude: handler.vehicle.get_latitude(),
            longitude: handler.vehicle.get_longitude(),
        },
    };

    // publish charging request to all chargers
    handler
        .client
        .publish(CHARGER_REQUEST, QoS::ExactlyOnce, false, request.to_bytes())
        .await
        .unwrap()
}

/**
 * Handles incoming charge offers from chargers. 
 */
pub async fn receive_offer(vehicle: SharedVehicle, payload: Bytes) {
    let mut handler = vehicle.lock().await;
    let charge_offer = match ChargeOffer::from_bytes(payload) {
        Ok(offer) => offer,
        Err(e) => {
            warn!("Failed to deserialize ChargeOffer: {}", e);
            return;
        }
    };
    if charge_offer.vehicle_name.eq(handler.vehicle.get_name()) {
        warn!(
            "Received charge offer from {}: {} kWh at {}€",
            charge_offer.charger_name, charge_offer.charge_amount, charge_offer.charge_price
        );
        handler.charge_offers.push(charge_offer.clone());
    }
    else {
        warn!(
            "Received charge offer for {} but this vehicle is {}. Ignoring offer.",
            charge_offer.vehicle_name, handler.vehicle.get_name()
        );
    }
}

/**
 * Accepts the best charge offer based on the distance to the charger and the charge price.
 */
pub async fn accept_offer(vehicle: SharedVehicle) {
    let mut handler = vehicle.lock().await;

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
    info!("Driving to charger: {}", offer.charger_name);

    // drive to the charger
    handler.vehicle.set_status(VehicleStatus::Driving);
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
        "Sending charge acceptance to charger: {} with real amount: {} kWh",
        acceptance.charger_name, acceptance.real_amount
    );

    handler.client.publish(
        CHARGER_ACCEPT,
        QoS::ExactlyOnce,
        false,
        acceptance.to_bytes(),
    ).await.unwrap()
}

pub async fn receive_charging_port(vehicle: SharedVehicle, payload: Bytes) {
    let mut handler = vehicle.lock().await;
}

