use bytes::Bytes;

pub const INTERVAL_15_MINS: usize = 900; // 15 minutes in seconds

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Phase {
    Process,
    Commerce,
    PowerImport,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TickPayload {
    pub tick: u64,
    pub phase: Phase,
    pub timestamp: usize,
    pub configuration: TickConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TickConfig {
    /// The wait inbetween ticks in seconds
    pub speed: f64,
    pub start_date: String,
}

impl TickPayload {
    pub fn from_bytes(bytes: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&bytes)
    }
}