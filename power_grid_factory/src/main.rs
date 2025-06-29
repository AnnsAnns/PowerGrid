use consumer::consumer::ConsumerType;
use tokio::task::{self, JoinHandle};
use tracing_subscriber::{field::debug, fmt::writer::MakeWriterExt};

mod shutdown;
mod spawn_tasks;
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
    let file_appender = tracing_appender::rolling::daily("logs", "power_grid.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        // Set debug level for file output
        .with_max_level(std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "warn".to_string())
            .parse()
            .unwrap_or(tracing::Level::WARN))
        // Add file output in addition to stdout
        .with_writer(std::io::stdout.and(non_blocking))
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::debug!("PowerGrid starting up...");

    let mut power_grid = PowerGrid::spawn_new(5, 3, 2).await;

    tracing::debug!("PowerGrid spawned with {} turbines, {} chargers, and {} consumers.", 
        power_grid.turbine.len(), 
        power_grid.charger.len(), 
        power_grid.consumer.len());

    loop {
        // Sleep for a while before the next check
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
