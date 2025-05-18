use std::collections::HashMap;

use crate::turbine::Turbine;

use super::{TemperatureData, WindData};

pub struct Cache {
    temperature: HashMap<usize, Vec<TemperatureData>>,
    wind: HashMap<usize, Vec<WindData>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            temperature: HashMap::new(),
            wind: HashMap::new(),
        }
    }

    pub fn get_temperature(&self, id: usize) -> Option<&Vec<TemperatureData>> {
        self.temperature.get(&id)
    }

    pub fn get_wind(&self, id: usize) -> Option<&Vec<WindData>> {
        self.wind.get(&id)
    }

    pub fn set_temperature(&mut self, id: usize, data: Vec<TemperatureData>) {
        self.temperature.insert(id, data);
    }

    pub fn set_wind(&mut self, id: usize, data: Vec<WindData>) {
        self.wind.insert(id, data);
    }
}

impl Turbine {
    pub async fn set_wind_date_to_cache(&mut self, id: usize) {
        if self.cache.get_wind(id).is_none() {
            let data = WindData::for_id(id).await;
            self.cache.set_wind(id, data);
        }
    }

    pub async fn get_wind_date_from_cache(&self, id: usize) -> Option<&Vec<WindData>> {
        self.cache.get_wind(id)
    }

    pub async fn set_temperature_date_to_cache(&mut self, id: usize) {
        if self.cache.get_temperature(id).is_none() {
            let data = TemperatureData::for_id(id).await;
            self.cache.set_temperature(id, data);
        }
    }

    pub async fn get_temperature_date_from_cache(&self, id: usize) -> Option<&Vec<TemperatureData>> {
        self.cache.get_temperature(id)
    }
}