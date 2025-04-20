use super::MetaDataElement;

const WIND_METADATA_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/wind/now/zehn_now_ff_Beschreibung_Stationen.txt";
const AIR_TEMP_METADATA_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/air_temperature/now/zehn_now_tu_Beschreibung_Stationen.txt";
const REQUEST_URL_TEMP: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/air_temperature/now/10minutenwerte_TU_";
const REQUEST_URL_WIND: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/wind/now/10minutenwerte_wind_";

#[derive(Debug, Clone)]
pub enum MetaDataType {
    Wind,
    AirTemperature,
}

impl MetaDataType {
    pub fn to_url(&self) -> &str {
        match self {
            MetaDataType::Wind => WIND_METADATA_URL,
            MetaDataType::AirTemperature => AIR_TEMP_METADATA_URL,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MetaDataType::Wind => "Wind".to_string(),
            MetaDataType::AirTemperature => "AirTemperature".to_string(),
        }
    }

    pub fn to_access_url(&self) -> String {
        match self {
            MetaDataType::Wind => REQUEST_URL_WIND.to_string(),
            MetaDataType::AirTemperature => REQUEST_URL_TEMP.to_string(),
        }
    }
}