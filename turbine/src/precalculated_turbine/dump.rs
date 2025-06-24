use super::PrecalculatedTurbine;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct DumpTurbine {
    name: String,
    latitude: f64,  // in degrees
    longitude: f64, // in degrees
    cached_power_output: Vec<f64>, // Dynamic size so we can properly dump it (easily)
    ticker: usize,
}

impl DumpTurbine {
    pub fn dump_from_turbine(turbine: &PrecalculatedTurbine, path: &str) {
        let turbine = DumpTurbine {
            name: turbine.name.clone(),
            latitude: turbine.latitude,
            longitude: turbine.longitude,
            cached_power_output: turbine.cached_power_output.to_vec(),
            ticker: turbine.ticker,
        };

        // Serialize the turbine to a JSON string
        let json_data = serde_json::to_string(&turbine).expect("Failed to serialize turbine");

        // Write the JSON string to the specified file
        std::fs::write(path, json_data).expect("Failed to write turbine data to file");   
    }

    pub fn read_from_file(path: &str) -> PrecalculatedTurbine {
        // Read the JSON string from the specified file
        let json_data = std::fs::read_to_string(path).expect("Failed to read turbine data from file");

        // Deserialize the JSON string back into a DumpTurbine struct
        let dump_turbine: DumpTurbine = serde_json::from_str(&json_data).expect("Failed to deserialize turbine");

        // Convert DumpTurbine back to PrecalculatedTurbine
        PrecalculatedTurbine {
            name: dump_turbine.name,
            latitude: dump_turbine.latitude,
            longitude: dump_turbine.longitude,
            cached_power_output: dump_turbine.cached_power_output.try_into().expect("Cached power output size mismatch"),
            ticker: dump_turbine.ticker,
        }
    }
}