use bitcode::{Decode, Encode};
use bytes::Bytes;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Encode, Decode)]
pub struct ChargeRequest {
    pub own_id: String,
    pub charge_amount: f64,
    pub latitude: f64,
    pub longitude: f64,
}

impl ChargeRequest {
    pub fn new(own_id: String, charge_amount: f64, latitude: f64, longitude: f64) -> Self {
        ChargeRequest {
            own_id,
            charge_amount,
            latitude,
            longitude,
        }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}