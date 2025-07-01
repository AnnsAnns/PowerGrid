use std::{sync::Arc, time::Duration};

use powercable::*;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use serde_json::json;
use tokio::sync::Mutex;
use tracing::{info, trace};

use crate::{
    meta_data, precalculated_turbine::PrecalculatedTurbine, turbine, SharedTurbine, TurbineHandler,
};

const POSITIONS: [(f64, f64); 5] = [
    (53.596585151232766, 10.020507601699903), // Hamburg
    (52.483683040244046, 12.228618337143569), // Brandenburg
    (51.08546553534163, 7.218648830231728),   // Köln
    (49.32797939192757, 11.184455339704543),  // Nürnberg
    (48.34887278552995, 9.885435336999953),   // Ulm
];

pub async fn init(location: usize, use_dump: bool, seed: u64) -> (SharedTurbine, EventLoop) {
    let (latitude, longitude, name) = {
        let pos = POSITIONS
            .get(location % POSITIONS.len())
            .unwrap_or(&POSITIONS[0]);
        let name = format!("Turbine {}", generate_unique_name(seed));
        (pos.0, pos.1, name)
    };
    let name = format!("{} {}", name, generate_unique_name(seed)); // In cases where we have multiple turbines at the same location, we generate a unique name.

    let dump_file = format!("data/{}_turbine_dump.json", location);

    // We also want to create one if the file does not exist.
    let precalculated_turbine = if std::fs::metadata(dump_file.clone()).is_err() || !use_dump {
        info!(
            "Generating new turbine at location: ({}, {})",
            latitude, longitude
        );
        let turbine = turbine::Turbine::new(
            turbine::random_rotor_dimension(seed),
            latitude,
            longitude,
            meta_data::MetaDataWrapper::new(meta_data::MetaDataType::AirTemperature)
                .await
                .unwrap(),
            meta_data::MetaDataWrapper::new(meta_data::MetaDataType::Wind)
                .await
                .unwrap(),
        );
        let precalculated_turbine = PrecalculatedTurbine::from_turbine(turbine).await;
        PrecalculatedTurbine::dump_from_turbine(&precalculated_turbine, &dump_file);
        info!("Turbine dump file created: {}", dump_file);
        precalculated_turbine
    } else {
        info!(
            "Loading turbine from dump file: {}",
            dump_file
        );
        PrecalculatedTurbine::read_from_file(&dump_file)
    };

    let mut mqttoptions = MqttOptions::new(name.clone(), MQTT_BROKER, MQTT_BROKER_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(20));
    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

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

pub async fn publish_location(handler: SharedTurbine) {
    let mut handler = handler.lock().await;
    // Extract all values before mutably borrowing client
    let name = handler.name.clone();
    let latitude = handler.turbine.get_latitude();
    let longitude = handler.turbine.get_longitude();
    let power = handler.turbine.get_power_output();
    let earned = handler.total_earned;
    let visible = handler.turbine.visible;
    let client = &mut handler.client;
    let location_payload = json!({
        "name" : name,
        "lat": latitude,
        "lon": longitude,
        "icon": ":zap:",
        "label": format!("{:.1}kW ({:.1}€)", power, earned),
        "deleted": !visible,
    })
    .to_string();

    client
        .publish(
            POWER_LOCATION_TOPIC,
            QoS::ExactlyOnce,
            true,
            location_payload.clone(),
        )
        .await
        .unwrap();
    trace!(
        "Published on topic {}: {}",
        POWER_LOCATION_TOPIC,
        location_payload.clone()
    );
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
    client
        .subscribe(CONFIG_TURBINE_SCALE, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(CONFIG_TURBINE, QoS::ExactlyOnce)
        .await
        .unwrap();
    info!("Subscribed to topics");
}
