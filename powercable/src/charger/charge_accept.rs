use bitcode::{Decode, Encode};
use bytes::Bytes;

/// # Description
/// ChargeAccept represents a message sent by a charger to a vehicle to accept a charging request.
/// 
/// # Fields
/// - `charger_name`: The name of the charger accepting the request.
/// - `vehicle_name`: The name of the vehicle that is being accepted for charging.
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeAccept {
    pub charger_name: String,
    pub vehicle_name: String,
    // TODO: estimated arrival time
}

impl ChargeAccept {
    /// # Description
    /// Creates a new ChargeAccept instance.
    /// 
    /// # Arguments
    /// - `charger_name`: The name of the charger accepting the request.
    /// - `vehicle_name`: The name of the vehicle that is being accepted for charging.
    /// 
    /// # Returns
    /// A new ChargeAccept instance with the specified charger and vehicle names.
    pub fn new(
        charger_name: String,
        vehicle_name: String,
    ) -> Self {
        ChargeAccept {
            charger_name,
            vehicle_name,
        }
    }

    /// # Description
    /// Creates a ChargeAccept instance from a byte array.
    /// 
    /// # Arguments
    /// - `bytes`: A byte array containing the encoded ChargeAccept message.
    /// 
    /// # Returns
    /// A Result containing the ChargeAccept instance if decoding is successful, or an error if it fails.
    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    /// # Description
    /// Converts the ChargeAccept instance to a byte array.
    /// 
    /// # Returns
    /// A Bytes instance containing the encoded ChargeAccept message.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}