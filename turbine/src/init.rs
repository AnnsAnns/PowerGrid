use std::{sync::Arc, time::Duration};

use log::{info, trace};
use powercable::*;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{meta_data, precalculated_turbine::{PrecalculatedTurbine}, turbine, SharedTurbine, TurbineHandler};

pub async fn init() -> (SharedTurbine, EventLoop) {
    let (latitude, longitude, name) = {
        let rand_pos = generate_rnd_pos();
        let name = format!("Turbine {}", generate_unique_name());
        (rand_pos.latitude, rand_pos.longitude, name)
    };
    let name = format!("{} {}", name, generate_unique_name()); // In cases where we have multiple turbines at the same location, we generate a unique name.

    let turbine = turbine::Turbine::new(
        turbine::random_rotor_dimension(),
        latitude,
        longitude,
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature)
            .await
            .unwrap(),
        meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind)
            .await
            .unwrap(),
    );

    let mut mqttoptions = MqttOptions::new(
        name.clone(),
        MQTT_BROKER,
        MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(20));
    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

    let precalculated_turbine = PrecalculatedTurbine::from_turbine(turbine).await;
    let offer_handler = OfferHandler::new();

    (
        Arc::new(Mutex::new(TurbineHandler {
            name,
            turbine: precalculated_turbine,
            offer_handler,
            client,
            remaining_power: 0.0,
            total_earned: 0.0,
        })),
        eventloop,
    )
}

pub async fn publish_location(
    handler: SharedTurbine,
) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.name.clone();
    let latitude = handler.turbine.get_latitude();
    let longitude = handler.turbine.get_longitude();
    let power = handler.turbine.get_power_output();
    let earned = handler.total_earned;
    let client = &mut handler.client;
    let location_payload = json!({
        "name" : name,
        "lat": latitude,
        "lon": longitude,
        "icon": ":zap:",
        "label": format!("{:.1}kW ({:.1}€)", power, earned),
    })
    .to_string();

    client.publish(
        POWER_LOCATION_TOPIC,
        QoS::ExactlyOnce,
        true,
        location_payload.clone(),
    ).await.unwrap();
    trace!("Published on topic {}: {}", POWER_LOCATION_TOPIC, location_payload.clone());
}

pub async fn subscribe(handler: SharedTurbine) {
    let mut handler = handler.lock().await;
    let client = &mut handler.client;
    client
        .subscribe(TICK_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(BUY_OFFER_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(ACK_ACCEPT_BUY_OFFER_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    info!("Subscribed to topics");
}
