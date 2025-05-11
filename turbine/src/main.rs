use handler::handle_tick;
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
                        TICK_TOPIC => {
                            let payload =
                                serde_json::from_slice::<powercable::tickgen::TickPayload>(
                                    &p.payload,
                                )
                                .unwrap();

                            task::spawn(handle_tick(
                                handler.clone(),
                                payload,
                            ));
                        }
                        powercable::BUY_OFFER_TOPIC => {
                            let offer: powercable::Offer =
                                serde_json::from_slice(&p.payload).unwrap();
                            info!("Received buy offer: {:?}", offer);
                            offer_handler.add_offer(offer.clone());
                        }
                        powercable::ACCEPT_BUY_OFFER_TOPIC => {
                            let id: String = powercable::get_id_from_topic(&p.topic);
                            let offer = offer_handler.get_offer(id.as_str());
                            if offer.is_some() {
                                let offer = offer.unwrap();
                                info!("Accepted buy offer: {:?}", offer);
                                client
                                    .publish(
                                        powercable::ACK_ACCEPT_BUY_OFFER_TOPIC,
                                        QoS::ExactlyOnce,
                                        false,
                                        id,
                                    )
                                    .await
                                    .unwrap();
                            } else {
                                info!("No offer found for ID: {}", id);
                            }
                        }
                        powercable::ACK_ACCEPT_BUY_OFFER_TOPIC => {
                            let offer_id = powercable::get_id_from_topic(&p.topic);
                            info!("Received ACK for offer: {}", offer_id);
                            if let Some(offer) = offer_handler.get_offer(offer_id.as_str()) {
                                info!("Offer accepted: {:?}", offer);
                                offer_handler.remove_offer(offer_id.as_str());
                            }
                        }
                        &_ => {
                            info!("Unknown topic: {}", p.topic);
                        }
                    }
                }
            }
            Err(e) => {
                debug!("Error = {e:?}");
                break;
            }
        }
    }
}
