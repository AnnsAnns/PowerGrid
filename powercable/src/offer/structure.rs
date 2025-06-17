use bytes::Bytes;
use bitcode::{Encode, Decode};

pub const OFFER_PACKAGE_SIZE: f64 = 25.0;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Offer {
    id: String,
    price: f64,
    amount: f64,
    latitude: f64,// TODO: needed?
    longitude: f64,// TODO: needed?
    accepted_by: Option<String>,
    ack_for: Option<String>,
}

impl Offer {
    pub fn new(id: String, price: f64, amount: f64, latitude: f64, longitude: f64) -> Self {
        Offer { id, price, amount, latitude, longitude, accepted_by: None, ack_for: None }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_accepted_by(&self) -> Option<&String> {
        self.accepted_by.as_ref()
    }

    pub fn get_ack_for(&self) -> Option<&String> {
        self.ack_for.as_ref()
    }

    pub fn set_accepted_by(&mut self, accepted_by: String) {
        self.accepted_by = Some(accepted_by);
    }

    pub fn set_ack_for(&mut self, ack_for: String) {
        self.ack_for = Some(ack_for);
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, bitcode::Error> {
        bitcode::decode(&bytes)
    }

    pub fn to_bytes(&self) -> Bytes {
        Bytes::from(bitcode::encode(self))
    }
}