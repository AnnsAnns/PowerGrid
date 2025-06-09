use bitcode::{Decode, Encode};
use bytes::Bytes;
use crate ::Position;

/**
 * ChargeOffer represents an offer from a charger to a vehicle.
 * It includes the charger's name, the vehicle's name, the charge price, the amount of charge offered, and the charger's position.
 */
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Encode, Decode)]
pub struct ChargeOffer {
    pub charger_name: String,
    pub vehicle_name: String,
    pub charge_price: f64,
    pub charge_amount: f64,
    pub charger_position: Position,
}

impl ChargeOffer {
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        charge_price: f64,
        charge_amount: f64,
        charger_position: Position,
    ) -> Self {
        ChargeOffer {
            charger_name,
            vehicle_name,
            charge_price,
            charge_amount,
            charger_position,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}