use crate::meta_data::MetaDataType;

use super::{download_data_for, read_for};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TemperatureData {
    #[serde(rename = "STATIONS_ID")]
    pub stations_id: usize,
    #[serde(rename = "MESS_DATUM")]
    pub date: String,
    #[serde(rename = "QN")]
    pub quality_level: usize,
    #[serde(rename = "PP_10")]
    pub air_pressure: f64,
    #[serde(rename = "TT_10")]
    pub air_temperature: f64,
    #[serde(rename = "TM5_10")]
    pub air_temperature_ground: f64,
    #[serde(rename = "RF_10")]
    pub relative_humidity: f64,
    #[serde(rename = "TD_10")]
    pub dew_point_temperature: f64,
    #[serde(rename = "eor")]
    pub eor: String,
}


impl TemperatureData {
    pub async fn for_id(stations_id: usize) -> Vec<Self> {
        let data_type = MetaDataType::AirTemperature;
        download_data_for(stations_id, data_type.clone()).await.unwrap();
        let mut reader = read_for(stations_id, data_type).unwrap();
        let mut records = Vec::new();
        for result in reader.deserialize() {
            let record: TemperatureData = result.unwrap();
            records.push(record);
        }
        records
    }
}