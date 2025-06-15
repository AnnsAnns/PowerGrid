use bytes::Bytes;

pub const TICK_AS_MIN: usize = 15;// our tick in minutes

// do not change these values
pub const TICK_AS_SEC: usize = TICK_AS_MIN * 60;// our tick in seconds
pub const TICK_AS_HOUR: f64 = TICK_AS_MIN as f64 / 60.0;// our tick in hours
pub const PHASE_AS_MIN: usize = TICK_AS_MIN / 3;// our phase in minutes
pub const PHASE_AS_SEC: usize = PHASE_AS_MIN * 60;// our phase in seconds
pub const PHASE_AS_HOUR: f64 = PHASE_AS_MIN as f64 / 60.0;// our phase in hours

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