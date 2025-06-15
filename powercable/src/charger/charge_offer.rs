use bitcode::{Decode, Encode};
use bytes::Bytes;
use crate ::Position;

/// # Description
/// This module defines the `ChargeOffer` struct, which represents an offer made by a charger to a vehicle for charging services.
/// 
/// # Fields
/// - `charger_name`: The name of the charger making the offer.
/// - `vehicle_name`: The name of the vehicle for which the offer is made.
/// - `charge_price`: The price per unit of charge offered by the charger.
/// - `charge_amount`: The amount of charge offered by the charger, in kWh.
/// - `charger_position`: The position of the charger in the world map, represented as a `Position` struct.
#[derive(Debug, Clone, Encode, Decode)]
pub struct ChargeOffer {
    pub charger_name: String,
    pub vehicle_name: String,
    pub charge_price: f64,
    pub charge_amount: usize,
    pub charger_position: Position,
}

impl ChargeOffer {
    /// # Description
    /// Creates a new `ChargeOffer` instance with the specified parameters.
    /// 
    /// # Parameters
    /// - `charger_name`: The name of the charger making the offer.
    /// - `vehicle_name`: The name of the vehicle for which the offer is made.
    /// - `charge_price`: The price per unit of charge offered by the charger.
    /// - `charge_amount`: The amount of charge offered by the charger, in kWh.
    /// - `charger_position`: The position of the charger in the world map, represented as a `Position` struct.
    /// 
    /// # Returns
    /// A new `ChargeOffer` instance with the specified parameters.
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        charge_price: f64,
        charge_amount: usize,
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

    /// # Description
    /// Creates a new `ChargeOffer` instance from a byte array.
    /// 
    /// # Parameters
    /// - `bytes`: A `Bytes` object containing the encoded `ChargeOffer`.
    /// 
    /// # Returns
    /// A `Result` containing the decoded `ChargeOffer` or an error if decoding fails.
    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    /// # Description
    /// Encodes the `ChargeOffer` instance into a byte array.
    /// 
    /// # Returns
    /// A `Bytes` object containing the encoded `ChargeOffer`.
    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}