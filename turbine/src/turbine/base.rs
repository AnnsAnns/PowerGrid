#![allow(unused)]

use crate::{meta_data::{ApproximationElement, MetaDataWrapper}, parsing::{Cache, TemperatureData, WindData}};

pub struct Turbine {
    rotor_dimension: f64, // in meters
    latitude: f64, // in degrees
    longitude: f64, // in degrees
    ticker: usize, 
    pub temperature_metadata: MetaDataWrapper,
    pub wind_speed_metadata: MetaDataWrapper,
    pub closest_wind_stations: Option<Vec<ApproximationElement>>,
    pub closest_temperature_stations: Option<Vec<ApproximationElement>>,
    pub approximate_wind: Option<WindData>,
    pub approximate_temperature: Option<TemperatureData>,
    pub cache: Cache,
}

impl Turbine {
    pub fn new(
        rotor_dimension: f64,
        latitude: f64,
        longitude: f64,
        temperature_metadata: MetaDataWrapper,
        wind_speed_metadata: MetaDataWrapper,
    ) -> Self {
        Turbine {
            rotor_dimension,
            latitude,
            longitude,
            temperature_metadata,
            wind_speed_metadata,
            ticker: 0,
            closest_wind_stations: None,
            closest_temperature_stations: None,
            approximate_wind: None,
            approximate_temperature: None,
            cache: Cache::new(),
        }
    }

    pub fn get_rotor_dimension(&self) -> f64 {
        self.rotor_dimension
    }

    pub fn get_rotor_area(&self) -> f64 {
        std::f64::consts::PI * (self.rotor_dimension / 2.0).powi(2)
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

    pub fn set_temperature_metadata(&mut self, metadata: MetaDataWrapper) {
        self.temperature_metadata = metadata;
    }

    pub fn set_wind_speed_metadata(&mut self, metadata: MetaDataWrapper) {
        self.wind_speed_metadata = metadata;
    }

    pub fn get_temperature_metadata(&self) -> &MetaDataWrapper {
        &self.temperature_metadata
    }

    pub fn get_wind_speed_metadata(&self) -> &MetaDataWrapper {
        &self.wind_speed_metadata
    }
}