use fake::faker::lorem::de_de::Word;
use fake::Fake;
use rand::Rng;

pub mod charger;
pub mod offer;
pub mod tickgen;

pub use offer::Offer;
pub use offer::offer_handler::OfferHandler;

pub const BUY_OFFER_TOPIC: &str = "market/buy_offer";
pub const ACCEPT_BUY_OFFER_TOPIC: &str = "market/accept_buy_offer";
pub const ACK_ACCEPT_BUY_OFFER_TOPIC: &str = "market/ack_accept_buy_offer";
pub const TICK_TOPIC: &str = "tickgen/tick";
pub const TICK_CONFIGURE: &str = "tickgen/configure";
pub const TICK_CONFIGURE_SPEED: &str = "tickgen/configure_speed";
pub const POWER_NETWORK_TOPIC: &str = "power/network";
pub const POWER_TRANSFORMER_CONSUMPTION_TOPIC: &str = "power/transformer/consumption";
pub const POWER_TRANSFORMER_GENERATION_TOPIC: &str = "power/transformer/generation";
pub const POWER_TRANSFORMER_DIFF_TOPIC: &str = "power/transformer/diff";
pub const POWER_CHARGER_TOPIC: &str = "power/charger";
pub const POWER_LOCATION_TOPIC: &str = "power/location";
pub const MQTT_BROKER: &str = "mosquitto_broker";
pub const MQTT_BROKER_PORT: u16 = 1883;

pub fn generate_latitude_longitude_within_germany() -> (f64, f64) {
    let mut rng = rand::rng();
    let latitude = rng.random_range(47.2701..55.0581);
    let longitude = rng.random_range(5.8663..15.0419);
    (latitude, longitude)
}

pub fn get_id_from_topic(topic: &str) -> String {
    // Split the topic into parts
    let parts: Vec<&str> = topic.split('/').collect();
    // Check if we have enough parts to extract an ID
    if parts.len() > 2 {
        // Return the ID portion (last part)
        return parts[parts.len() - 1].to_string();
    }
    "".to_string()
}

pub fn generate_unique_name() -> String {
    Word().fake()
}