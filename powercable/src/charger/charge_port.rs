use bitcode::{Decode, Encode};
use bytes::Bytes;

/**
 * # Description
 * Arrival represents a request from a vehicle to a charger for charging.
 * 
 * # Fields
 * - `charger_name`: The name of the charger the vehicle is requesting to charge at.
 * - `vehicle_name`: The name of the vehicle that is requesting to charge.
 * - `needed_amount`: The amount of charge the vehicle needs.
 */
#[derive(Debug, Clone, Encode, Decode)]
pub struct Get {
    pub charger_name: String,
    pub vehicle_name: String,
    pub amount: usize,
}

impl Get {
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        amount: usize,
    ) -> Self {
        Get {
            charger_name,
            vehicle_name,
            amount,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}