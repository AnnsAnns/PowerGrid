use bytes::Bytes;
use tracing::debug;

use crate::SharedTurbine;

/// # Description
/// The `show_handler` function processes incoming visibility configuration messages for the turbine.<br>
/// It updates the turbine's visibility based on the received payload.<br>
/// It is called when a message is received on the `CONFIG_TURBINE` topic.<br>
/// 
/// # Arguments
/// - `handler`: A shared reference to the turbine handler, which contains the turbine instance.
/// - `payload`: The incoming payload containing the visibility configuration in JSON format.
pub async fn show_handler(handler: SharedTurbine, payload: Bytes) {
    let mut handler = handler.lock().await;
    let value = serde_json::from_slice(&payload).unwrap();
    handler.turbine.visible = value;
    debug!("{} visibility set to: {}", handler.name, value);
}