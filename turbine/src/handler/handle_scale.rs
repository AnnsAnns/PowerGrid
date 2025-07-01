use bytes::Bytes;
use tracing::debug;

use crate::SharedTurbine;

/// # Description
/// The `scale_handler` function processes incoming scale configuration messages for the turbine.<br>
/// It updates the turbine's production scale based on the received payload.<br>
/// It is called when a message is received on the `CONFIG_TURBINE_SCALE` topic.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the turbine handler, which contains the turbine instance.
/// - `payload`: The incoming payload containing the scale configuration in JSON format.
pub async fn scale_handler(handler: SharedTurbine, payload: Bytes) {
    let scale = serde_json::from_slice(&payload).unwrap();
    let mut handler = handler.lock().await;
    handler.turbine.set_scale(scale);
    debug!("{} received scale: {:?}", handler.name, payload);
}