use bitcode::{Decode, Encode};
use bytes::Bytes;

/// # Description
/// ChargeAccept represents a message sent by a charger to a vehicle to accept a charging request.
/// 
/// # Fields
/// - `charger_name`: The name of the charger accepting the request.
/// - `vehicle_name`: The name of the vehicle that is being accepted for charging.
/// - `charge_price`: The price per unit of charge, which is calculated based on the distance to the vehicle and the current price of electricity.
/// - `distance`: The distance from the charger to the vehicle.
/// - `cost`: The total cost for the charging service, calculated as `charge_price * charge_amount`.
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeAccept {
    pub charger_name: String,
    pub vehicle_name: String,
    pub charge_price: f64,
    pub distance: f64,
    pub cost: f64,
    // TODO: estimated arrival time
}

impl ChargeAccept {
    /// # Description
    /// Creates a new ChargeAccept instance.
    /// 
    /// # Arguments
    /// - `charger_name`: The name of the charger accepting the request.
    /// - `vehicle_name`: The name of the vehicle that is being accepted for charging.
    /// - `charge_price`: The price per unit of charge, which is calculated based on the distance to the vehicle and the current price of electricity.
    /// - `distance`: The distance from the charger to the vehicle.
    /// - `cost`: The total price for the charging service, calculated as `charge_price * charge_amount`.
    /// 
    /// # Returns
    /// A new ChargeAccept instance with the specified charger and vehicle names and price.
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        charge_price: f64,
        distance: f64,
        cost: f64,
    ) -> Self {
        ChargeAccept {
            charger_name,
            vehicle_name,
            charge_price,
            distance,
            cost,
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