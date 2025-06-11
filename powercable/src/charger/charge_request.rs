use bitcode::{Decode, Encode};
use bytes::Bytes;
use crate::Position;

#[derive(Debug, Clone, Encode, Decode)]
/**
 * ChargeRequest represents a request from a vehicle to find a suitable charger.
 * It includes the vehicle's name, the amount of charge requested, and the vehicle's position (latitude, longitude).
 */
pub struct ChargeRequest {
    pub vehicle_name: String,
    pub charge_amount: usize, // in kWh
    pub vehicle_position: Position,
}

impl ChargeRequest {
    pub fn new(vehicle_name: String, charge_amount: usize, vehicle_position: Position) -> Self {
        ChargeRequest {
            vehicle_name,
            charge_amount,
            vehicle_position,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}