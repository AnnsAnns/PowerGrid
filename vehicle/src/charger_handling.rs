use log::{debug, info};
use powercable::{
    charger::{ChargeOffer, ChargeRequest, PRICE_DISTANCE_FACTOR}, Position, CHARGER_ACCEPT, CHARGER_REQUEST
};
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
        position: Position {
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
 * Handles incoming charge requests from chargers. 
 */
pub async fn receive_offer(vehicle: SharedVehicle, payload: ChargeOffer) {
    let mut handler = vehicle.lock().await;

    handler.charge_offers.push(payload.clone());
    info!(
        "Received charge offer from {}: {} kWh at {}€",
        payload.charger_name, payload.charge_amount, payload.charge_price
    );
}

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

    handler.vehicle.set_status(VehicleStatus::Driving);
    handler.vehicle.set_destination(
        offer.charger_position.latitude,
        offer.charger_position.longitude,
    );

    info!(
        "Accepting best offer from {}: {} kWh at {}€",
        offer.charger_name,
        offer.charge_amount,
        offer.charge_price
    );

    handler.charge_offers.clear();
    handler.target_charger = Some(offer.clone());

    handler.client.publish(
        CHARGER_ACCEPT,
        QoS::ExactlyOnce,
        false,
        offer.to_bytes(),
    ).await.unwrap()
}
