use crate::PowerGrid;

impl PowerGrid {
    pub async fn shutdown(&mut self) {
        // Shutdown all tasks gracefully
        for turbine in &self.turbine {
            let _ = turbine.abort();
        }
        for charger in &self.charger {
            let _ = charger.abort();
        }
        for consumer in &self.consumer {
            let _ = consumer.0.abort();
        }
        for vehicle in &self.vehicle {
            let _ = vehicle.abort();
        }
        let _ = self.transformer.abort();
        let _ = self.tickgen.abort();
        let _ = self.fusion_charger.abort();

        tracing::info!("PowerGrid has been shut down gracefully.");
    }
}

impl Drop for PowerGrid {
    fn drop(&mut self) {
        // Ensure that the shutdown method is called when the PowerGrid instance is dropped
        let _ = self.shutdown();
        tracing::info!("PowerGrid instance is being dropped.");   
    }
}