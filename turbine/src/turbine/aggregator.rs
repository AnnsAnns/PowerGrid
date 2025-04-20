use super::Turbine;

impl Turbine {
    pub fn get_closest_wind_stations(&mut self) {
        let approx = self.wind_speed_metadata.approximate_location(self.get_latitude(), self.get_longitude());

        for station in approx {
            println!("Station: {}", station.station.to_string());
            println!("Ratio: {}", station.ratio);
            println!("---------------------");
        }
    }
}