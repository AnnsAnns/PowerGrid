use crate::parsing::WindData;

use super::Turbine;

impl Turbine {
    pub async fn get_closest_wind_stations(&mut self) {
        println!("Fetching closest wind stations...");

        let approx = self.wind_speed_metadata.approximate_location(self.get_latitude(), self.get_longitude());

        self.closest_wind_stations = Some(approx.clone());
        
        for station in approx {
            println!("Station: {}", station.station.to_string());
            println!("Ratio: {}", station.ratio);
            let wind_data = WindData::for_id(station.station.stations_id).await;
            println!("Wind Data: {:?}", wind_data);
            println!("---------------------");
        }
    }
}