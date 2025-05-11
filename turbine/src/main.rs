use handler::{accept_buy_offer, ack_buy_offer, handle_buy_offer, handle_tick};
use log::{debug, info};
use powercable::*;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};
use turbine::Turbine;

mod handler;
mod init;
mod meta_data;
mod parsing;
mod turbine;

pub(crate) type SharedTurbine = Arc<Mutex<TurbineHandler>>;

struct TurbineHandler {
    pub turbine: Turbine,
    pub offer_handler: OfferHandler,
    pub client: AsyncClient,
    pub remaining_power: f64,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
    info!("Starting turbine simulation...");

    let (handler, eventloop) = init::init().await;

    init::subscribe(handler.clone()).await;

    info!("Turbine simulation started. Waiting for messages...");
    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                debug!("Event = {v:?}");
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = v {
                    match p.topic.as_str() {
                        TICK_TOPIC => task::spawn(handle_tick(handler.clone(), p.payload)),
                        BUY_OFFER_TOPIC => task::spawn(handle_buy_offer(
                            handler.clone(),
                            p.payload,
                        )),
                        ACCEPT_BUY_OFFER_TOPIC => task::spawn(accept_buy_offer(
                            handler.clone(),
                            p.topic,
                        )),
                        ACK_ACCEPT_BUY_OFFER_TOPIC => task::spawn(ack_buy_offer(handler, p.topic)),
                        _ => {
                            debug!("Unknown topic: {}", p.topic);
                        }
                    };
                };
            }
            Err(e) => {
                debug!("Error = {e:?}");
                break;
            }
        }
    }
}
