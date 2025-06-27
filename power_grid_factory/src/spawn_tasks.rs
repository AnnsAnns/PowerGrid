use consumer::consumer::ConsumerType;
use tokio::task::{self, JoinHandle};

use crate::PowerGrid;

impl PowerGrid {
    /// Restarts the PowerGrid by shutting down all tasks and spawning new ones.
    pub async fn restart(
        &mut self,
        amount_of_chargers: usize,
        amount_of_turbines: usize,
        amount_of_cars: usize,
    ) {
        // Shutdown all tasks gracefully
        self.shutdown().await;

        // Spawn new tasks overwriting itself
        *self = PowerGrid::spawn_new(amount_of_chargers, amount_of_turbines, amount_of_cars).await;
    }

    /// Spawns a new PowerGrid with the specified number of chargers, turbines, and cars.
    pub async fn spawn_new(
        amount_of_chargers: usize,
        amount_of_turbines: usize,
        amount_of_cars: usize,
    ) -> PowerGrid {
        let mut consumers: Vec<(JoinHandle<()>, ConsumerType)> = Vec::new();

        consumers.push((
            task::spawn(consumer::start_consumer(ConsumerType::H)),
            ConsumerType::H,
        ));
        consumers.push((
            task::spawn(consumer::start_consumer(ConsumerType::G)),
            ConsumerType::G,
        ));
        consumers.push((
            task::spawn(consumer::start_consumer(ConsumerType::L)),
            ConsumerType::L,
        ));

        PowerGrid {
            transformer: task::spawn(transformer::start_transformer()),
            tickgen: task::spawn(tickgen::start_tickgen()),
            turbine: (0..amount_of_turbines)
                .map(|_| task::spawn(turbine::start_turbine()))
                .collect(),
            charger: (0..amount_of_chargers)
                .map(|_| task::spawn(charger::start_charger()))
                .collect(),
            fusion_charger: task::spawn(fusion_reactor::start_fusion_gen()),
            consumer: consumers,
            vehicle: (0..amount_of_cars)
                .map(|_| task::spawn(vehicle::start_vehicle()))
                .collect(),
        }
    }
}