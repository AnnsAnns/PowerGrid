use bitcode::{Decode, Encode};
use bytes::Bytes;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Encode, Decode)]
pub struct ChargeOffer {
    pub own_id: String,
    pub charge_target: String,
    pub charge_price: f64,
    pub charge_amount: f64,
    pub latitude: f64,
    pub longitude: f64,
}

impl ChargeOffer {
    pub fn new(
        own_id: String,
        charge_target: String,
        charge_price: f64,
        charge_amount: f64,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        ChargeOffer {
            own_id,
            charge_target,
            charge_price,
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