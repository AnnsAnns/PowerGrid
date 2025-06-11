use bitcode::{Decode, Encode};
use bytes::Bytes;
use crate::Position;

#[derive(Debug, Clone, Encode, Decode)]
/**
 * # Description
 * Represents a charge request made by a vehicle to a charger.
 * 
 * # Fields
 * * `vehicle_name`: The name of the vehicle making the request.
 * * `charge_amount`: The amount of charge requested in kWh.
 * * `vehicle_position`: The position of the vehicle making the request.
 * * `vehicle_consumption`: The vehicle's consumption rate in kWh/km.
 */
pub struct ChargeRequest {
    pub vehicle_name: String,
    pub charge_amount: usize,
    pub vehicle_position: Position,
    pub vehicle_consumption: f64,
}

impl ChargeRequest {
    /**
     * # Description
     * Creates a new ChargeRequest instance.
     * 
     * # Arguments
     * * `vehicle_name`: The name of the vehicle making the request.
     * * `charge_amount`: The amount of charge requested in kWh.
     * * `vehicle_position`: The position of the vehicle making the request.
     * * `vehicle_consumption`: The vehicle's consumption rate in kWh/km.
     * 
     * # Returns
     * A new ChargeRequest instance.
     */
    pub fn new(vehicle_name: String, charge_amount: usize, vehicle_position: Position, vehicle_consumption: f64) -> Self {
        ChargeRequest {
            vehicle_name,
            charge_amount,
            vehicle_position,
            vehicle_consumption,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}