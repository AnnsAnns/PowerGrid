use bitcode::{Decode, Encode};
use bytes::Bytes;
use crate ::Position;

/**
 * # Description
 * Represents a charge offer made by a charger to a vehicle.
 * 
 * # Fields
 * * `charger_name`: The name of the charger making the offer.
 * * `vehicle_name`: The name of the vehicle receiving the offer.
 * * `charge_price`: The price per kWh for the charge.
 * * `charge_amount`: The amount of charge offered in kWh.
 * * `charger_position`: The position of the charger.
 * * `port`: The port number of the charger.
 */
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeOffer {
    pub charger_name: String,
    pub vehicle_name: String,
    pub charge_price: f64,
    pub charge_amount: f64,
    pub charger_position: Position,
}

/**
 * # Description
 * Creates a new ChargeOffer instance.
 * 
 * # Arguments
 * * `charger_name`: The name of the charger.
 * * `vehicle_name`: The name of the vehicle.
 * * `charge_price`: The price per kWh for the charge.
 * * `charge_amount`: The amount of charge offered in kWh.
 * * `charger_position`: The position of the charger.
 * * `port`: The port number of the charger.
 * 
 * # Returns
 * A new ChargeOffer instance.
 */
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