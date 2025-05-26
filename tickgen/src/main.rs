use log::{debug, info};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time};

#[tokio::main]
async fn main() {
    let log_path = format!("logs/tickgen.log");
    let _log2 = log2::open(log_path.as_str()).level("info").start();
    info!("Starting TickGen simulation...");

    let mut mqttoptions = MqttOptions::new(
        "tickgen",
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(powercable::TICK_CONFIGURE, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::TICK_CONFIGURE_SPEED, QoS::AtMostOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    let configuration = Arc::new(Mutex::new(powercable::tickgen::TickPayload {
        tick: 0,
        phase: powercable::tickgen::Phase::Process,
        timestamp: 0,
        configuration: powercable::tickgen::TickConfig {
            speed: 10.0,
            start_date: chrono::Utc::now().to_string(),
        },
    }));

    let configuration_clone = Arc::clone(&configuration);
    task::spawn(async move {
        let mut speed;
        loop {
            {
                let mut config = configuration_clone.lock().await;
                speed = config.configuration.speed;
                match config.phase {
                    powercable::tickgen::Phase::Process => {
                        config.phase = powercable::tickgen::Phase::Commerce;
                    }
                    powercable::tickgen::Phase::Commerce => {
                        config.phase = powercable::tickgen::Phase::PowerImport;
                    }
                    powercable::tickgen::Phase::PowerImport => {
                        config.tick += 1;
                        config.phase = powercable::tickgen::Phase::Process;
                    }
                }
                let start_date = config
                    .configuration
                    .start_date
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap();
                // Each tick is 15 minutes
                config.timestamp = (start_date
                    + chrono::Duration::minutes((config.tick * 15).try_into().unwrap()))
                .timestamp_millis() as usize;
                client
                    .publish(
                        powercable::TICK_TOPIC,
                        QoS::ExactlyOnce,
                        true,
                        serde_json::to_string(&*config).unwrap(),
                    )
                    .await
                    .unwrap();
            }
            time::sleep(Duration::from_secs_f64(speed / 2.0)).await;
        }
    });

    loop {
        let notification = match eventloop.poll().await {
            Ok(notification) => notification,
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        };
        debug!("Received = {:?}", notification);
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                powercable::TICK_CONFIGURE_SPEED => {
                    info!("Received speed configuration: {:?}", p.payload);

                    let new_speed: f64 = match std::str::from_utf8(&p.payload) {
                        Ok(s) => s.parse().unwrap_or(10.0),
                        Err(_) => {
                            info!("Invalid UTF-8 in speed configuration");
                            10.0
                        }
                    };
                    let config_copy = configuration.clone();
                    task::spawn(async move {
                        let mut config = config_copy.lock().await;
                        config.configuration.speed = new_speed;
                        info!("Updated speed configuration: {:?}", config);
                    });
                }
                powercable::TICK_CONFIGURE => {
                    let new_config: powercable::tickgen::TickConfig =
                        serde_json::from_slice(&p.payload).unwrap();
                    let config_copy = configuration.clone();
                    task::spawn(async move {
                        let mut config = config_copy.lock().await;
                        config.configuration = new_config;
                        info!("Updated configuration: {:?}", config);
                    });
                }
                _ => {
                    info!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
}
