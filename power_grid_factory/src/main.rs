use consumer::consumer::ConsumerType;
use tokio::task::{self, JoinHandle};

mod spawn_tasks;
mod shutdown;
mod watchdog;

struct PowerGrid {
    transformer: JoinHandle<()>,
    tickgen: JoinHandle<()>,
    turbine: Vec<JoinHandle<()>>,
    charger: Vec<JoinHandle<()>>,
    fusion_charger: JoinHandle<()>,
    consumer: Vec<(JoinHandle<()>, ConsumerType)>,
    vehicle: Vec<JoinHandle<()>>,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut power_grid = PowerGrid::spawn_new(5, 3, 10).await;

    loop {
        // Sleep for a while before the next check
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
