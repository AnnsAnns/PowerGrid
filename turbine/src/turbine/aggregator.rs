use log::{debug, info, warn};

use crate::{
    meta_data::MetaDataType,
    parsing::{TemperatureData, WindData},
};

use super::Turbine;

const WIND_STRENGTH_FALLBACK: f64 = 6.0;
const WIND_DIRECTION_FALLBACK: f64 = 200.0;
const AIR_TEMPERATURE_FALLBACK: f64 = 20.0;

impl Turbine {
    pub async fn get_closest_wind_stations(&mut self) {
        debug!("Fetching closest wind stations...");

        let approx = self
            .wind_speed_metadata
            .approximate_location(self.get_latitude(), self.get_longitude());

        self.closest_wind_stations = Some(approx.clone());

        for station in approx {
            debug!("Station: {}", station.station.to_string());
            debug!("Ratio: {}", station.ratio);
            debug!("---------------------");
        }
    }

    pub async fn get_closest_temperature_stations(&mut self) {
        debug!("Fetching closest temperature stations...");

        let approx = self
            .temperature_metadata
            .approximate_location(self.get_latitude(), self.get_longitude());

        self.closest_temperature_stations = Some(approx.clone());

        for station in approx {
            debug!("Station: {}", station.station.to_string());
            debug!("Ratio: {}", station.ratio);
            debug!("---------------------");
        }
    }

    pub async fn approximate_wind_data(&mut self) {
        if self.closest_wind_stations.is_none() {
            warn!("No closest wind stations found. Fetching first...");
            self.get_closest_wind_stations().await;
        }

        let mut data = WindData {
            stations_id: 0,
            date: String::new(),
            quality_level: 0,
            wind_strength: 0.0,
            wind_direction: 0.0,
            eor: String::new(),
        };

        for station in self.closest_wind_stations.as_ref().unwrap() {
            let wind_data = WindData::for_id(station.station.stations_id).await;

            let tick = self.get_tick() % wind_data.len();

            let current_data = wind_data.get(tick).unwrap();

            if current_data.wind_strength == -999.0 {
                warn!(
                    "Wind strength is -999.0 for station {}. Using fallback...",
                    station.station.stationsname
                );
                data.wind_strength += WIND_STRENGTH_FALLBACK * station.ratio;
            } else {
                data.wind_strength += current_data.wind_strength * station.ratio;
            }
            data.wind_direction += current_data.wind_direction * station.ratio;
            if current_data.wind_direction == -999.0 {
                warn!(
                    "Wind strength is -999.0 for station {}. Using fallback...",
                    station.station.stationsname
                );
                data.wind_direction += WIND_DIRECTION_FALLBACK * station.ratio;
            } else {
                data.wind_direction += current_data.wind_direction * station.ratio;
            }

            debug!(
                "Station: {} at ratio {} has strength {} and direction {}",
                station.station.stationsname,
                station.ratio,
                current_data.wind_strength,
                current_data.wind_direction
            );
        }

        debug!(
            "Approximate wind data for station - Strength: {} Direction: {}",
            data.wind_strength, data.wind_direction
        );
        self.approximate_wind = Some(data.clone());
    }

    pub async fn approximate_temperature_data(&mut self) {
        if self.closest_temperature_stations.is_none() {
            info!("No closest temperature stations found. Fetching first...");
            self.get_closest_temperature_stations().await;
        }

        let mut data = TemperatureData {
            stations_id: 0,
            date: String::new(),
            quality_level: 0,
            air_pressure: 0.0,
            air_temperature: 0.0,
            air_temperature_ground: 0.0,
            dew_point_temperature: 0.0,
            relative_humidity: 0.0,
            eor: String::new(),
        };

        for station in self.closest_temperature_stations.as_ref().unwrap() {
            let temperature_data = TemperatureData::for_id(station.station.stations_id).await;


            let tick = self.get_tick() % temperature_data.len();

            let first_data = temperature_data.get(tick).unwrap();
            data.air_temperature += first_data.air_temperature * station.ratio;
            data.air_pressure += first_data.air_pressure * station.ratio;
            data.relative_humidity += first_data.relative_humidity * station.ratio;
            data.dew_point_temperature += first_data.dew_point_temperature * station.ratio;
            data.air_temperature_ground += first_data.air_temperature_ground * station.ratio;

            debug!(
                "Station: {} at ratio {} has temperature {} and pressure {}",
                station.station.stationsname,
                station.ratio,
                first_data.air_temperature,
                first_data.air_pressure
            );
        }

        debug!(
            "Approximate temperature data for station - Temperature: {} Pressure: {}",
            data.air_temperature, data.air_pressure
        );
        self.approximate_temperature = Some(data.clone());
    }
}
