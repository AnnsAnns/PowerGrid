use bitcode::{Decode, Encode};
use bytes::Bytes;

/**
* # Description
* Represents a charge acceptance message sent by a charger to a vehicle.
* This message is sent when a vehicle accepts a charge offer from a charger.
* 
* # Fields
* - `charger_name`: The name of the charger accepting the request.
* - `vehicle_name`: The name of the vehicle that made the charge request.
*/
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeAccept {
    pub charger_name: String,
    pub vehicle_name: String,
    // TODO: estimated arrival time
}

impl ChargeAccept {
    /**
     * # Description
     * Creates a new ChargeAccept message.
     * 
     * # Arguments
     * * `charger_name`: The name of the charger.
     * * `vehicle_name`: The name of the vehicle.
     * 
     * # Returns
     * A new ChargeAccept instance.
     */
    pub fn new(
        charger_name: String,
        vehicle_name: String,
    ) -> Self {
        ChargeAccept {
            charger_name,
            vehicle_name,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}