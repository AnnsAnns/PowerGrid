use std::time::Duration;

use tracing::debug;
use powercable::{MAP_UPDATE_SPEED_IN_SECS, POWER_LOCATION_TOPIC};
use serde_json::json;
use tokio::time::sleep;

use crate::SharedConsumer;

pub async fn map_update_task(handler: SharedConsumer) {
    loop {
        {
            let handler = handler.lock().await;
            let location_payload = json!({
                "name" : handler.consumer.get_consumer_type().to_string(),
                "lat": handler.consumer.get_latitude(),
                "lon": handler.consumer.get_longitude(),
                "icon": handler.consumer.get_consumer_type().to_icon(),
                "label": format!("{:.1}kW", handler.consumer.get_current_consumption()),
            })
            .to_string();
            handler
                .client
                .publish(
                    POWER_LOCATION_TOPIC,
                    rumqttc::QoS::ExactlyOnce,
                    true,
                    location_payload.clone(),
                )
                .await
                .unwrap();
            debug!("Published location: {:?}", location_payload);
        }

        sleep(Duration::from_secs(MAP_UPDATE_SPEED_IN_SECS)).await;
    }
}
