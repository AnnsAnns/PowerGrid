use std::sync::Arc;

use log::info;
use powercable::{tickgen::{Phase, TickPayload}, POWER_NETWORK_TOPIC};
use rumqttc::QoS;
use tokio::sync::Mutex;

use crate::TurbineHandler;

pub async fn process_tick(
    handler: Arc<Mutex<TurbineHandler>>,
) {
    let (client, power) = {
        let mut handler = handler.lock().await;
        handler.offer_handler.remove_all_offers();
        handler.turbine.tick();
        handler.turbine.approximate_wind_data().await;
        handler.turbine.approximate_temperature_data().await;
        handler.remaining_power = handler.turbine.get_power_output();
        info!("Current power output: {} Watt", handler.remaining_power);

        (handler.client.clone(), handler.remaining_power)
    };

    client
    .publish(
        POWER_NETWORK_TOPIC,
        QoS::ExactlyOnce,
        false,
        power.to_string(),
    )
    .await;
}

pub async fn commerce_tick(
    handler: Arc<Mutex<TurbineHandler>>,
) {
    
}

pub async fn handle_tick(
    handler: Arc<Mutex<TurbineHandler>>,
    payload: TickPayload,
) {
    match payload.phase {
        Phase::Process => process_tick(handler.clone()).await,
        Phase::Commerce => commerce_tick(handler.clone()).await,
    }
}