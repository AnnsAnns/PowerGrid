use crate::vehicle::VehicleStatus;
use crate::SharedVehicle;
use bytes::Bytes;
use log::{debug, info, trace};
use powercable::{
    charger::{ChargeAccept, ChargeOffer, ChargeRequest, Get, PRICE_DISTANCE_FACTOR}, generate_rnd_pos, Position, CHARGER_ACCEPT, CHARGER_CHARGING_GET, CHARGER_CHARGING_RELEASE, CHARGER_REQUEST
};
use rumqttc::{tokio_rustls::rustls::internal::msgs::handshake, QoS};

/**
 * # Description
 * Sends a charge request to all chargers.
 * This function creates a ChargeRequest message containing the vehicle's name, the amount of charge needed,
 * and the vehicle's current position. Important is that the amount doesn´t contains the amount of energy needed to drive to the charger
 *
 * # Parameters
 * - `handler`: The shared vehicle handler containing the vehicle and its state.
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
    if charge_offer.vehicle_name.eq(handler.vehicle.get_name()) {
        info!("Received charge offer: {:?}", charge_offer);
        info!(
            "{} is {}km away",
            charge_offer.charger_name,
            handler.vehicle.distance_to(
                charge_offer.charger_position.latitude,
                charge_offer.charger_position.longitude
            )
        );
        handler.charge_offers.push(charge_offer.clone());
    } else {
        debug!(
            "Received offer for another vehicle: {}. Ignoring.",
            charge_offer.vehicle_name
        );
        trace!(
            "Received offer for another vehicle: {:?}. Ignoring.",
            charge_offer
        );
    }
}

/**
 * # Description
 * Accepts the best charge offer based on the distance to the charger and the charge price.
 * The best offer is determined by calculating a price that combines the charge price and the distance to the charger.
 *
 * # Parameters
 * - `handler`: The shared vehicle handler containing the vehicle and its state.
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
        let distance = handler.vehicle.distance_to(
            offer.charger_position.latitude,
            offer.charger_position.longitude,
        );
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
    handler
        .vehicle
        .set_status(VehicleStatus::SearchingForCharger);
    handler.vehicle.set_destination(Position {
        latitude: offer.charger_position.latitude,
        longitude: offer.charger_position.longitude,
    });

    info!(
        "Accepting best offer from {}: {} kWh at {}€",
        offer.charger_name, offer.charge_amount, offer.charge_price
    );

    handler.charge_offers.clear();
    handler.target_charger = Some(offer.clone());

    let acceptance = ChargeAccept {
        charger_name: offer.charger_name.clone(),
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

/**
 * # Description
 * Creates a Get message to request the amount for one tick of charging.
 *
 * # Parameters
 * - `handler`: The shared vehicle handler containing the vehicle and its state.
 */
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
    if get.vehicle_name.eq(handler.vehicle.get_name()) {
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
