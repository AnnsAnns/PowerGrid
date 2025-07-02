use bitcode::{Decode, Encode};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
use serde::Serialize;

pub mod charger;
pub mod chart_entry;
pub mod offer;
pub mod tickgen;

pub use chart_entry::ChartEntry;
pub use offer::offer_handler::OfferHandler;
pub use offer::Offer;

pub const CONFIG_TURBINE : &str = "config/turbine"; // also for fusion reactor
pub const CONFIG_CONSUMER: &str = "config/consumer";
pub const CONFIG_VEHICLE: &str = "config/vehicle"; // also for chargers
pub const CONFIG_TURBINE_SCALE: &str = "config/turbine/scale";
pub const CONFIG_CONSUMER_SCALE: &str = "config/consumer/scale";
pub const CONFIG_VEHICLE_SCALE: &str = "config/vehicle/scale";
pub const CONFIG_VEHICLE_ALGORITHM: &str = "config/vehicle/algorithm";
pub const BUY_OFFER_TOPIC: &str = "market/buy_offer";
pub const ACCEPT_BUY_OFFER_TOPIC: &str = "market/accept_buy_offer";
pub const ACK_ACCEPT_BUY_OFFER_TOPIC: &str = "market/ack_accept_buy_offer";
pub const TICK_TOPIC: &str = "tickgen/tick";
pub const TICK_CONFIGURE: &str = "tickgen/configure";
pub const TICK_CONFIGURE_SPEED: &str = "tickgen/configure_speed";
pub const TICK_CONFIGURE_AMOUNT_TO_RUN: &str = "tickgen/configure_amount_to_run";
pub const POWER_TRANSFORMER_CONSUMPTION_TOPIC: &str = "power/transformer/consumption";
pub const POWER_TRANSFORMER_CONSUMPTION_TOPIC_FORMATTED: &str = "power/transformer/consumption/transformed";
pub const POWER_TRANSFORMER_GENERATION_TOPIC: &str = "power/transformer/generation";
pub const POWER_TRANSFORMER_STATS_TOPIC: &str = "power/transformer/stats";
pub const POWER_TRANSFORMER_DIFF_TOPIC: &str = "power/transformer/diff";
pub const POWER_TRANSFORMER_PRICE_TOPIC: &str = "power/transformer/stats/price";
pub const POWER_TRANSFORMER_EARNED_TOPIC: &str = "power/transformer/stats/earnings";
pub const POWER_CHARGER_TOPIC: &str = "power/charger";
pub const POWER_CHARGER_TRANSFORMED_TOPIC: &str = "power/charger/transformed";
pub const POWER_CONSUMER_TOPIC: &str = "power/consumer";
pub const POWER_LOCATION_TOPIC: &str = "power/location";
pub const WORLDMAP_EVENT_TOPIC: &str = "worldmap/event";
pub const CHARGER_REQUEST: &str = "charger/request";// vehicle send request to all chargers
pub const CHARGER_OFFER: &str = "charger/offer";// charger sends offer to vehicle
pub const CHARGER_OFFER_AVG_PRICE: &str = "charger/offer/avg/price"; // transformer sends average offer price to frontend
pub const CHARGER_OFFER_AVG_DISTANCE: &str = "charger/offer/avg/distance"; // transformer sends average offer distance to frontend
pub const CHARGER_OFFER_AVG_COST: &str = "charger/offer/avg/cost"; // transformer sends average offer cost to frontend
pub const CHARGER_ACCEPT: &str = "charger/accept";// vehicle accepts offer from charger
pub const CHARGER_CHARGING_GET: &str = "charger/charging/get";// vehicle requests energy from the charger
pub const CHARGER_CHARGING_ACK: &str = "charger/charging/ack";// charger responds with energy to vehicle
pub const CHARGER_CHARGING_RELEASE: &str = "charger/charging/release";
pub const VEHICLE_TOPIC: &str = "vehicle";
pub const MQTT_BROKER: &str = "mosquitto_broker";
pub const MQTT_BROKER_PORT: u16 = 1883;
pub const MAP_UPDATE_SPEED_IN_SECS: u64 = 1;
pub const RANDOM_SEED: u64 = 02_07_25; // Seed for random number generation

/// We use prime numbers to represent different types of entities in the system.
/// This helps to ensure that the generated IDs are unique and can be easily distinguished.
pub enum OwnType {
    Charger = 7,
    Vehicle = 3,
    Consumer = 11,
    Turbine = 5,
}

/// Generate a random seed based on the index and the type of entity.
/// There is a low probability of collisions, but it is acceptable for our use case.
pub fn generate_seed(i: u64, own_type: OwnType) -> u64 {
    (i+1) * own_type as u64 + RANDOM_SEED
}

