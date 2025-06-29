use super::PrecalculatedTurbine;

impl PrecalculatedTurbine {
    pub fn dump_from_turbine(turbine: &PrecalculatedTurbine, path: &str) {
        // Serialize the turbine to a JSON string
        let json_data = serde_json::to_string(&turbine).expect("Failed to serialize turbine");

        // Check whether data directory exists, if not create it
        if !std::path::Path::new("data").exists() {
            std::fs::create_dir("data").expect("Failed to create data directory");
        }

        // Write the JSON string to the specified file
        std::fs::write(path, json_data).expect("Failed to write turbine data to file");   
    }

    pub fn read_from_file(path: &str) -> PrecalculatedTurbine {
        // Read the JSON string from the specified file
        let json_data = std::fs::read_to_string(path).expect("Failed to read turbine data from file");

        // Deserialize the JSON string back into a DumpTurbine struct
        let dump_turbine: PrecalculatedTurbine = serde_json::from_str(&json_data).expect("Failed to deserialize turbine");

        dump_turbine
    }
}