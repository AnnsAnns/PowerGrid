use log::{debug, info};
use powercable::{
    charger::{ChargeOffer, ChargeRequest, PRICE_DISTANCE_FACTOR}, CHARGER_ACCEPT, CHARGER_REQUEST
};
use rumqttc::QoS;

use crate::SharedVehicle;

pub async fn create_charger_request(vehicle: SharedVehicle) {
    let handler = vehicle.lock().await;

    let request = ChargeRequest {
        own_id: handler.vehicle.get_name().clone(),
        charge_amount: handler.vehicle.battery_non_mut().state_of_charge() * 100.0,
        latitude: handler.vehicle.get_latitude(),
        longitude: handler.vehicle.get_longitude(),
    };

    handler
        .client
        .publish(CHARGER_REQUEST, QoS::ExactlyOnce, false, request.to_bytes())
        .await
        .unwrap()
}

pub async fn receive_offer(vehicle: SharedVehicle, payload: ChargeOffer) {
    let mut handler = vehicle.lock().await;

    handler.charge_offers.push(payload.clone());
    info!(
        "Received charge offer from {}: {} kWh at {}€",
        payload.own_id, payload.charge_amount, payload.charge_price
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
        let distance = handler.vehicle.distance_to(offer.latitude, offer.longitude);
        let price =
            offer.charge_price * (1.0 - PRICE_DISTANCE_FACTOR) + distance * PRICE_DISTANCE_FACTOR;

        debug!(
            "Offer from {}: {} kWh at {}€ (distance: {} km, calculated price (with distance): {:.2}€)",
            offer.own_id,
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

    handler.vehicle.set_destination(
        offer.latitude,
        offer.longitude,
    );

    info!(
        "Accepting best offer from {}: {} kWh at {}€",
        offer.own_id,
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
