use powercable::tickgen::TickPayload;
use rumqttc::AsyncClient;

mod handle_tick;

pub use handle_tick::handle_tick;