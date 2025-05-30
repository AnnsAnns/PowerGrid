use log::{debug, info, warn};
use powercable::{
    tickgen::{Phase, TickPayload, INTERVAL_15_MINS},
    ChartEntry, Offer, OfferHandler, ACCEPT_BUY_OFFER_TOPIC, POWER_TRANSFORMER_GENERATION_TOPIC,
};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};

const OWN_TOPIC: &str = "Fusion Reactor";
const SELL_PRICE: f64 = 0.90;

struct FusionReactor {
    total_power_produced: f64,
    cash_earned: f64,
    offer_handler: OfferHandler,
    client: AsyncClient,
    power_sold_this_tick: f64,
}

async fn process_offers(handler: Arc<Mutex<FusionReactor>>, tick_payload: TickPayload) {
    let mut power_this_tick = 0.0;
    while handler.lock().await.offer_handler.has_offers() {
        let mut offer = handler
            .lock()
            .await
            .offer_handler
            .get_first_offer()
            .unwrap()
            .clone();
        debug!(
            "Processing offer {} with price {} and amount {}",
            offer.get_id(),
            offer.get_price(),
            offer.get_amount()
        );

        offer.set_accepted_by(OWN_TOPIC.to_string());

        handler
            .lock()
            .await
            .client
            .publish(
                ACCEPT_BUY_OFFER_TOPIC,
                QoS::AtMostOnce,
                true,
                offer.to_bytes(),
            )
            .await
            .unwrap();

        handler
            .lock()
            .await
            .offer_handler
            .add_sent_offer(offer.clone());

        handler
            .lock()
            .await
            .offer_handler
            .remove_offer(offer.get_id());
    }
}

async fn process_buy(handler: Arc<Mutex<FusionReactor>>, offer: Offer) {
    if offer.get_price() < SELL_PRICE {
        debug!(
            "Received offer with price {} below minimum sell price {}",
            offer.get_price(),
            SELL_PRICE
        );
        return;
    }

    handler.lock().await.offer_handler.add_offer(offer.clone());
}

async fn process_accept_buy_offer(handler: Arc<Mutex<FusionReactor>>, offer: Offer) {
    if offer.get_accepted_by().is_none() {
        warn!(
            "Received accepted offer {} without accepted_by field",
            offer.get_id()
        );
        return;
    }

    if handler
        .lock()
        .await
        .offer_handler
        .has_sent_offer(&offer.get_id())
    {
        if offer.get_accepted_by().unwrap() != OWN_TOPIC {
            debug!(
                "Offer {} was accepted by {} - not by us",
                offer.get_id(),
                offer.get_accepted_by().unwrap()
            );
            // No need to handle remaining power as fusion reactor doesn't track it
        } else {
            debug!(
                "Our offer {} was accepted by us ({})",
                offer.get_id(),
                offer.get_accepted_by().unwrap()
            );

            let power_to_sell = offer.get_amount();
            handler.lock().await.power_sold_this_tick += power_to_sell;
            handler.lock().await.cash_earned += power_to_sell as f64 * offer.get_price();
            info!(
                "Selling {} kWh for {} EUR",
                power_to_sell,
                offer.get_price()
            );
        }
    }
}

async fn process_tick(handler: Arc<Mutex<FusionReactor>>, tick_payload: TickPayload) {
    let mut handler = handler.lock().await;
    handler.offer_handler.remove_all_offers();

    handler
        .client
        .publish(
            POWER_TRANSFORMER_GENERATION_TOPIC,
            QoS::AtMostOnce,
            true,
            ChartEntry::new(
                OWN_TOPIC.to_string(),
                handler.power_sold_this_tick as isize,
                tick_payload.timestamp,
            )
            .to_string(),
        )
        .await
        .unwrap();

    handler.total_power_produced += handler.power_sold_this_tick;
    handler.power_sold_this_tick = 0.0;
}

#[tokio::main]
async fn main() {
    let log_path = format!("logs/fusion_reactor.log");
    let _log2 = log2::open(log_path.as_str()).level("debug").start();
    info!("Starting fusion reactor simulation...");

    let mut mqttoptions = MqttOptions::new(
        "fusion_reactor",
        powercable::MQTT_BROKER,
        powercable::MQTT_BROKER_PORT,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let mut fusion_reactor = Arc::new(Mutex::new(FusionReactor {
        total_power_produced: 0.0,
        cash_earned: 0.0,
        offer_handler: OfferHandler::new(),
        client: client.clone(),
        power_sold_this_tick: 0.0,
    }));

    client
        .subscribe(powercable::TICK_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::BUY_OFFER_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    client
        .subscribe(powercable::ACK_ACCEPT_BUY_OFFER_TOPIC, QoS::AtMostOnce)
        .await
        .unwrap();
    info!("Connected to MQTT broker");

    while let Ok(notification) = eventloop.poll().await {
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                powercable::TICK_TOPIC => {
                    let tick_payload: TickPayload = serde_json::from_slice(&p.payload).unwrap();

                    match tick_payload.phase {
                        Phase::Process => {
                            task::spawn(process_tick(fusion_reactor.clone(), tick_payload));
                        }
                        Phase::Commerce => {
                            debug!("Commerce phase");
                        }
                        Phase::PowerImport => {
                            task::spawn(process_offers(fusion_reactor.clone(), tick_payload));
                        }
                    }
                }
                powercable::BUY_OFFER_TOPIC => {
                    let offer = Offer::from_bytes(p.payload).unwrap();
                    task::spawn(process_buy(fusion_reactor.clone(), offer));
                }
                powercable::ACK_ACCEPT_BUY_OFFER_TOPIC => {
                    let offer = Offer::from_bytes(p.payload).unwrap();
                    task::spawn(process_accept_buy_offer(fusion_reactor.clone(), offer));
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }

    info!(
        "Simulation complete! Total power produced: {:.2} kWh, Cash earned: {:.2} EUR",
        fusion_reactor.lock().await.total_power_produced,
        fusion_reactor.lock().await.cash_earned
    );
}
