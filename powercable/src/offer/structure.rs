use bytes::{Buf, Bytes};

pub const OFFER_PACKAGE_SIZE: f64 = 1.0;
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Offer {
    id: String,
    price: f64,
    amount: f64,
    accepted_by: Option<String>,
    ack_for: Option<String>,
}

impl Offer {
    pub fn new(id: String, price: f64, amount: f64) -> Self {
        Offer { id, price, amount, accepted_by: None, ack_for: None }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_price(&self) -> f64 {
        self.price
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

    pub fn from_bytes(bytes: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }

    pub fn to_bytes(&self) -> Result<Bytes, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(Bytes::from(json))
    }
}