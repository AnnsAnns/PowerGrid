use crate::PowerGrid;

impl PowerGrid {
    /// Checks whether all the tasks are running and restarts them if they are not.
    /// This method is intended to be run periodically to ensure the health of the system.
    /// For example, if the turbine crashes, it should be restarted.
    pub async fn check_health_and_restart(&mut self) {
        // Check if the transformer task is still running
        if self.transformer.is_finished() {
            tracing::warn!("Transformer task has stopped. Restarting...");
            self.transformer = tokio::task::spawn(transformer::start_transformer());
        }

        // Check if the tickgen task is still running
        if self.tickgen.is_finished() {
            tracing::warn!("Tickgen task has stopped. Restarting...");
            self.tickgen = tokio::task::spawn(tickgen::start_tickgen());
        }

        // Check each turbine task
        for (i, turbine) in self.turbine.iter_mut().enumerate() {
            if turbine.is_finished() {
                tracing::warn!("Turbine {} task has stopped. Restarting...", i);
                *turbine = tokio::task::spawn(turbine::start_turbine(i));
            }
        }

        // Check each charger task
        for (i, charger) in self.charger.iter_mut().enumerate() {
            if charger.is_finished() {
                tracing::warn!("Charger {} task has stopped. Restarting...", i);
                *charger = tokio::task::spawn(charger::start_charger(i as u64));
            }
        }

        // Check the fusion charger task
        if self.fusion_charger.is_finished() {
            tracing::warn!("Fusion charger task has stopped. Restarting...");
            self.fusion_charger = tokio::task::spawn(fusion_reactor::start_fusion_gen());
        }

        // Check each consumer task
        for (i, (consumer_task, consumer_type)) in self.consumer.iter_mut().enumerate() {
            if consumer_task.is_finished() {
                tracing::warn!("Consumer {} of type {:?} has stopped. Restarting...", i, consumer_type);
                *consumer_task = tokio::task::spawn(consumer::start_consumer(*consumer_type, i as u64));
            }
        }

        // Check each vehicle task
        for (i, vehicle) in self.vehicle.iter_mut().enumerate() {
            if vehicle.is_finished() {
                tracing::warn!("Vehicle {} task has stopped. Restarting...", i);
                *vehicle = tokio::task::spawn(vehicle::start_vehicle(i as u64));
            }
        }
    }
}