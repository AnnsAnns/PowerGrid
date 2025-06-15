use bitcode::{Decode, Encode};
use bytes::Bytes;
use crate::Position;

/// # Description
/// ChargeRequest represents a request from a vehicle to a charger for charging.
/// 
/// # Fields
/// - `vehicle_name`: The name of the vehicle making the request.
/// - `charge_amount`: The amount of charge requested in kWh.
/// - `vehicle_position`: The position of the vehicle making the request.
/// - `vehicle_consumption`: The vehicle's consumption rate in kWh/100km.
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeRequest {
    pub vehicle_name: String,
    pub charge_amount: usize,
    pub vehicle_position: Position,
    pub vehicle_consumption: f64,
}

impl ChargeRequest {
    /// # Description
    /// Creates a new ChargeRequest instance.
    /// 
    /// # Arguments
    /// - `vehicle_name`: The name of the vehicle making the request.
    /// - `charge_amount`: The amount of charge requested in kWh.
    /// - `vehicle_position`: The position of the vehicle making the request.
    /// - `vehicle_consumption`: The vehicle's consumption rate in kWh/100km.
    /// 
    /// # Returns
    /// A new ChargeRequest instance with the specified parameters.
    pub fn new(vehicle_name: String, charge_amount: usize, vehicle_position: Position, vehicle_consumption: f64) -> Self {
        ChargeRequest {
            vehicle_name,
            charge_amount,
            vehicle_position,
            vehicle_consumption,
        }
    }

    /// # Description
    /// Creates a ChargeRequest instance from a byte array.
    /// 
    /// # Arguments
    /// - `bytes`: A byte array containing the encoded ChargeRequest message.
    /// 
    /// # Returns
    /// A Result containing the ChargeRequest instance if decoding is successful, or an error if it fails.
    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    /// # Description
    /// Converts the ChargeRequest instance to a byte array.
    /// 
    /// # Returns
    /// A Bytes instance containing the encoded ChargeRequest message.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}