use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::{result, time::Duration};

mod charger;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("turbine", "mosquitto_broker", 1884);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();
    

    task::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            println!("Received = {:?}", notification);
        }
    });    
}