use crate::meta_data::{MetaDataElement, MetaDataWrapper};

pub struct Turbine {
    rotor_dimension: f64, // in meters
    latitude: f64, // in degrees
    longitude: f64, // in degrees
    temperature_metadata: MetaDataWrapper,
    wind_speed_metadata: MetaDataWrapper,
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
        }
    }

    pub fn get_rotor_dimension(&self) -> f64 {
        self.rotor_dimension
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
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