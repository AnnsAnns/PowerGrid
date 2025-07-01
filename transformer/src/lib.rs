use tracing::{debug, info, warn};
use powercable::{tickgen::{Phase, TickPayload, TICK_AS_SEC}, ChartEntry, Offer, ACK_ACCEPT_BUY_OFFER_TOPIC, POWER_TRANSFORMER_PRICE_TOPIC};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use transformer::Transformer;

mod transformer;

const OWN_TOPIC: &str = "Total";

pub async fn start_transformer() {
    info!("Starting turbine simulation...");
    
    let mut transformer = Transformer::new();

    let mut mqttoptions = MqttOptions::new(
        "transformer",
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    let mut lowest_sell_price_of_tick = 0.0;
    let mut sells_total: f64 = 0.0;
    let mut sell_amount: f64 = 0.0;
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(powercable::TICK_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::POWER_TRANSFORMER_CONSUMPTION_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::POWER_TRANSFORMER_GENERATION_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    client.subscribe(ACK_ACCEPT_BUY_OFFER_TOPIC, QoS::ExactlyOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    while let Ok(notification) = eventloop.poll().await {
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
                            powercable::POWER_TRANSFORMER_STATS_TOPIC,
                            QoS::ExactlyOnce,
                            true,
                            ChartEntry::new(
                                "Consumption".to_string(),
                                transformer.get_current_consumption() as isize,
                                tick_payload.timestamp - TICK_AS_SEC,
                            ).to_string(),
                        )
                        .await
                        .unwrap();
                    client
                        .publish(
                            powercable::POWER_TRANSFORMER_STATS_TOPIC,
                            QoS::ExactlyOnce,
                            true,
                            ChartEntry::new(
                                "Generation".to_string(),
                                transformer.get_current_power() as isize,
                                tick_payload.timestamp - TICK_AS_SEC,
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
                                OWN_TOPIC.to_string(),
                                transformer.get_difference() as isize,
                                tick_payload.timestamp - TICK_AS_SEC,
                            ).to_string(),
                        )
                        .await
                        .unwrap();

                    if sell_amount == 0.0 && sells_total == 0.0 {
                        debug!("No sells this tick, skipping price calculations");
                        continue;
                    }

                    client.publish(
                        POWER_TRANSFORMER_PRICE_TOPIC,
                        QoS::ExactlyOnce,
                        false,
                        ChartEntry::new(
                            "Lowest Sell Price".to_string(),
                            (lowest_sell_price_of_tick * 100.0) as isize,
                            tick_payload.timestamp - TICK_AS_SEC,
                        ).to_string(),
                    ).await.unwrap();

                    let average_sell_price = ((sells_total / sell_amount) * 100.0) as isize;

                    debug!("Average Sell Price: {}", average_sell_price);
                    debug!("Total Sells: {}, Sell Amount: {}", sells_total, sell_amount);
                    debug!("Lowest Sell Price of Tick: {}", lowest_sell_price_of_tick);

                    client.publish(
                        POWER_TRANSFORMER_PRICE_TOPIC,
                        QoS::ExactlyOnce,
                        false,
                        ChartEntry::new(
                            "Average Sell Price".to_string(),
                            average_sell_price,
                            tick_payload.timestamp - TICK_AS_SEC,
                        ).to_string(),
                    ).await.unwrap();

                    lowest_sell_price_of_tick = 1.0;
                    sells_total = 0.0;
                    sell_amount = 0.0;
                    transformer.reset();
                }

                powercable::POWER_TRANSFORMER_GENERATION_TOPIC => {
                    let payload = ChartEntry::from_bytes(p.payload).unwrap();
                    if payload.topic == OWN_TOPIC {
                        continue;
                    }
                    debug!("Received generation data: {:?}", payload);
                    
                    transformer.add_power(payload.payload as f64);
                }

                powercable::POWER_TRANSFORMER_CONSUMPTION_TOPIC => {
                    let payload = ChartEntry::from_bytes(p.payload).unwrap();
                    if payload.topic == OWN_TOPIC {
                        continue;
                    }
                    debug!("Received consumption data: {:?}", payload);
                    
                    transformer.add_consumption(payload.payload as f64);
                }

                ACK_ACCEPT_BUY_OFFER_TOPIC => {
                    let offer = Offer::from_bytes(p.payload).unwrap();
                    debug!("Received Offer ACK: {:?}", offer);
                    if offer.get_id().starts_with("L") || offer.get_id().starts_with("G") || offer.get_id().starts_with("H") {
                        debug!("Ignoring Consumer ACKs");
                        continue;
                    }

                    sells_total += offer.get_price();
                    sell_amount += 1.0;

                    if offer.get_price() < lowest_sell_price_of_tick {
                        lowest_sell_price_of_tick = offer.get_price();
                    }
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }

    info!("Exiting transformer simulation");
}
