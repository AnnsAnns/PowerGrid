use bitcode::{Decode, Encode};
use bytes::Bytes;

/**
 * Arrival represents a vehicle arriving at a charger.
 * It includes the charger's name, the vehicle's name, and the amount of charge needed.
 */
#[derive(Debug, Clone, Encode, Decode)]
pub struct Arrival {
    pub charger_name: String,
    pub vehicle_name: String,
    pub needed_amount: usize,
}

impl Arrival {
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        needed_amount: usize,
    ) -> Self {
        Arrival {
            charger_name,
            vehicle_name,
            needed_amount,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}

/**
 * Port represents the answer from a charger to a vehicle
 * when the vehicle arrives at the charger.
 * It includes the charger's name, the vehicle's name, and the port number for the vehicle.
 */
#[derive(Debug, Clone, Encode, Decode)]
pub struct Port {
    pub charger_name: String,
    pub vehicle_name: String,
    pub port: usize,
}

impl Port {
    pub fn new(
        charger_name: String,
        vehicle_name: String,
        port: usize,
    ) -> Self {
        Port {
            charger_name,
            vehicle_name,
            port,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}