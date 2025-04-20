use super::MetaDataWrapper;

pub struct ApproximationElement {
    pub station_id: String,
    pub ratio: f64,
}

impl MetaDataWrapper {
    /// This function tries to trianagulate the location of a turbine
    /// based on the metadata provided. That means, that it will
    /// give us three points, which are cloest to the turbine and
    /// the ratio of the distance to the turbine and thus
    /// the importance of the point.
    ///
    /// For example, if the turbine is exactly on the spot of a point,
    /// the ratio will be 1.0 and the other two points will have a ratio
    /// of 0.0. If the turbine is in the middle of the three points,
    /// the ratio will be 0.33 for each point.
    /// ... hopefully :)
    pub fn approximate_location(
        &mut self,
        latitude: f64,
        longitude: f64,
    ) -> Vec<ApproximationElement> {
        let closest_stations = match self.get_nearest_n_stations(3, latitude, longitude) {
            Some(stations) => stations,
            None => {
                panic!("Not enough stations found for approximation");
            }
        };

        // Calculate the distances to the turbine
        let distances: Vec<f64> = closest_stations
            .iter()
            .map(|station| {
                station.calculate_distance(latitude, longitude)
            })
            .collect();
        
        // Calculate the total distance
        let total_distance: f64 = distances.iter().sum();

        // Calculate the ratios
        let ratios: Vec<f64> = distances
            .iter()
            .map(|&distance| 1.0 - (distance / total_distance))
            .collect();

        // Create the approximation elements
        closest_stations
            .iter()
            .enumerate()
            .map(|(i, station)| ApproximationElement {
                station_id: station.stations_id.clone(),
                ratio: ratios[i],
            })
            .collect()
        
    }
}