use handler::{ack_buy_offer, handle_buy_offer, handle_tick};
use tracing::{debug, info};
use powercable::*;
use precalculated_turbine::PrecalculatedTurbine;
use rumqttc::AsyncClient;
use std::sync::Arc;
use tokio::{sync::Mutex, task};

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
    let (handler, mut eventloop) = init::init(location, true, seed).await;

    let name = handler.lock().await.name.clone();
    info!("Turbine simulation started with name: {}", name);

    init::subscribe(handler.clone()).await;

    info!("Turbine simulation started. Waiting for messages...");
    loop {
        let event = eventloop.poll().await;
        match event {
            Ok(v) => {
                debug!("Event = {v:?}");
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = v {
                    match p.topic.as_str() {
                        TICK_TOPIC => task::spawn(handle_tick(handler.clone(), p.payload.clone())),
                        BUY_OFFER_TOPIC => {
                            task::spawn(handle_buy_offer(handler.clone(), p.payload.clone()))
                        }
                        ACK_ACCEPT_BUY_OFFER_TOPIC => task::spawn(ack_buy_offer(handler.clone(), p.payload.clone())),
                        _ => task::spawn(async move {
                            debug!("Unknown topic: {}", p.topic);
                        }),
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
