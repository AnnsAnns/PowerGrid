use handler::{ack_buy_offer, handle_buy_offer, handle_tick};
use tracing::{info, warn};
use powercable::*;
use precalculated_turbine::PrecalculatedTurbine;
use rumqttc::AsyncClient;
use std::{sync::Arc, thread::spawn};
use tokio::{sync::Mutex, task};

use crate::handler::scale_handler;
use crate::handler::show_handler;

mod handler;
mod init;
mod meta_data;
mod parsing;
mod turbine;
mod precalculated_turbine;

pub(crate) type SharedTurbine = Arc<Mutex<TurbineHandler>>;

struct TurbineHandler {
    pub name: String,
    pub turbine: PrecalculatedTurbine,
    pub offer_handler: OfferHandler,
    pub client: AsyncClient,
    pub remaining_power: f64,
    pub total_earned: f64,
}

pub async fn start_turbine(location: usize) {
    // Print working directory
    //println!("Current working directory: {:?}", std::env::current_dir());

    let seed = generate_seed(location as u64, OwnType::Turbine);
    let (shared_turbine, mut eventloop) = init::init(location, true, seed).await;

    let name = shared_turbine.lock().await.name.clone();
    info!("Turbine simulation started with name: {}", name);

    init::subscribe(shared_turbine.clone()).await;

    info!("Turbine simulation started. Waiting for messages...");
    while let Ok(notification) = eventloop.poll().await {
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = notification {
            match p.topic.as_str() {
                TICK_TOPIC => {
                    task::spawn(handle_tick(shared_turbine.clone(), p.payload.clone()));
                }
                BUY_OFFER_TOPIC => {
                    task::spawn(handle_buy_offer(shared_turbine.clone(), p.payload.clone()));
                }
                ACK_ACCEPT_BUY_OFFER_TOPIC => {
                    task::spawn(ack_buy_offer(shared_turbine.clone(), p.payload.clone()));
                }
                CONFIG_TURBINE_SCALE => {
                    task::spawn(scale_handler(shared_turbine.clone(), p.payload));
                }
                CONFIG_TURBINE => {
                    task::spawn(show_handler(shared_turbine.clone(), p.payload));
                }
                _ => {
                    warn!("Unknown topic: {}", p.topic);
                }
            }
        }
    }
    println!("Exiting turbine simulation...");
}