// Kiel
const NORTH_LIMIT: (f64, f64) = (54.236555997661384, 9.828710882743488);
// Leipzig
const EAST_LIMIT: (f64, f64) = (51.57629017432522, 12.427933450893512);
// Stuttgart
const SOUTH_LIMIT: (f64, f64) = (49.11158947259421, 10.206213793834436);
// Essen
const WEST_LIMIT: (f64, f64) = (51.00929968161735, 6.282484743251983);

/// The threshold for considering two positions as "near" each other, in kilometers.
const NEAR_THRESHOLD: f64 = 6.0;

/// # Description
/// The Position struct represents a geographical position defined by latitude and longitude.
/// It is used to represent the position of vehicles, chargers, and other entities in the system.
/// 
/// # Fields
/// - `latitude`: The latitude of the position in degrees.
/// - `longitude`: The longitude of the position in degrees.
#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode, Serialize)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

impl Position {
    /// # Description
    /// Creates a new Position instance with the specified latitude and longitude.
    /// 
    /// # Arguments
    /// - `latitude`: The latitude of the position in degrees.
    /// - `longitude`: The longitude of the position in degrees.
    /// 
    /// # Returns
    /// - A new Position instance with the specified latitude and longitude.
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Position { latitude, longitude }
    }

    /// # Description
    /// Calculates the distance to another position using the Haversine formula.
    /// 
    /// # Arguments
    /// - `other_position`: The other position to which the distance is calculated.
    /// 
    /// # Returns
    /// - The distance in kilometers between the two positions.
    pub fn distance_to(&self, other_position: Position) -> f64 {
        let earth_radius_km = 6371.0; // Radius of the Earth in kilometers

        let lat1_rad = self.latitude.to_radians();
        let lon1_rad = self.longitude.to_radians();
        let lat2_rad = other_position.latitude.to_radians();
        let lon2_rad = other_position.longitude.to_radians();

        let delta_lat = lat2_rad - lat1_rad;
        let delta_lon = lon2_rad - lon1_rad;

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        earth_radius_km * c
    }

    /// # Description
    /// Creates a new Position instance from a tuple containing latitude and longitude.
    /// 
    /// # Arguments
    /// - `position`: A tuple containing the latitude and longitude of the position.
    /// 
    /// # Returns
    /// - A new Position instance with the specified latitude and longitude.
    pub fn from_tuple(position: (f64, f64)) -> Self {
        Position {
            latitude: position.0,
            longitude: position.1,
        }
    }

    /// # Description
    /// Converts the Position instance to a tuple containing latitude and longitude.
    /// 
    /// # Returns
    /// - A tuple containing the latitude and longitude of the position.
    pub fn to_tuple(&self) -> (f64, f64) {
        (self.latitude, self.longitude)
    }

    /// # Description
    /// Checks if the current position is near another position within a defined threshold.
    /// 
    /// # Arguments
    /// - `other_position`: The other position to check against.
    /// 
    /// # Returns
    /// - A boolean indicating whether the current position is near the other position.
    pub fn is_near(&self, other_position: Position) -> bool {
        self.distance_to(other_position) < NEAR_THRESHOLD
    }
}

/// # Description
/// Generates a random geographical position within the defined limits of Germany.
/// 
/// # Returns
/// - A Position instance with a random latitude and longitude.
pub fn generate_rnd_pos(seed: u64) -> Position {
    let mut rng = StdRng::seed_from_u64(seed);
    let latitude = rng.random_range(SOUTH_LIMIT.0..NORTH_LIMIT.0);
    let longitude = rng.random_range(WEST_LIMIT.1..EAST_LIMIT.1);
    Position::new(latitude, longitude)
}

/// # Description
/// Extracts the ID from a topic string.
/// 
/// # Arguments
/// - `topic`: The topic string from which the ID is extracted.
/// 
/// # Returns
/// - A String containing the ID extracted from the topic. If the topic does not contain enough parts, an empty string is returned.
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

pub fn generate_unique_name(seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);
    let vowels = "aeiou";
    let consonants = "bcdfghjklmnpqrstvwxyz";
    let rand_vowel = |rng: &mut StdRng| vowels.chars().nth(rng.random_range(0..vowels.len())).unwrap();
    let rand_consonant = |rng: &mut StdRng| consonants.chars().nth(rng.random_range(0..consonants.len())).unwrap();

    let mut word: String = "".to_owned();
    for _ in 2..5 {
        word.push_str(&match rng.random_range(0..=3) {
            0 => format!("{}", rand_vowel(&mut rng)),
            1 => format!("{}{}", rand_vowel(&mut rng), rand_consonant(&mut rng)), 
            2 => format!("{}{}", rand_consonant(&mut rng), rand_vowel(&mut rng)),
            3 => format!("{}{}{}", rand_consonant(&mut rng), rand_vowel(&mut rng), rand_consonant(&mut rng)),
            _ => String::new(),
        });
    }

    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}