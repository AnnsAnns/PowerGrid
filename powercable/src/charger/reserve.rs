
/// Request URI: `NAME/charger/reserve`
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReserveRequest {
    pub own_id: String,
}