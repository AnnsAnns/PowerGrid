use std::{sync::Arc, time::Duration};

use log::info;
use powercable::*;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{handler, meta_data, turbine, SharedTurbine, TurbineHandler};

pub async fn init(name: String) -> (SharedTurbine, EventLoop) {
    let (latitude, longitude) = powercable::generate_latitude_longitude_within_germany();

    let mut turbine = turbine::Turbine::new(
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

    turbine.approximate_wind_data().await;
    turbine.approximate_temperature_data().await;
    let current_power = turbine.get_power_output();
    let offer_handler = OfferHandler::new();

    let location_payload = json!({
        "name" : name.clone(),
        "lat": latitude,
        "lon": longitude
    })
    .to_string();

    client
        .publish(
            "power/turbine/location",
            QoS::ExactlyOnce,
            true,
            location_payload,
        )
        .await
        .unwrap();

    info!(
        "Published location data: {:?}, {:?} to MQTT broker",
        latitude, longitude
    );

    (
        Arc::new(Mutex::new(TurbineHandler {
            name,
            turbine,
            offer_handler,
            client,
            remaining_power: current_power,
        })),
        eventloop,
    )
}

pub async fn subscribe(handler: SharedTurbine) {
    let mut handler = handler.lock().await;
    let client = &mut handler.client;
    client
        .subscribe(TICK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(BUY_OFFER_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(ACK_ACCEPT_BUY_OFFER_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    info!("Subscribed to topics");
}
