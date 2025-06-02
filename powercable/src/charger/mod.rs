mod charge_request;
mod charge_offer;

/// This is the distance factor used to calculate the price of a charge
/// offer based on the distance to the charger.
pub const PRICE_DISTANCE_FACTOR: f64 = 0.3;

pub use self::charge_request::ChargeRequest;
pub use self::charge_offer::ChargeOffer;