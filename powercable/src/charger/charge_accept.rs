use bitcode::{Decode, Encode};
use bytes::Bytes;

/**
 * ChargeAccept represents the acceptance of a charge offer by a vehicle.
 * It contains the name of the charger, the name of the vehicle,
 * and the real amount of charge at the cars arrival in kWh.
 */
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeAccept {
    pub charger_name: String,
    pub vehicle_name: String,
    // TODO: estimated arrival time
    pub real_amount: usize, // in kWh
}

impl ChargeAccept {
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        real_amount: usize,
    ) -> Self {
        ChargeAccept {
            charger_name,
            vehicle_name,
            real_amount,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}