use crate::turbine::Turbine;

const CACHED_ENTRIES: usize = 70000;

mod dump;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrecalculatedTurbine {
    name: String,
    latitude: f64,  // in degrees
    longitude: f64, // in degrees
    cached_power_output: Vec<f64>,
    ticker: usize,
}

impl PrecalculatedTurbine {
    /// Consumes a constructed turbine and returns a PrecalculatedTurbine
    /// This is a quite heavy operation, however, this is done only once
    /// and then drastically improves the performance of the turbine
    pub async fn from_turbine(mut turbine: Turbine) -> Self {
        let name = format!(
            "Turbine_{}_{}",
            turbine.get_latitude(),
            turbine.get_longitude()
        );
        let latitude = turbine.get_latitude();
        let longitude = turbine.get_longitude();
        let mut cached_power_output = vec![0.0; CACHED_ENTRIES];

        for i in 0..CACHED_ENTRIES {
            turbine.tick();
            turbine.approximate_wind_data().await;
            turbine.approximate_temperature_data().await;
            cached_power_output[i] = turbine.get_power_output();
        }

        PrecalculatedTurbine {
            name,
            latitude,
            longitude,
            cached_power_output,
            ticker: turbine.get_tick(),
        }
    }

    pub fn get_power_output(&self) -> f64 {
        self.cached_power_output[self.ticker % CACHED_ENTRIES]
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_tick(&self) -> usize {
        self.ticker
    }

    pub fn tick(&mut self) {
        self.ticker += 1;
    }
}
