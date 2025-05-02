use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::task;
use std::time::Duration;
use fake::{faker::lorem::de_de::Word, Fake};

mod charger;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("turbine", "mosquitto_broker", 1884);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).await.unwrap();
    let charger_name: String = Word().fake();

    println!("Connecting as {}", charger_name);

    task::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            println!("Received = {:?}", notification);
        }
    });    
}