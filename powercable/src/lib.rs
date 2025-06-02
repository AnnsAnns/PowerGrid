use fake::faker::lorem::de_de::Word;
use fake::Fake;
use rand::Rng;

pub mod charger;
pub mod chart_entry;
pub mod offer;
pub mod tickgen;

pub use chart_entry::ChartEntry;
pub use offer::offer_handler::OfferHandler;
pub use offer::Offer;

pub const BUY_OFFER_TOPIC: &str = "market/buy_offer";
pub const ACCEPT_BUY_OFFER_TOPIC: &str = "market/accept_buy_offer";
pub const ACK_ACCEPT_BUY_OFFER_TOPIC: &str = "market/ack_accept_buy_offer";
pub const TICK_TOPIC: &str = "tickgen/tick";
pub const TICK_CONFIGURE: &str = "tickgen/configure";
pub const TICK_CONFIGURE_SPEED: &str = "tickgen/configure_speed";
pub const POWER_TRANSFORMER_CONSUMPTION_TOPIC: &str = "power/transformer/consumption";
pub const POWER_TRANSFORMER_GENERATION_TOPIC: &str = "power/transformer/generation";
pub const POWER_TRANSFORMER_STATS_TOPIC: &str = "power/transformer/stats";
pub const POWER_TRANSFORMER_DIFF_TOPIC: &str = "power/transformer/diff";
pub const POWER_TRANSFORMER_PRICE_TOPIC: &str = "power/transformer/stats/price";
pub const POWER_TRANSFORMER_EARNED_TOPIC: &str = "power/transformer/stats/earnings";
pub const POWER_CHARGER_TOPIC: &str = "power/charger";
pub const POWER_CONSUMER_TOPIC: &str = "power/consumer";
pub const POWER_LOCATION_TOPIC: &str = "power/location";
pub const POWER_CONSUMER_SCALE: &str = "power/consumer/scale";
pub const WORLDMAP_EVENT_TOPIC: &str = "worldmap/event";
pub const CHARGER_REQUEST: &str = "charger/request";
pub const CHARGER_OFFER: &str = "charger/offer";
pub const CHARGER_ACCEPT: &str = "charger/accept";
pub const MQTT_BROKER: &str = "mosquitto_broker";
pub const MQTT_BROKER_PORT: u16 = 1883;
pub const MAP_UPDATE_SPEED_IN_SECS: u64 = 1;

// Around Neumünster
const NORTH_LIMIT: (f64, f64) = (54.08200660036042, 9.916791893513686);
// Around A24 between Trittau & Schwarzenbek
const EAST_LIMIT: (f64, f64) = (53.5519537146248, 10.67829930807445);
// Around Brackel
const SOUTH_LIMIT: (f64, f64) = (53.29354767455468, 10.043061643463892);
// Around Stade
const WEST_LIMIT: (f64, f64) = (53.59095432811228, 9.430617993044924);

pub fn generate_latitude_longitude_within_germany() -> (f64, f64) {
    let mut rng = rand::rng();
    let latitude = rng.random_range(SOUTH_LIMIT.0..NORTH_LIMIT.0);
    let longitude = rng.random_range(WEST_LIMIT.1..EAST_LIMIT.1);
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
