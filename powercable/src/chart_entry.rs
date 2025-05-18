use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ChartEntry {
    pub topic: String,
    pub payload: isize,
    pub timestamp: usize,
}

impl ChartEntry {
    pub fn new_no_topic(payload: isize, timestamp: usize) -> Self {
        ChartEntry { topic: String::new(), payload, timestamp }
    }

    pub fn new_no_timestamp(topic: String, payload: isize) -> Self {
        ChartEntry { topic, payload, timestamp: 0 }
    }

    pub fn new(topic: String, payload: isize, timestamp: usize) -> Self {
        ChartEntry { topic, payload, timestamp }
    }

    pub fn from_bytes(bytes: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}