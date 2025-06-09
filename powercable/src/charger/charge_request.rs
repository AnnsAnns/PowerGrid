use bitcode::{Decode, Encode};
use bytes::Bytes;

use crate::Position;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Encode, Decode)]
/**
 * ChargeRequest represents a request from a vehicle to find a suitable charger.
 * It includes the vehicle's name, the amount of charge requested, and the vehicle's position (latitude, longitude).
 */
pub struct ChargeRequest {
    pub vehicle_name: String,
    pub charge_amount: usize, // in kWh
    pub position: Position,
}

impl ChargeRequest {
    pub fn new(vehicle_name: String, charge_amount: usize, latitude: f64, longitude: f64) -> Self {
        ChargeRequest {
            vehicle_name,
            charge_amount,
            position: Position::new(latitude, longitude),
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}