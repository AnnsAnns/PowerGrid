use log::{debug, info, warn};
use powercable::{tickgen::{Phase, TickPayload}, ChartEntry};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use transformer::Transformer;

mod transformer;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Warn)
        .init();
    info!("Starting turbine simulation...");
    let mut transformer = Transformer::new();

    let mut mqttoptions = MqttOptions::new(
        "transformer",
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::POWER_NETWORK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    while let Ok(notification) = eventloop.poll().await {
        debug!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                powercable::TICK_TOPIC => {
                    let tick_payload: TickPayload = serde_json::from_slice(&p.payload).unwrap();
                    if tick_payload.phase != Phase::Process {
                        debug!("Ignoring tick payload");
                        continue;
                    }

                    client
                        .publish(
                            powercable::POWER_TRANSFORMER_CONSUMPTION_TOPIC,
                            QoS::ExactlyOnce,
                            true,
                            ChartEntry::new(
                                "Total".to_string(),
                                transformer.get_current_consumption() as isize,
                                tick_payload.timestamp,
                            ).to_string(),
                        )
                        .await
                        .unwrap();
                    client
                        .publish(
                            powercable::POWER_TRANSFORMER_GENERATION_TOPIC,
                            QoS::ExactlyOnce,
                            true,
                            ChartEntry::new(
                                "Total".to_string(),
                                transformer.get_current_power() as isize,
                                tick_payload.timestamp,
                            ).to_string(),
                        )
                        .await
                        .unwrap();
                    client
                        .publish(
                            powercable::POWER_TRANSFORMER_DIFF_TOPIC,
                            QoS::ExactlyOnce,
                            true,
                            ChartEntry::new(
                                "Total".to_string(),
                                transformer.get_difference() as isize,
                                tick_payload.timestamp,
                            ).to_string(),
                        )
                        .await
                        .unwrap();
                    transformer.reset();
                }
                powercable::POWER_NETWORK_TOPIC => {
                    let parameter: f64 = serde_json::from_slice(&p.payload).unwrap();
                    log::debug!("Received parameter: {}", parameter);
                    if parameter > 0.0 {
                        transformer.add_power(parameter);
                    } else {
                        transformer.add_consumption(parameter.abs());
                    }
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
}
