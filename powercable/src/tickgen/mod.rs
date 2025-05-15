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
    pub timestamp: String,
    pub configuration: TickConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TickConfig {
    /// The wait inbetween ticks in seconds
    pub speed: f64,
    pub start_date: String,
}
