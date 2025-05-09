use bytes::{Buf, Bytes};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Offer {
    id: String,
    price: f64,
    amount: f64,
}

impl Offer {
    pub fn new(id: String, price: f64, amount: f64) -> Self {
        Offer { id, price, amount }
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

    pub fn from_bytes(bytes: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }

    pub fn to_bytes(&self) -> Result<Bytes, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        Ok(Bytes::from(json))
    }
}